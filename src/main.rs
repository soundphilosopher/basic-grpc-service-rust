use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};

use basic_grpc_service_rust::{
    FILE_DESCRIPTOR_SET, info,
    sdk::{
        basic::{
            service::v1::{
                BackgroundRequest, BackgroundResponse, BackgroundResponseEvent, HelloRequest,
                HelloResponse, HelloResponseEvent, SomeServiceData, SomeServiceResponse, State,
                TalkRequest, TalkResponse,
            },
            v1::basic_service_server::{BasicService, BasicServiceServer},
        },
        io::cloudevents::v1::{CloudEvent, cloud_event::Data::ProtoData},
    },
    success, talk, utils,
};
use futures_core::Stream;
use prost_types::{Any, Timestamp};
use rand::{Rng, rng};
use rustls::crypto::{CryptoProvider, ring};
use tokio::{fs, signal, sync::mpsc, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    Status,
    transport::{Identity, Server, ServerTlsConfig},
};
use tonic_reflection::server::Builder as ReflectionBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
struct BasicServiceV1;

#[tonic::async_trait]
impl BasicService for BasicServiceV1 {
    type TalkStream =
        Pin<Box<dyn Stream<Item = Result<TalkResponse, tonic::Status>> + Send + Sync + 'static>>;
    type BackgroundStream = Pin<
        Box<dyn Stream<Item = Result<BackgroundResponse, tonic::Status>> + Send + Sync + 'static>,
    >;

    async fn hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloResponse>, tonic::Status> {
        let event = HelloResponseEvent {
            greeting: format!("Hello, {}!", request.into_inner().message),
        };

        let any = Any {
            type_url: "basic.service.v1.HelloResponseEvent".to_string(),
            value: prost::Message::encode_to_vec(&event),
        };

        let cloudevent = CloudEvent {
            id: Uuid::new_v4().to_string(),
            source: "/basic/hello".to_string(),
            spec_version: "1.0".to_string(),
            r#type: "io.basic.hello".to_string(),
            attributes: std::collections::HashMap::new(),
            data: Some(ProtoData(any)),
        };

        let response = HelloResponse {
            cloud_event: Some(cloudevent),
        };

        Ok(tonic::Response::new(response))
    }

    async fn talk(
        &self,
        request: tonic::Request<tonic::Streaming<TalkRequest>>,
    ) -> Result<tonic::Response<Self::TalkStream>, tonic::Status> {
        let mut inbound = request.into_inner();
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            while let Some(req) = inbound.message().await.transpose() {
                match req {
                    Ok(talk_req) => {
                        let (answer, _) = talk::reply(&talk_req.message);
                        let response = TalkResponse { answer: answer };
                        if tx.send(Ok(response)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx
                            .send(Err(Status::internal(format!(
                                "Failed to receive message: {}",
                                e
                            ))))
                            .await;
                        break;
                    }
                }
            }
        });

        Ok(tonic::Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn background(
        &self,
        request: tonic::Request<BackgroundRequest>,
    ) -> Result<tonic::Response<Self::BackgroundStream>, tonic::Status> {
        let processes = request.into_inner().processes.max(0) as usize;

        // Stream to the client
        let (tx_out, rx_out) =
            mpsc::channel::<Result<BackgroundResponse, tonic::Status>>(processes);

        // Internal channel for worker results
        let (tx_res, mut rx_res) = mpsc::channel::<SomeServiceResponse>(processes);

        tokio::spawn(async move {
            // 1) spawn workers
            for i in 1..=processes {
                let tx_res = tx_res.clone();
                tokio::spawn(async move {
                    // pretend we’re “calling a process”
                    let delay = rng().random_range(1..=3);
                    sleep(Duration::from_secs(delay)).await;

                    let some_response = SomeServiceResponse {
                        id: Uuid::new_v4().to_string(),
                        name: format!("service-{}", i),
                        version: "1.1.2".to_string(),
                        data: Some(SomeServiceData {
                            r#type: "protocol".to_string(),
                            value: utils::random_protocol(),
                        }),
                    };

                    // ignore send error if coordinator is gone
                    let _ = tx_res.send(some_response).await;
                });
            }
            drop(tx_res); // Important: close so rx_res ends when all workers finish

            // 2) coordinator aggregates and streams updates
            let mut event = BackgroundResponseEvent {
                started_at: Some(Timestamp::from(SystemTime::now())),
                state: State::Process as i32,
                completed_at: None,
                responses: Vec::with_capacity(processes),
            };

            // Send an initial "started" event if you want (optional)
            if let Err(_) = tx_out
                .send(Ok(utils::create_background_response(&event)))
                .await
            {
                return; // client disconnected
            }

            // As each worker finishes, push and stream a snapshot
            while let Some(resp) = rx_res.recv().await {
                event.responses.push(resp);

                if tx_out
                    .send(Ok(utils::create_background_response(&event)))
                    .await
                    .is_err()
                {
                    return; // client disconnected
                }
            }

            // 3) all done -> mark complete and send final snapshot
            event.state = State::Complete as i32;
            event.completed_at = Some(Timestamp::from(SystemTime::now()));

            let _ = tx_out
                .send(Ok(utils::create_background_response(&event)))
                .await;
        });

        Ok(tonic::Response::new(Box::pin(ReceiverStream::new(rx_out))))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Arc<CryptoProvider> = Arc::new(ring::default_provider());
    CryptoProvider::install_default((*provider).clone())
        .expect("Failed to install default CryptoProvider");

    let addr = "127.0.0.1:50443".parse()?;
    let cert = fs::read("certs/local.crt").await?;
    let key = fs::read("certs/local.key").await?;

    let identity = Identity::from_pem(cert, key);
    let tls_config = ServerTlsConfig::new().identity(identity);

    let reflection_service_v1 = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;
    let reflection_service_v1_alpha = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    info!("Starting gRPC server on {}", addr);
    Server::builder()
        .tls_config(tls_config)?
        .add_service(BasicServiceServer::new(BasicServiceV1::default()))
        .add_service(reflection_service_v1)
        .add_service(reflection_service_v1_alpha)
        .serve_with_shutdown(addr, async {
            signal::ctrl_c().await.expect("Failed to listen to Ctrl+C");
            info!("Shutting down gRPC server...");
        })
        .await?;

    success!("gRPC server stopped.");

    Ok(())
}

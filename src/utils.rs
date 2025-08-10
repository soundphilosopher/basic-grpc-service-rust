use crate::sdk::basic::service::v1::{BackgroundResponse, BackgroundResponseEvent, State};
use crate::sdk::io::cloudevents::v1::{CloudEvent, cloud_event::Data::ProtoData};
use prost::Message;
use prost_types::{Any, Timestamp};
use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct StateManager {
    state: Arc<Mutex<HashMap<String, State>>>,
    start: Arc<Mutex<HashMap<String, Timestamp>>>,
    complete: Arc<Mutex<HashMap<String, Timestamp>>>,
    errors: Arc<Mutex<HashMap<String, Vec<String>>>>, // Replace String with actual error type if needed
}

impl StateManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&self, hash: &str, state: State) {
        let mut state_map = self.state.lock().unwrap();
        let mut start_map = self.start.lock().unwrap();

        state_map.insert(hash.to_string(), state);
        start_map.insert(hash.to_string(), Self::current_timestamp());
    }

    pub fn finish(&self, hash: &str, state: State) {
        let mut state_map = self.state.lock().unwrap();
        let mut complete_map = self.complete.lock().unwrap();

        state_map.insert(hash.to_string(), state);
        complete_map.insert(hash.to_string(), Self::current_timestamp());
    }

    pub fn get_state(
        &self,
        hash: &str,
    ) -> (
        Option<State>,
        Option<Timestamp>,
        Option<Timestamp>,
        Option<Vec<String>>,
    ) {
        let state = self.state.lock().unwrap().get(hash).cloned();
        let start = self.start.lock().unwrap().get(hash).cloned();
        let complete = self.complete.lock().unwrap().get(hash).cloned();
        let errors = self.errors.lock().unwrap().get(hash).cloned();
        (state, start, complete, errors)
    }

    pub fn set_error(&self, hash: &str, err: Option<String>) {
        if let Some(error) = err {
            let mut errors_map = self.errors.lock().unwrap();
            errors_map.entry(hash.to_string()).or_default().push(error);
        }
    }

    pub fn has_errors(&self, hash: &str) -> bool {
        self.errors
            .lock()
            .unwrap()
            .get(hash)
            .map_or(false, |errs| !errs.is_empty())
    }

    pub fn get_errors(&self, hash: &str) -> Vec<String> {
        self.errors
            .lock()
            .unwrap()
            .get(hash)
            .cloned()
            .unwrap_or_default()
    }

    fn current_timestamp() -> Timestamp {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        Timestamp {
            seconds: now.as_secs() as i64,
            nanos: now.subsec_nanos() as i32,
        }
    }
}

pub fn random_protocol() -> String {
    let protocols = vec!["rest", "rpc", "grpc", "ws"];
    let mut rng = rand::rng();
    protocols[rng.random_range(0..protocols.len())].to_string()
}

pub fn create_background_response(event: &BackgroundResponseEvent) -> BackgroundResponse {
    let any = Any {
        type_url: "type.googleapis.com/basic.service.v1.BackgroundResponseEvent".to_string(),
        value: Message::encode_to_vec(event),
    };

    let cloudevent = CloudEvent {
        id: Uuid::new_v4().to_string(),
        source: "basic.v1/Background".to_string(),
        spec_version: "1.0".to_string(),
        // You might want this to be the *Event* type, not *Response*
        r#type: "type.googleapis.com/basic.service.v1.BackgroundResponse".to_string(),
        attributes: Default::default(),
        data: Some(ProtoData(any)),
    };

    BackgroundResponse {
        cloud_event: Some(cloudevent),
    }
}

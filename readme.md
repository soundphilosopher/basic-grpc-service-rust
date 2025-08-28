# 🚀 Basic gRPC Service in Rust

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tonic](https://img.shields.io/badge/Tonic-0.14.0-blue.svg)](https://github.com/hyperium/tonic)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen.svg)](https://github.com/your-username/basic-grpc-service-rust)
[![gRPC](https://img.shields.io/badge/gRPC-Ready-ff69b4.svg)](https://grpc.io/)
[![TLS](https://img.shields.io/badge/TLS-Enabled-success.svg)](https://en.wikipedia.org/wiki/Transport_Layer_Security)

> *"Because who doesn't love a blazingly fast 🦀 Rust service that speaks gRPC fluently?"*

A lightning-fast, production-ready gRPC service built with Rust that showcases the power of async programming, type safety, and the incredible Tonic framework. This service demonstrates three different gRPC communication patterns: unary calls, bidirectional streaming, and server-side streaming with background processing!

## ✨ Features

- 🔥 **Blazingly Fast**: Built with Rust and Tokio for maximum performance
- 🛡️ **Type Safe**: Leverages Prost for bulletproof Protocol Buffer serialization
- 🔐 **TLS Secured**: Local certificate support with mkcert integration
- 🌊 **Streaming Support**: Bidirectional and server-side streaming capabilities
- 📡 **gRPC Reflection**: Built-in reflection support for easy service discovery
- 🎯 **Cloud Events**: CloudEvents integration for event-driven architecture
- 🔄 **Background Processing**: Async background task processing with real-time updates
- 📝 **Auto-Generated Code**: Seamless Protocol Buffer code generation

## 🛠️ Tech Stack

- **Runtime**: [Tokio](https://tokio.rs/) - The asynchronous runtime for Rust
- **gRPC Framework**: [Tonic](https://github.com/hyperium/tonic) - A native gRPC client & server
- **Serialization**: [Prost](https://github.com/tokio-rs/prost) - Protocol Buffer implementation
- **TLS**: [Rustls](https://github.com/rustls/rustls) with Ring crypto provider
- **Code Generation**: [tonic-prost-build](https://docs.rs/tonic-prost-build) - Build-time code generation
- **Certificates**: [mkcert](https://github.com/FiloSottile/mkcert) - Local certificate authority

## 📋 Prerequisites

Before you embark on this gRPC adventure, make sure you have:

- **Rust** (>= 1.70.0) - [Install Rust](https://rustup.rs/)
- **mkcert** - For generating local certificates
- **Protocol Buffers** (optional, for manual proto compilation)

## 🚀 Quick Start

### 1. Clone & Setup
```bash
git clone https://github.com/your-username/basic-grpc-service-rust.git
cd basic-grpc-service-rust
```

### 2. Generate Local Certificates
```bash
# Install mkcert if you haven't already
# macOS: brew install mkcert
# Ubuntu: sudo apt install libnss3-tools && curl -s https://api.github.com/repos/FiloSottile/mkcert/releases/latest | grep browser_download_url | grep linux-amd64 | cut -d '"' -f 4 | wget -qi - && chmod +x mkcert-v*-linux-amd64 && sudo cp mkcert-v*-linux-amd64 /usr/local/bin/mkcert

# Create local CA
mkcert -install

# Generate certificates
mkcert -cert-file certs/local.crt -key-file certs/local.key localhost 127.0.0.1 ::1
```

### 3. Build & Run
```bash
# Build the project (this will auto-generate gRPC code)
cargo build

# Run the server
cargo run
```

🎉 Your gRPC server is now running on `https://127.0.0.1:50443`!

## 🎮 API Playground

### Service Overview

Our `BasicService` provides three exciting endpoints:

#### 1. 👋 Hello (Unary RPC)
A simple greeting service that returns a CloudEvent-wrapped response.

**Proto Definition:**
```protobuf
rpc Hello(HelloRequest) returns (HelloResponse);
```

#### 2. 💬 Talk (Bidirectional Streaming)
Real-time conversation with the service - send messages and get instant responses!

**Proto Definition:**
```protobuf
rpc Talk(stream TalkRequest) returns (stream TalkResponse);
```

#### 3. ⚡ Background (Server Streaming)
Kick off multiple background processes and watch them complete in real-time.

**Proto Definition:**
```protobuf
rpc Background(BackgroundRequest) returns (stream BackgroundResponse);
```

### Testing with grpcurl

```bash
# Test Hello endpoint
grpcurl -d '{"message": "World"}' 127.0.0.1:50443 basic.v1.BasicService/Hello

# Test Talk endpoint
cat <<EOM | grpcurl -d @ 127.0.0.1:50443 basic.v1.BasicService/Talk
{
  "message": "Hello"
}
{
  "message": "How are you?"
}
{
  "message": "Goodbye"
}
EOM

# Test Background processing (start 5 processes)
grpcurl -d '{"processes": 5}' 127.0.0.1:50443 basic.v1.BasicService/Background
```

## 🏗️ Project Structure

```
basic-grpc-service-rust/
├── 📁 proto/                  # Protocol Buffer definitions
│   ├── basic/v1/basic.proto  # Main service definition
│   └── basic/service/v1/     # Message definitions
├── 📁 src/
│   ├── 📁 sdk/               # 🤖 Auto-generated gRPC code
│   ├── 📁 talk/              # Conversation logic
│   ├── main.rs               # 🚀 Server entrypoint
│   ├── lib.rs                # Library exports
│   └── utils.rs              # Utility functions
├── 📁 certs/                 # 🔐 TLS certificates
├── build.rs                  # 🔧 Build-time code generation
└── Cargo.toml                # 📦 Dependencies
```

## 🔧 Build Process

The magic happens in `build.rs`! During compilation:

1. **Proto Compilation**: `tonic-prost-build` reads all `.proto` files
2. **Code Generation**: Generates Rust types and gRPC service traits
3. **Output**: Generated code lands in `src/sdk/` directory
4. **Descriptor**: Creates a binary descriptor for gRPC reflection

```rust
tonic_prost_build::configure()
    .build_server(true)
    .out_dir("src/sdk")
    .file_descriptor_set_path("src/sdk/descriptor.bin")
    .compile_protos(&["proto/basic/v1/basic.proto"], &["proto"])?;
```

## 🛡️ Security Features

- **TLS 1.3**: Modern encryption with Rustls and Ring crypto provider
- **Certificate Validation**: Local certificate authority with mkcert
- **Type Safety**: Compile-time guarantees with Rust's type system
- **Memory Safety**: No buffer overflows or memory leaks

## 🧪 Development

### Adding New Services

1. Define your service in a `.proto` file under `proto/`
2. Update `build.rs` to include your new proto file
3. Implement the generated service trait
4. Register the service in `main.rs`

### Code Generation

The build process automatically generates:
- Message types (structs with serialization)
- Service traits (async functions you implement)
- Client stubs (for testing and client development)
- gRPC reflection metadata

## 📊 Performance

Built for speed with:
- **Zero-copy deserialization** with Prost
- **Async I/O** with Tokio's multi-threaded runtime
- **Connection pooling** and HTTP/2 multiplexing via Tonic
- **Efficient streaming** with backpressure handling

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. 🍴 Fork the repository
2. 🌿 Create your feature branch (`git checkout -b feature/amazing-feature`)
3. 💾 Commit your changes (`git commit -m 'Add amazing feature'`)
4. 📤 Push to the branch (`git push origin feature/amazing-feature`)
5. 🔄 Open a Pull Request

## 📝 ToDo's

- [x] Implement basic gRPC services
- [x] Implement util functions
- [x] Add comprehensive README
- [x] Add LICENSE file
- [ ] Add unit tests
- [ ] Add integration tests
- [ ] Add Docker support
- [ ] Add health check endpoint
- [ ] Add metrics and observability
- [ ] Add client examples in multiple languages

## 📚 Learn More

- [Tonic Documentation](https://docs.rs/tonic/)
- [Prost Documentation](https://docs.rs/prost/)
- [gRPC Documentation](https://grpc.io/docs/)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)
- [Tokio Guide](https://tokio.rs/tokio/tutorial)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Made with ❤️ and 🦀 by developers who believe in the power of type-safe, high-performance systems**

[⭐ Star this repo](https://github.com/your-username/basic-grpc-service-rust) if you found it helpful!

</div>
```

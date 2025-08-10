fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .out_dir("src/sdk")
        .file_descriptor_set_path("src/sdk/descriptor.bin")
        .compile_protos(&["proto/basic/v1/basic.proto"], &["proto"])?;

    Ok(())
}

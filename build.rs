fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("./proto/calculator.proto")?;

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path("calculator_descriptor.bin")
        .compile(&["./proto/calculator.proto"], &["proto"])?;
    
    Ok(())
}

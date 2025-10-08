fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile Flow protobuf definitions
    tonic_build::configure()
        .build_server(false)
        .compile_protos(
            &["proto/flow/access/access.proto"],
            &["proto"],
        )?;
    Ok(())
}

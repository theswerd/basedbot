fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .build_transport(true)
        .compile_well_known_types(true)
        .compile_protos(&["proto/hal_pb.proto"], &["proto"])?;

    println!("cargo:rerun-if-changed=proto/hal_pb.proto");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_transport(true)
        .compile_well_known_types(true)
        .emit_rerun_if_changed(true)
        .compile_protos(&[
            "kos/actuator.proto",
            "kos/common.proto",
            "kos/imu.proto",
            "kos/inference.proto",
            "kos/process_manager.proto",
            "kos/system.proto",
            "google/longrunning/operations.proto",
            ], &["proto", &format!("{}/googleapis", "proto")])?;

    Ok(())
}

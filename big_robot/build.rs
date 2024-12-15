fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .build_transport(true)
        .compile_well_known_types(true)
        .emit_rerun_if_changed(true)
        .compile_protos(&[
            "proto/actuator.proto",
            "proto/common.proto",
            "proto/imu.proto",
            "proto/inference.proto",
            "proto/process_manager.proto",
            "proto/system.proto",
            ], &[
                "actuator_pb",
                "common_pb",
                "imu_pb",
                "inference_pb",
                "process_manager_pb",
                "system_pb",
                ])?;

    Ok(())
}

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the Protobuf files
    let proto_root = "proto";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // List of Protobuf files
    let protos = [
        "kos/common.proto",
        "kos/actuator.proto",
        "kos/imu.proto",
        "kos/inference.proto",
        "kos/process_manager.proto",
        "kos/system.proto",
        "google/longrunning/operations.proto",
    ];

    std::fs::create_dir_all(out_dir.join("kos")).expect("Failed to create output directory");

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_transport(true)
        .compile_well_known_types(true)
        .out_dir(out_dir.join("kos"))
        .emit_rerun_if_changed(true)
        .compile_protos(&protos, &["proto", &format!("{}/googleapis", "proto")])?;

    // Re-run the build script if any of the proto files change
    for proto in &protos {
        println!("cargo:rerun-if-changed={}/kos/{}", proto_root, proto);
    }
    println!("cargo:rerun-if-changed={}", proto_root);

    Ok(())
}

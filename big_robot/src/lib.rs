#![allow(unknown_lints)]
#![allow(clippy::doc_lazy_continuation)]

// pub mod config;
mod grpc_interface;
// pub mod hal;
// pub mod services;
// pub mod telemetry;
// pub mod telemetry_types;

pub use grpc_interface::google as google_proto;
pub use grpc_interface::kos as kos_proto;


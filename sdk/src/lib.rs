pub mod proto {
    tonic::include_proto!("hal_pb");
}

pub use proto::servo_control_client::ServoControlClient as ZerothClient;

use std::net::TcpStream;

use humanoid::Humanoid;
use mini_robot::MiniRobot;
use tonic::{transport::Channel, Request};
// use zeroth::{CalibrationRequest, JointPosition, ServoId, TorqueSetting, WifiCredentials};
use bon::{builder, Builder};
use zeroth::JointPosition;

pub mod humanoid;
pub mod mini_robot;

#[tokio::main]
async fn main() {
    let client = zeroth::Client::connect("grpc://192.168.42.1:50051").await;

    let mut client = match client {
        Ok(client) => client,
        Err(e) => panic!("Failed to connect to the server: {:?}", e),
    };

    client.disable_movement().await.unwrap();

    let mut robot = MiniRobot::new(client);
    robot.calibrate().await.unwrap();

    // robot.se/t
}

// 0 -90 90

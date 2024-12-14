use std::net::TcpStream;

use humanoid::Humanoid;
use mini_robot::MiniRobot;
use tonic::{transport::Channel, Request};
// use zeroth::{CalibrationRequest, JointPosition, ServoId, TorqueSetting, WifiCredentials};
use bon::{builder, Builder};
use zeroth::JointPosition;

pub mod mini_robot;
pub mod humanoid;

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

    robot.set_left_shoulder_yaw(45.0).await.unwrap();
    robot.set_right_eblow_yaw(90.0).await.unwrap();
    robot.set_left_elbow_yaw(90.0).await.unwrap();


    
}

// 0 -90 90

use std::net::TcpStream;

use tonic::{transport::Channel, Request};
// use zeroth::{CalibrationRequest, JointPosition, ServoId, TorqueSetting, WifiCredentials};
use bon::{builder, Builder};
use zeroth::JointPosition;


#[tokio::main]
async fn main() {
    let client = zeroth::Client::connect("grpc://192.168.42.1:50051").await;

    let mut client = match client {
        Ok(client) => client,
        Err(e) => panic!("Failed to connect to the server: {:?}", e),
    };

    client.disable_movement().await.unwrap();

    let mut robot = MiniRobot::new(client);

    robot.set_left_shoulder_yaw(129.0).await.unwrap();
}

pub struct MiniRobot {
    client: zeroth::Client,
    calibration: Option<MiniRobotCalibration>,
}

#[derive(Builder)]
pub struct MiniRobotCalibration {
    pub left_shoulder_yaw_min: f32,
    pub left_shoulder_yaw_max: f32,
}

impl MiniRobot {
    pub fn new(client: zeroth::Client) -> Self {
        return MiniRobot {
            client,
            calibration: None,
        };
    }
}

pub trait Humanoid {
    async fn calibrate(&mut self) -> Result<(), ()>;

    async fn set_left_shoulder_yaw(&mut self, yaw: f32) -> Result<(), ()>;
}

impl Humanoid for MiniRobot {
    async fn calibrate(&mut self) -> Result<(), ()> {
        // let left_shoulder_yaw_info = self
        //     .client
        //     .get_(Request::new(ServoId { id: 15 }))
        //     .await
        //     .unwrap();

        // let yaw = if let zeroth::servo_info_response::Result::Info(yaw) =
        //     left_shoulder_yaw_info.into_inner().result.unwrap()
        // {
        //     yaw
        // } else {
        //     return Err(());
        // };

        Ok(())
        // self.client.start_calibration(
        //     Request::new(
        //         CalibrationRequest {

        //         }
        //     )
        // )
    }

    async fn set_left_shoulder_yaw(&mut self, yaw: f32) -> Result<(), ()> {
        let _ = self
            .client
            .set_position(
                JointPosition {
                    id: 15,
                    position: yaw,
                    speed: 100.0,
                }
            )
            .await;
        Ok(())
    }
}

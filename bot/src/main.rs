use std::{collections::BTreeMap, io::Read, net::TcpStream, time::Duration};

use humanoid::{Humanoid, Joint};
use mini_robot::MiniRobot;
use serde_json::from_str;
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

    std::thread::sleep(Duration::from_secs(1));


    initial_position(&mut robot).await;



    let frames = file_to_frames(
        "/Users/benswerdlow/Documents/GitHub/basedbot/pose_mappings/flapping_motion.json",
    );

    // for frame in frames.iter() {
    //     std::thread::sleep(Duration::from_millis(100));

    //     for (joint, value) in &frame.joints {
    //         robot.set_joint(joint.clone(), value.clone()).await.unwrap();
    //     }
    // }

    println!("FRAMES: {:?}", frames);
    std::thread::sleep(Duration::from_millis(200));
}

#[derive(Debug)]
pub struct Frame {
    pub joints: BTreeMap<humanoid::Joint, f32>,
}

pub fn file_to_frames(file: &str) -> Vec<Frame> {
    let mut string = String::new();
    std::fs::File::open(file)
        .unwrap()
        .read_to_string(&mut string)
        .unwrap();

    let json: serde_json::Value = serde_json::from_str(&string).unwrap();
    let json = json.as_array().unwrap().clone();
    let mut frames: Vec<Frame> = Vec::new();
    for frame in json {
        let frame = frame.as_object().unwrap().clone();
        let mut joints = BTreeMap::new();
        for (joint_id, joint_value) in frame.into_iter() {
            let joint_servo_id = zeroth::ServoId::try_from(
                // joint_id as number
                from_str::<i32>(&joint_id).unwrap(),
            )
            .unwrap();
            let humanoid_id: Joint = Joint::from(joint_servo_id);

            joints.insert(humanoid_id, joint_value.as_f64().unwrap() as f32);
            // frame.insert(joint
        }
        frames.push(Frame { joints });
    }

    frames
}

pub async fn initial_position(robot: &mut impl humanoid::Humanoid) {
    robot
        .set_joint(humanoid::Joint::RightElbowYaw, 0.0)
        .await
        .unwrap();
    robot
        .set_joint(humanoid::Joint::RightShoulderPitch, 0.0)
        .await
        .unwrap();
    robot
        .set_joint(humanoid::Joint::RightShoulderYaw, 0.0)
        .await
        .unwrap();
    robot
        .set_joint(humanoid::Joint::LeftElbowYaw, 0.0)
        .await
        .unwrap();
    robot
        .set_joint(humanoid::Joint::LeftShoulderPitch, 0.0)
        .await
        .unwrap();
    robot
        .set_joint(humanoid::Joint::LeftShoulderYaw, 0.0)
        .await
        .unwrap();
}

// 0 -90 90

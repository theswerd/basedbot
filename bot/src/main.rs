use std::{collections::BTreeMap, io::Read, time::Duration};

use ::humanoid::{Humanoid, Joint};
use mini_robot::{Frame, MiniRobot};
use serde_json::from_str;

pub mod humanoid;
pub mod mini_robot;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = zeroth::Client::connect("grpc://192.168.42.1:50051").await;

    let client = match client {
        Ok(client) => client,
        Err(e) => panic!("Failed to connect to the server: {:?}", e),
    };

    println!("Connected!");

    // client.disable_movement().await.unwrap();

    // client.enable_movement().await.unwrap();

    let mut robot = MiniRobot::new(client);

    robot.calibrate().await?;

    println!("Calibrated");

    tokio::time::sleep(Duration::from_secs(1)).await;

    initial_position(&mut robot).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let frames = file_to_frames(
        "/Users/benswerdlow/Documents/GitHub/basedbot/pose_mappings/flapping_motion_2.json",
    )?;

    std::thread::sleep(Duration::from_secs(1));

    for frame in frames {
        robot.push_frame(frame);
    }

    // loop {
    //     println!("LOOPing {}", robot.queue.len());
    //     let out = robot.step().await?;
    //     if !out {
    //         break;
    //     }
    // }
    // for frame in frames.iter() {
    //     std::thread::sleep(Duration::from_millis(100));

    //     for (joint, value) in &frame.joints {
    //         robot.set_joint(joint.clone(), value.clone()).await.unwrap();
    //     }
    // }

    // println!("FRAMES: {:?}", frames);
    tokio::time::sleep(Duration::from_millis(200)).await;

    Ok(())
}

pub fn file_to_frames(file: &str) -> eyre::Result<Vec<Frame>> {
    let mut string = String::new();
    std::fs::File::open(file)
        .unwrap()
        .read_to_string(&mut string)?;

    let json: serde_json::Value = serde_json::from_str(&string)?;
    let json = json
        .as_array()
        .ok_or_else(|| eyre::eyre!("Expected JSON array"))?
        .clone();
    let mut frames: Vec<Frame> = Vec::new();
    for frame in json {
        let frame = frame
            .as_object()
            .ok_or_else(|| eyre::eyre!("Expected JSON object"))?
            .clone();
        let mut joints = BTreeMap::new();
        for (joint_id, joint_value) in frame.into_iter() {
            let joint_servo_id = zeroth::ServoId::try_from(from_str::<i32>(&joint_id)?)?;
            let humanoid_id: Joint = Joint::try_from(crate::humanoid::ServoId(joint_servo_id))?;

            joints.insert(
                humanoid_id,
                joint_value
                    .as_f64()
                    .ok_or_else(|| eyre::eyre!("Expected floating point value"))?
                    as f32,
            );
            // frame.insert(joint
        }
        frames.push(Frame { joints });
    }

    Ok(frames)
}

pub async fn initial_position(robot: &mut impl Humanoid) -> eyre::Result<()> {
    let mut initial_joints_btree = BTreeMap::new();
    initial_joints_btree.insert(Joint::RightElbowYaw, 0.0);
    initial_joints_btree.insert(Joint::LeftElbowYaw, 0.0);
    initial_joints_btree.insert(Joint::RightShoulderPitch, 90.0);
    initial_joints_btree.insert(Joint::LeftShoulderPitch, 90.0);

    initial_joints_btree.insert(Joint::RightShoulderYaw, 0.0);
    initial_joints_btree.insert(Joint::LeftShoulderYaw, 0.0);
    initial_joints_btree.insert(Joint::LeftAnklePitch, 0.0);
    initial_joints_btree.insert(Joint::RightAnklePitch, 0.0); // TODO: REFVESRSE
    initial_joints_btree.insert(Joint::LeftHipPitch, 90.0); // TODO: REVERSE
    initial_joints_btree.insert(Joint::RightHipPitch, 0.0);
    initial_joints_btree.insert(Joint::LeftHipYaw, 90.0); // TODO: REVERSE
    initial_joints_btree.insert(Joint::RightHipYaw, 0.0);
    robot.set_joints(initial_joints_btree).await.unwrap();
    // robot
    //     .set_joint(humanoid::Joint::RightElbowYaw, 0.0)
    //     .await
    //     .unwrap();
    // robot
    //     .set_joint(humanoid::Joint::RightShoulderPitch, 0.0)
    //     .await
    //     .unwrap();
    // robot
    //     .set_joint(humanoid::Joint::RightShoulderYaw, 0.0)
    //     .await
    //     .unwrap();
    // robot
    //     .set_joint(humanoid::Joint::LeftElbowYaw, 0.0)
    //     .await
    //     .unwrap();
    // robot
    //     .set_joint(humanoid::Joint::LeftShoulderPitch, 0.0)
    //     .await
    //     .unwrap();
    // robot
    //     .set_joint(humanoid::Joint::LeftShoulderYaw, 0.0)
    //     .await
    //     .unwrap();
    Ok(())
}

// 0 -90 90

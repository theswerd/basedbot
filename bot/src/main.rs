use std::{collections::BTreeMap, io::Read, sync::Arc, time::Duration};

use ::humanoid::{Frame, FrameQueue, Humanoid, Joint, Runtime};
use mini_robot::MiniRobot;
use serde::Deserialize;
use serde_json::from_str;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

pub mod humanoid;
pub mod mini_robot;
pub mod k_bot;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = zeroth::Client::connect("grpc://192.168.42.1:50051").await;

    let mut client = match client {
        Ok(client) => client,
        Err(e) => panic!("Failed to connect to the server: {:?}", e),
    };

    println!("Connected!");

    client.enable_movement().await.unwrap();

    // return Ok(());
    // // client.enable_movement().await.unwrap();

    let robot = MiniRobot::new(client);
    let robot = ::humanoid::Runtime::new(robot);

    robot.lock().await.calibrate().await?;

    println!("Calibrated");

    tokio::time::sleep(Duration::from_secs(1)).await;

    initial_position(&robot).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // let queue = robot.queue.clone();

    stream_frame_from_server(robot).await?;

    Ok(())
}

pub async fn stream_frame_from_server<H: Humanoid>(
    mut robot: Runtime<H>,
    // frame_queue: Arc<crossbeam::queue::SegQueue<Frame>>,
) -> eyre::Result<()> {
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8020").await?;
    let app = Router::new()
        .route("/status", get(|| async { "OK" }))
        .route("/frame", post(frame_handler))
        .with_state(robot.queue());

    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // tokio::task::spawn(async move {
    //     loop {
    //         println!("LOOPing {}", robot.queue.len());
    //         let out = robot.step().await.unwrap();
    //         if !out {
    //             break;
    //         }
    //     }
    // }).await.unwrap();

    let _handle = tokio::spawn(async {
        println!("Listening on http://{}", tcp_listener.local_addr().unwrap());

        axum::serve(tcp_listener, app.into_make_service())
            .await
            .unwrap();
    });

    println!("Run loop started");
    loop {
        println!("LOOPing {}", robot.queue_len());

        let out = robot.step().await.unwrap();
        // if !out {
        //     break;
        // }
    }
    Ok(())
}

pub async fn load_and_run_frames<H: Humanoid>(robot: &mut Runtime<H>) {
    let frames =
        file_to_frames("/Users/benswerdlow/Documents/GitHub/basedbot/pose_mappings/pose_data.json")
            .unwrap();

    tokio::time::sleep(Duration::from_secs(1)).await;

    for frame in frames {
        robot.push_frame(frame);
    }

    loop {
        println!("LOOPing {}", robot.queue_len());
        let out = robot.step().await.unwrap();
        if !out {
            break;
        }
    }
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
        // let frame = frame
        //     .as_object()
        //     .ok_or_else(|| eyre::eyre!("Expected JSON object"))?
        //     .clone();
        // let mut joints = BTreeMap::new();
        // for (joint_id, joint_value) in frame.into_iter() {
        //     let joint_servo_id = zeroth::ServoId::try_from(from_str::<i32>(&joint_id)?)?;
        //     let humanoid_id: Joint = Joint::try_from(crate::humanoid::ServoId(joint_servo_id))?;

        //     joints.insert(
        //         humanoid_id,
        //         joint_value
        //             .as_f64()
        //             .ok_or_else(|| eyre::eyre!("Expected floating point value"))?
        //             as f32,
        //     );
        //     // frame.insert(joint
        // }
        let joints = frame_json_to_frame(frame).unwrap().joints;

        frames.push(Frame { joints });
    }

    Ok(frames)
}

pub fn frame_json_to_frame(frame_json: serde_json::Value) -> eyre::Result<Frame> {
    let frame_json = frame_json
        .as_object()
        .ok_or_else(|| eyre::eyre!("Expected JSON object"))?
        .clone();

    let mut joints = BTreeMap::new();
    for (joint_id, joint_value) in frame_json.into_iter() {
        let joint_servo_id = zeroth::ServoId::try_from(from_str::<i32>(&joint_id)?)?;
        let humanoid_id: Joint = Joint::try_from(crate::humanoid::ServoId(joint_servo_id))?;

        joints.insert(
            humanoid_id,
            joint_value
                .as_f64()
                .ok_or_else(|| eyre::eyre!("Expected floating point value"))? as f32,
        );
        // frame.insert(joint
    }

    Ok(Frame { joints })
}

pub async fn initial_position<H: Humanoid>(robot: &Runtime<H>) -> eyre::Result<()> {
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
    initial_joints_btree.insert(Joint::LeftKneeYaw, 45.0); // is center, move to 0
    initial_joints_btree.insert(Joint::LeftKneePitch, 45.0);
    initial_joints_btree.insert(Joint::RightKneeYaw, 45.0);
    initial_joints_btree.insert(Joint::RightKneePitch, 45.0);
    initial_joints_btree.insert(Joint::LeftHipPitch, 45.0); // recenter on 0
    initial_joints_btree.insert(Joint::RightHipPitch, 45.0); // Reverse, recenter on 0

    // initial_joints_btree.insert(Joint::RightKneePitch, -90.0);

    // robot
    //     .lock()
    //     .await
    //     .set_joints(initial_joints_btree)
    //     .await
    //     .unwrap();
    let mut lock = robot.lock().await;
    for (joint, value) in initial_joints_btree {
        lock.set_joint(joint, value).await?;
    }

    Ok(())
}

// 0 -90 90

async fn frame_handler(
    State(frame_queue): State<Arc<FrameQueue>>,
    Json(payload): Json<FrameData>,
) -> (StatusCode, Json<serde_json::Value>) {
    println!("Received frame: {:?}", payload.joints);
    let frame = frame_json_to_frame(payload.joints).unwrap();

    println!("Received frame: {:?}", frame);
    frame_queue.overwrite(frame);

    (StatusCode::CREATED, Json(serde_json::json!({})))
}

// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
struct FrameData {
    joints: serde_json::Value,
}

/*
{
 joints: {
 "15": 0.0,
 }
}

*/

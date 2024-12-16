use std::collections::BTreeMap;
use std::sync::Arc;

use bon::Builder;
use kbot::ActuatorId;
use tokio::sync::Mutex;

use humanoid::Humanoid;
use humanoid::Joint;
use humanoid::JointPosition;

#[derive(Clone)]
pub struct KBot {
    client: Arc<Mutex<kbot::Client>>,
    calibration: KBotCalibration,
}

#[derive(Builder, Clone, Default)]
pub struct KBotCalibration {
    // shoulder
    pub left_shoulder_yaw_min: f32,
    pub left_shoulder_yaw_max: f32,
    pub left_elbow_yaw_min: f32,
    pub left_elbow_yaw_max: f32,
    pub right_elbow_yaw_min: f32,
    pub right_elbow_yaw_max: f32,
    pub left_shoulder_pitch_min: f32,
    pub left_shoulder_pitch_max: f32,
    pub right_shoulder_pitch_min: f32,
    pub right_shoulder_pitch_max: f32,
    pub right_shoulder_yaw_min: f32,
    pub right_shoulder_yaw_max: f32,

    // hip
    pub left_hip_pitch_min: f32,
    pub left_hip_pitch_max: f32,
    pub left_hip_yaw_min: f32,
    pub left_hip_yaw_max: f32,
    // pub left_hip_roll_min: f32,
    // pub left_hip_roll_max: f32,
    pub right_hip_pitch_min: f32,
    pub right_hip_pitch_max: f32,
    pub right_hip_yaw_min: f32,
    pub right_hip_yaw_max: f32,
    // pub right_hip_roll_min: f32,
    // pub right_hip_roll_max: f32,

    // knee
    pub left_knee_pitch_min: f32,
    pub left_knee_pitch_max: f32,
    pub right_knee_pitch_min: f32,
    pub right_knee_pitch_max: f32,
    pub left_knee_yaw_min: f32,
    pub left_knee_yaw_max: f32,
    pub right_knee_yaw_min: f32,
    pub right_knee_yaw_max: f32,

    // ankle
    pub left_ankle_pitch_min: f32,
    pub left_ankle_pitch_max: f32,
    pub right_ankle_pitch_min: f32,
    pub right_ankle_pitch_max: f32,
}

impl KBot {
    pub fn new(client: kbot::Client) -> Self {
        let client = Arc::new(tokio::sync::Mutex::new(client));

        KBot {
            client,
            calibration: Default::default(),
        }
    }
}

fn no_such_servo() -> eyre::Report {
    eyre::eyre!("No such servo")
}

impl Humanoid for KBot {
    type JointId = ActuatorId;

    async fn calibrate(&mut self) -> eyre::Result<()> {
        Ok(())
    }

    fn translate(&self, joint: Joint, value: f32) -> f32 {
        value
    }

    async fn stabilize(&mut self) -> eyre::Result<()> {
        Ok(())
    }

    async fn get_joint(&self, joint: Joint) -> eyre::Result<JointPosition> {
        let mut client = self.client.lock().await;

        //    let res = client.get_actuator_state(
        //     joint.into()
        //    ).await?;

        // Ok(JointPosition {
        //     joint,
        //     position: 0.0,
        //     speed: 0.0,
        // })
        todo!()
    }

    async fn set_joint(&mut self, joint: Joint, position: f32) -> eyre::Result<()> {
        todo!()
    }

    async fn set_joints(
        &mut self,
        joints: std::collections::BTreeMap<Joint, f32>,
    ) -> eyre::Result<()> {
        let joints = joints
            .into_iter()
            .map(|(joint, value)| {
                let servo_id: i32 = joint.into();
                eyre::Ok((servo_id.try_into()?, value))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;

        self.client.lock().await.set_positions(joints).await?;
        Ok(())
    }
}

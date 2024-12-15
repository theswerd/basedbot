use std::collections::BTreeMap;
use std::sync::Arc;

use bon::Builder;
use eyre::Ok;
use tokio::sync::Mutex;
use zeroth::JointPosition;
use zeroth::ServoId;

use crate::humanoid::Humanoid;
use crate::humanoid::Joint;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub joints: BTreeMap<Joint, f32>,
}

pub struct MiniRobot {
    current: Option<Frame>,
    queue: Arc<crossbeam::queue::SegQueue<Frame>>,
    // last_tick: Instant,
    client: Arc<Mutex<zeroth::Client>>,
    calibration: MiniRobotCalibration,
    balancing_task: tokio::task::JoinHandle<()>,
}

#[derive(Builder, Clone, Default)]
pub struct MiniRobotCalibration {
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
    pub left_hip_roll_min: f32,
    pub left_hip_roll_max: f32,
    pub right_hip_pitch_min: f32,
    pub right_hip_pitch_max: f32,
    pub right_hip_yaw_min: f32,
    pub right_hip_yaw_max: f32,
    pub right_hip_roll_min: f32,
    pub right_hip_roll_max: f32,

    // ankle
    pub left_ankle_pitch_min: f32,
    pub left_ankle_pitch_max: f32,
    pub right_ankle_pitch_min: f32,
    pub right_ankle_pitch_max: f32,
}

impl MiniRobot {
    pub fn new(client: zeroth::Client) -> Self {
        let client = Arc::new(tokio::sync::Mutex::new(client));

        let balancing_task = tokio::spawn(async move {
            //
        });

        MiniRobot {
            current: None,
            queue: Arc::new(crossbeam::queue::SegQueue::new()),
            // last_tick: Instant::now(),
            client,
            calibration: Default::default(),
            balancing_task,
        }
    }

    pub fn advance(&mut self) {
        if let Some(frame) = self.queue.pop() {
            self.current.replace(frame);
        }
    }

    pub fn push_frame(&self, frame: Frame) {
        self.queue.push(frame);
    }

    pub fn is_complete(&self, current_state: Frame) -> bool {
        if let Some(frame) = &self.current {
            return frame == &current_state;
        }

        return false;
    }

    pub async fn step(&mut self) -> eyre::Result<bool> {
        let Some(current) = &self.current.clone() else {
            return Ok(false);
        };

        let mut done = true;
        for (servo, _) in current.joints.clone().into_iter() {
            let current = self.get_joint(servo).await?;

            if current.speed.abs() != 0. {
                done = false
            }
        }

        if done {
            self.advance();
            if let Some(current) = self.current.clone() {
                self.set_joints(current.joints).await?;
            }
        }

        Ok(true)
    }
}

impl Drop for MiniRobot {
    fn drop(&mut self) {
        self.balancing_task.abort();
    }
}

impl Humanoid for MiniRobot {
    async fn calibrate(&mut self) -> eyre::Result<()> {
        // let left_shoulder_yaw_info = self.client.get_servo_info(id: ServoId::LeftShoulderYaw).await.unwrap().unwrap();
        // let right_shoulder_yaw_info = self.client.get_servo_info(12).await.unwrap().unwrap(); // Assuming ID 12 for right shoulder
        // shoulder info functions
        let left_shoulder_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftShoulderYaw)
            .await?
            .unwrap();
        let right_shoulder_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightShoulderYaw)
            .await?
            .unwrap();
        let right_elbow_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightElbowYaw)
            .await?
            .unwrap();

        let left_elbow_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftElbowYaw)
            .await?
            .unwrap();

        let left_shoulder_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftShoulderPitch)
            .await?
            .unwrap();

        let right_shoulder_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightShoulderPitch)
            .await?
            .unwrap();

        //hip info functions
        let left_hip_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipPitch)
            .await?
            .unwrap();

        let left_hip_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipYaw)
            .await?
            .unwrap();

        let left_hip_roll_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipRoll)
            .await?
            .unwrap();

        let right_hip_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipPitch)
            .await?
            .unwrap();

        let right_hip_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipYaw)
            .await?
            .unwrap();

        let right_hip_roll_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipRoll)
            .await?
            .unwrap();

        // ANKLE INFO
        let left_ankle_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftAnklePitch)
            .await?
            .unwrap();

        let right_ankle_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightAnklePitch)
            .await?
            .unwrap();

        let calibration_builder = MiniRobotCalibration::builder()
            .left_shoulder_yaw_max(left_shoulder_yaw_info.max_position)
            .left_shoulder_yaw_min(left_shoulder_yaw_info.min_position)
            .left_elbow_yaw_max(left_elbow_yaw_info.max_position)
            .left_elbow_yaw_min(left_elbow_yaw_info.min_position)
            .right_elbow_yaw_max(right_elbow_yaw_info.max_position)
            .right_elbow_yaw_min(right_elbow_yaw_info.min_position)
            .left_shoulder_pitch_max(left_shoulder_pitch_info.max_position)
            .left_shoulder_pitch_min(left_shoulder_pitch_info.min_position)
            .right_shoulder_pitch_max(right_shoulder_pitch_info.max_position)
            .right_shoulder_pitch_min(right_shoulder_pitch_info.min_position)
            .right_shoulder_yaw_max(right_shoulder_yaw_info.max_position)
            .right_shoulder_yaw_min(right_shoulder_yaw_info.min_position)
            .left_hip_pitch_min(left_hip_pitch_info.min_position)
            .left_hip_pitch_max(left_hip_pitch_info.max_position)
            .left_hip_yaw_min(left_hip_yaw_info.min_position)
            .left_hip_yaw_max(left_hip_yaw_info.max_position)
            .left_hip_roll_min(left_hip_roll_info.min_position)
            .left_hip_roll_max(left_hip_roll_info.max_position)
            .right_hip_pitch_min(right_hip_pitch_info.min_position)
            .right_hip_pitch_max(right_hip_pitch_info.max_position)
            .right_hip_yaw_min(right_hip_yaw_info.min_position)
            .right_hip_yaw_max(right_hip_yaw_info.max_position)
            .right_hip_roll_min(right_hip_roll_info.min_position)
            .right_hip_roll_max(right_hip_roll_info.max_position)
            .left_ankle_pitch_min(left_ankle_pitch_info.min_position)
            .left_ankle_pitch_max(left_ankle_pitch_info.max_position)
            .right_ankle_pitch_min(right_ankle_pitch_info.min_position)
            .right_ankle_pitch_max(right_ankle_pitch_info.max_position);

        self.calibration = calibration_builder.build();

        Ok(())
    }

    async fn set_joint(&mut self, joint: crate::humanoid::Joint, value: f32) -> eyre::Result<()> {
        let Some(servo_id) = joint.into() else {
            return Err(zeroth::Error::ServoNotFound.into());
        };

        let value = match joint {
            crate::humanoid::Joint::LeftHipPitch => {
                value * (self.calibration.left_hip_pitch_max - self.calibration.left_hip_pitch_min)
                    / 90.0
                    + self.calibration.left_hip_pitch_min
            }
            crate::humanoid::Joint::LeftHipYaw => {
                value * (self.calibration.left_hip_yaw_max - self.calibration.left_hip_yaw_min)
                    / 90.0
                    + self.calibration.left_hip_yaw_min
            }
            crate::humanoid::Joint::RightHipPitch => {
                value
                    * (self.calibration.right_hip_pitch_max - self.calibration.right_hip_pitch_min)
                    / 90.0
                    + self.calibration.right_hip_pitch_min
            }
            crate::humanoid::Joint::RightHipYaw => {
                value * (self.calibration.right_hip_yaw_max - self.calibration.right_hip_yaw_min)
                    / 90.0
                    + self.calibration.right_hip_yaw_min
            }
            crate::humanoid::Joint::LeftKneePitch => todo!(),
            crate::humanoid::Joint::LeftKneeYaw => todo!(),
            crate::humanoid::Joint::RightKneePitch => todo!(),
            crate::humanoid::Joint::RightKneeYaw => todo!(),
            crate::humanoid::Joint::LeftAnklePitch => {
                (value + 45.0)
                    * (self.calibration.left_ankle_pitch_max
                        - self.calibration.left_ankle_pitch_min)
                    / 90.0
                    + self.calibration.left_ankle_pitch_min
            }
            crate::humanoid::Joint::LeftAnkleYaw => todo!(),
            crate::humanoid::Joint::RightAnklePitch => {
                (value + 45.0)
                    * (self.calibration.right_ankle_pitch_max
                        - self.calibration.right_ankle_pitch_min)
                    / 90.0
                    + self.calibration.right_ankle_pitch_min
            }
            crate::humanoid::Joint::RightAnkleYaw => todo!(),
            crate::humanoid::Joint::LeftShoulderPitch => {
                (value + 45.0)
                    * (self.calibration.left_shoulder_pitch_max
                        - self.calibration.left_shoulder_pitch_min)
                    / 90.0
                    + self.calibration.left_shoulder_pitch_min
            }
            crate::humanoid::Joint::LeftShoulderYaw => {
                value
                    * (self.calibration.left_shoulder_yaw_max
                        - self.calibration.left_shoulder_yaw_min)
                    / 90.0
                    + self.calibration.left_shoulder_yaw_min
            }
            crate::humanoid::Joint::RightShoulderPitch => {
                (45.0 - value)
                    * (self.calibration.right_shoulder_pitch_max
                        - self.calibration.right_shoulder_pitch_min)
                    / 90.0
                    + self.calibration.right_shoulder_pitch_min
            }
            crate::humanoid::Joint::RightShoulderYaw => {
                (90.0 - value)
                    * (self.calibration.right_shoulder_yaw_max
                        - self.calibration.right_shoulder_yaw_min)
                    / 90.0
                    + self.calibration.right_shoulder_yaw_min
            }
            crate::humanoid::Joint::LeftElbowPitch => todo!(),
            crate::humanoid::Joint::LeftElbowYaw => {
                (value + 90.0)
                    * (self.calibration.left_elbow_yaw_max - self.calibration.left_elbow_yaw_min)
                    / 180.0
                    + self.calibration.left_elbow_yaw_min
            }
            crate::humanoid::Joint::RightElbowPitch => todo!(),
            crate::humanoid::Joint::RightElbowYaw => {
                (90.0 - value)
                    * (self.calibration.right_elbow_yaw_max - self.calibration.right_elbow_yaw_min)
                    / 180.0
                    + self.calibration.right_elbow_yaw_min
            }
            crate::humanoid::Joint::LeftWristPitch => todo!(),
            crate::humanoid::Joint::LeftWristYaw => todo!(),
            crate::humanoid::Joint::RightWristPitch => todo!(),
            crate::humanoid::Joint::RightWristYaw => todo!(),
            crate::humanoid::Joint::NeckPitch => todo!(),
            crate::humanoid::Joint::NeckYaw => todo!(),
        };

        self.client
            .lock()
            .await
            .set_position(JointPosition {
                id: servo_id,
                position: value,
                speed: 100.,
            })
            .await?;

        Ok(())
    }

    async fn get_joint(
        &self,
        joint: crate::humanoid::Joint,
    ) -> eyre::Result<zeroth::JointPosition> {
        match joint {
            crate::humanoid::Joint::LeftHipPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftHipPitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::LeftHipYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftHipYaw)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::RightHipPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightHipPitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::RightHipYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightHipYaw)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::LeftKneePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftKneePitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::LeftKneeYaw => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::RightKneePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightKneePitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::RightKneeYaw => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::LeftAnklePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftAnklePitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::LeftAnkleYaw => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::RightAnklePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightAnklePitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::RightAnkleYaw => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::LeftShoulderPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftShoulderPitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::LeftShoulderYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftShoulderYaw)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::RightShoulderPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightShoulderPitch)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::RightShoulderYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightShoulderYaw)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::LeftElbowPitch => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::LeftElbowYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftElbowYaw)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::RightElbowPitch => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::RightElbowYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightElbowYaw)
                    .await?
                    .unwrap();
                Ok(zeroth::JointPosition {
                    id: position.id.into(),
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            crate::humanoid::Joint::LeftWristPitch => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::LeftWristYaw => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::RightWristPitch => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::RightWristYaw => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::NeckPitch => Err(zeroth::Error::ServoNotFound.into()),
            crate::humanoid::Joint::NeckYaw => Err(zeroth::Error::ServoNotFound.into()),
        }
    }

    async fn set_joints(
        &mut self,
        joints: std::collections::BTreeMap<crate::humanoid::Joint, f32>,
    ) -> eyre::Result<()> {
        self.client
            .lock()
            .await
            .set_positions(
                joints
                    .into_iter()
                    .map(|(joint, value)| {
                        let servo_id: Option<ServoId> = joint.into();
                        JointPosition {
                            id: servo_id.unwrap(),
                            position: value,
                            speed: 100.,
                        }
                    })
                    .collect(),
            )
            .await
            .unwrap();

        Ok(())
    }
}

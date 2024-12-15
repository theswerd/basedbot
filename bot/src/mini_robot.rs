use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use bon::Builder;
use eyre::Ok;
use tokio::sync::Mutex;
use zeroth::ServoId;

use humanoid::Humanoid;
use humanoid::Joint;
use humanoid::JointPosition;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub joints: BTreeMap<Joint, f32>,
}

pub struct MiniRobot {
    current: Option<Frame>,
    pub queue: Arc<crossbeam::queue::SegQueue<Frame>>,
    client: Arc<Mutex<zeroth::Client>>,
    calibration: MiniRobotCalibration,
}

impl MiniRobot {
    pub async fn disable_movement(&mut self) {
        self.client.lock().await.disable_movement().await.unwrap();
    }

    pub async fn enable_movement(&mut self) {
        self.client.lock().await.enable_movement().await.unwrap();
    }
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

impl MiniRobot {
    pub fn new(client: zeroth::Client) -> Self {
        let client = Arc::new(tokio::sync::Mutex::new(client));

        MiniRobot {
            current: None,
            queue: Arc::new(crossbeam::queue::SegQueue::new()),
            // last_tick: Instant::now(),
            client,
            calibration: Default::default(),
        }
    }

    pub fn advance(&mut self) -> bool {
        if let Some(frame) = self.queue.pop() {
            self.current.replace(frame);
            return true;
        }
        false
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
        let current = match self.current.clone() {
            Some(current) => current,
            None => {
                if let Some(next) = self.queue.pop() {
                    self.current = Some(next.clone());
                    next
                } else {
                    return Ok(false);
                }
            }
        };

        println!("RUNNING CURRENT FRAME: {:?}", current);
        self.set_joints(current.joints.clone()).await.unwrap();

        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;

            // check if all joints are within a 5 degree of the target
            let mut done = true;
            for (joint, value) in &current.joints {
                let current = self.get_joint(joint.clone()).await?;
                let joint_position = self.translate(joint.clone(), value.clone());
                let dist = (current.position - joint_position).abs();

                let dist_check = dist > 10.0;
                if current.speed > 10.0 && dist_check {
                    println!(
                        "Re-looping looping because {:?} is {} off, it is at {}, it wants to be at {} | {}",
                        current.joint,
                        dist,
                        current.position,
                        joint_position,
                        dist_check
                    );
                    done = false;
                }
            }

            if done {
                break;
            }
        }

        Ok(self.advance())
    }
}

fn no_such_servo() -> eyre::Report {
    eyre::eyre!("No such servo")
}

impl Humanoid for MiniRobot {
    async fn stabilize(&mut self) -> eyre::Result<()> {
        println!("Stabilization not implemented");
        Ok(())
    }

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
            .ok_or_else(no_such_servo)?;

        let right_shoulder_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightShoulderYaw)
            .await?
            .ok_or_else(no_such_servo)?;
        let right_elbow_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightElbowYaw)
            .await?
            .ok_or_else(no_such_servo)?;

        let left_elbow_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftElbowYaw)
            .await?
            .ok_or_else(no_such_servo)?;

        let left_shoulder_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftShoulderPitch)
            .await?
            .ok_or_else(no_such_servo)?;

        let right_shoulder_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightShoulderPitch)
            .await?
            .ok_or_else(no_such_servo)?;

        //hip info functions
        let left_hip_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipPitch)
            .await?
            .ok_or_else(no_such_servo)?;

        let left_hip_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipYaw)
            .await?
            .ok_or_else(no_such_servo)?;

        let right_hip_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipPitch)
            .await?
            .ok_or_else(no_such_servo)?;

        let right_hip_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightHipYaw)
            .await?
            .ok_or_else(no_such_servo)?;

        // knee info
        let left_knee_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftKneePitch)
            .await?
            .ok_or_else(no_such_servo)?;

        let right_knee_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightKneePitch)
            .await?
            .ok_or_else(no_such_servo)?;

        let right_knee_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightHipRoll)
            .await?
            .ok_or_else(no_such_servo)?;

        let left_knee_yaw_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftHipRoll)
            .await?
            .ok_or_else(no_such_servo)?;

        // ANKLE INFO
        let left_ankle_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::LeftAnklePitch)
            .await?
            .ok_or_else(no_such_servo)?;

        let right_ankle_pitch_info = self
            .client
            .lock()
            .await
            .get_servo_info(ServoId::RightAnklePitch)
            .await?
            .ok_or_else(no_such_servo)?;

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
            // .left_hip_roll_min(left_hip_roll_info.min_position) // mapped to knee yaw
            // .left_hip_roll_max(left_hip_roll_info.max_position) // mapped to knee yaw
            .right_hip_pitch_min(right_hip_pitch_info.min_position)
            .right_hip_pitch_max(right_hip_pitch_info.max_position)
            .right_hip_yaw_min(right_hip_yaw_info.min_position)
            .right_hip_yaw_max(right_hip_yaw_info.max_position)
            // .right_hip_roll_min(right_hip_roll_info.min_position) // mapped to knee yaw
            // .right_hip_roll_max(right_hip_roll_info.max_position) // mapped to knee yaw
            .left_ankle_pitch_min(left_ankle_pitch_info.min_position)
            .left_ankle_pitch_max(left_ankle_pitch_info.max_position)
            .right_ankle_pitch_min(right_ankle_pitch_info.min_position)
            .right_ankle_pitch_max(right_ankle_pitch_info.max_position)
            .left_knee_pitch_min(left_knee_pitch_info.min_position)
            .left_knee_pitch_max(left_knee_pitch_info.max_position)
            .right_knee_pitch_min(right_knee_pitch_info.min_position)
            .right_knee_pitch_max(right_knee_pitch_info.max_position)
            .right_knee_yaw_min(right_knee_yaw_info.min_position)
            .right_knee_yaw_max(right_knee_yaw_info.max_position)
            .left_knee_yaw_min(left_knee_yaw_info.min_position)
            .left_knee_yaw_max(left_knee_yaw_info.max_position);

        self.calibration = calibration_builder.build();

        Ok(())
    }

    fn translate(&self, joint: Joint, value: f32) -> f32 {
        let value = match joint {
            humanoid::Joint::LeftKneeYaw => {
                value * (self.calibration.left_knee_yaw_max - self.calibration.left_knee_yaw_min)
                    / 90.0
                    + self.calibration.left_knee_yaw_min
            }
            humanoid::Joint::LeftHipPitch => {
                value * (self.calibration.left_hip_pitch_max - self.calibration.left_hip_pitch_min)
                    / 90.0
                    + self.calibration.left_hip_pitch_min
            }
            humanoid::Joint::LeftHipYaw => {
                value * (self.calibration.left_hip_yaw_max - self.calibration.left_hip_yaw_min)
                    / 90.0
                    + self.calibration.left_hip_yaw_min
            }
            humanoid::Joint::RightKneeYaw => {
                value * (self.calibration.right_knee_yaw_max - self.calibration.right_knee_yaw_min)
                    / 90.0
                    + self.calibration.right_knee_yaw_min
            }
            humanoid::Joint::RightHipPitch => {
                value
                    * (self.calibration.right_hip_pitch_max - self.calibration.right_hip_pitch_min)
                    / 90.0
                    + self.calibration.right_hip_pitch_min
            }
            humanoid::Joint::RightHipYaw => {
                value * (self.calibration.right_hip_yaw_max - self.calibration.right_hip_yaw_min)
                    / 90.0
                    + self.calibration.right_hip_yaw_min
            }
            humanoid::Joint::LeftKneePitch => {
                value
                    * (self.calibration.left_knee_pitch_max - self.calibration.left_knee_pitch_min)
                    / 90.0
                    + self.calibration.left_knee_pitch_min
            }
            humanoid::Joint::RightKneePitch => {
                value
                    * (self.calibration.right_knee_pitch_max
                        - self.calibration.right_knee_pitch_min)
                    / 90.0
                    + self.calibration.right_knee_pitch_min
            }
            humanoid::Joint::LeftHipRoll => todo!(),
            humanoid::Joint::RightHipRoll => todo!(),
            humanoid::Joint::LeftAnklePitch => {
                (value + 45.0)
                    * (self.calibration.left_ankle_pitch_max
                        - self.calibration.left_ankle_pitch_min)
                    / 90.0
                    + self.calibration.left_ankle_pitch_min
            }
            humanoid::Joint::LeftAnkleYaw => todo!(),
            humanoid::Joint::RightAnklePitch => {
                (value + 45.0)
                    * (self.calibration.right_ankle_pitch_max
                        - self.calibration.right_ankle_pitch_min)
                    / 90.0
                    + self.calibration.right_ankle_pitch_min
            }
            humanoid::Joint::RightAnkleYaw => todo!(),
            humanoid::Joint::LeftShoulderPitch => {
                (value + 45.0)
                    * (self.calibration.left_shoulder_pitch_max
                        - self.calibration.left_shoulder_pitch_min)
                    / 90.0
                    + self.calibration.left_shoulder_pitch_min
            }
            humanoid::Joint::LeftShoulderYaw => {
                value
                    * (self.calibration.left_shoulder_yaw_max
                        - self.calibration.left_shoulder_yaw_min)
                    / 90.0
                    + self.calibration.left_shoulder_yaw_min
            }
            humanoid::Joint::RightShoulderPitch => {
                (45.0 - value)
                    * (self.calibration.right_shoulder_pitch_max
                        - self.calibration.right_shoulder_pitch_min)
                    / 90.0
                    + self.calibration.right_shoulder_pitch_min
            }
            humanoid::Joint::RightShoulderYaw => {
                (90.0 - value)
                    * (self.calibration.right_shoulder_yaw_max
                        - self.calibration.right_shoulder_yaw_min)
                    / 90.0
                    + self.calibration.right_shoulder_yaw_min
            }
            humanoid::Joint::LeftElbowPitch => todo!(),
            humanoid::Joint::LeftElbowYaw => {
                (value + 90.0)
                    * (self.calibration.left_elbow_yaw_max - self.calibration.left_elbow_yaw_min)
                    / 180.0
                    + self.calibration.left_elbow_yaw_min
            }
            humanoid::Joint::RightElbowPitch => todo!(),
            humanoid::Joint::RightElbowYaw => {
                (90.0 - value)
                    * (self.calibration.right_elbow_yaw_max - self.calibration.right_elbow_yaw_min)
                    / 180.0
                    + self.calibration.right_elbow_yaw_min
            }
            humanoid::Joint::LeftWristPitch => todo!(),
            humanoid::Joint::LeftWristYaw => todo!(),
            humanoid::Joint::RightWristPitch => todo!(),
            humanoid::Joint::RightWristYaw => todo!(),
            humanoid::Joint::NeckPitch => todo!(),
            humanoid::Joint::NeckYaw => todo!(),
        };

        value

        // value.clamp(0.0, 90.0)
    }

    async fn get_joint(&self, joint: humanoid::Joint) -> eyre::Result<humanoid::JointPosition> {
        match joint {
            humanoid::Joint::RightHipRoll => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::LeftHipRoll => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::LeftKneeYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftHipRoll)
                    .await?
                    .unwrap();
                Ok(humanoid::JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftHipPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftHipPitch)
                    .await?
                    .unwrap();
                println!("Base position: {:?}", position.current_position);
                Ok(humanoid::JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftHipYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftHipYaw)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightKneeYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightHipRoll)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightHipPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightHipPitch)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightHipYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightHipYaw)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftKneePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftKneePitch)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightKneePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightKneePitch)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftAnklePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftAnklePitch)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftAnkleYaw => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::RightAnklePitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightAnklePitch)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightAnkleYaw => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::LeftShoulderPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftShoulderPitch)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftShoulderYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftShoulderYaw)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightShoulderPitch => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightShoulderPitch)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightShoulderYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightShoulderYaw)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftElbowPitch => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::LeftElbowYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::LeftElbowYaw)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::RightElbowPitch => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::RightElbowYaw => {
                let position = self
                    .client
                    .lock()
                    .await
                    .get_servo_info(ServoId::RightElbowYaw)
                    .await?
                    .unwrap();
                Ok(JointPosition {
                    joint,
                    speed: position.speed,
                    position: position.current_position,
                })
            }
            humanoid::Joint::LeftWristPitch => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::LeftWristYaw => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::RightWristPitch => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::RightWristYaw => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::NeckPitch => Err(zeroth::Error::ServoNotFound.into()),
            humanoid::Joint::NeckYaw => Err(zeroth::Error::ServoNotFound.into()),
        }
    }

    async fn set_joints(
        &mut self,
        joints: std::collections::BTreeMap<humanoid::Joint, f32>,
    ) -> eyre::Result<()> {
        self.client
            .lock()
            .await
            .set_positions(
                joints
                    .into_iter()
                    .map(|(joint, value)| {
                        let servo_id: crate::humanoid::ServoId = joint.try_into()?;
                        Ok(zeroth::JointPosition {
                            id: servo_id.0,
                            position: self.translate(joint, value),
                            speed: 1000.0,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )
            .await
            .unwrap();

        Ok(())
    }
}

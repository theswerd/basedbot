use bon::Builder;
use zeroth::JointPosition;
use zeroth::ServoId;

use crate::humanoid::Humanoid;

pub struct MiniRobot {
    client: zeroth::Client,
    calibration: MiniRobotCalibration,
}

#[derive(Builder, Clone, Default)]
pub struct MiniRobotCalibration {
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

    pub left_hip_pitch_min: f32,
    pub left_hip_pitch_max: f32,
    pub left_hip_yaw_min: f32,
    pub left_hip_yaw_max: f32,
    pub right_hip_pitch_min: f32,
    pub right_hip_pitch_max: f32,
    pub right_hip_yaw_min: f32,
    pub right_hip_yaw_max: f32,
}

impl MiniRobot {
    pub fn new(client: zeroth::Client) -> Self {
        return MiniRobot {
            client,
            calibration: Default::default(),
        };
    }
}

impl Humanoid for MiniRobot {
    async fn calibrate(&mut self) -> eyre::Result<()> {
        // let left_shoulder_yaw_info = self.client.get_servo_info(id: ServoId::LeftShoulderYaw).await.unwrap().unwrap();
        // let right_shoulder_yaw_info = self.client.get_servo_info(12).await.unwrap().unwrap(); // Assuming ID 12 for right shoulder
        let left_shoulder_yaw_info = self
            .client
            .get_servo_info(ServoId::LeftShoulderYaw)
            .await
            .unwrap()
            .unwrap();
        let right_shoulder_yaw_info = self
            .client
            .get_servo_info(ServoId::RightShoulderYaw)
            .await
            .unwrap()
            .unwrap();
        let right_elbow_yaw_info = self
            .client
            .get_servo_info(ServoId::RightElbowYaw)
            .await
            .unwrap()
            .unwrap();

        let left_elbow_yaw_info = self
            .client
            .get_servo_info(ServoId::LeftElbowYaw)
            .await
            .unwrap()
            .unwrap();

        let left_shoulder_pitch_info = self
            .client
            .get_servo_info(ServoId::LeftShoulderPitch)
            .await
            .unwrap()
            .unwrap();

        let right_shoulder_pitch_info = self
            .client
            .get_servo_info(ServoId::RightShoulderPitch)
            .await
            .unwrap()
            .unwrap();

        let left_hip_pitch_info = self
            .client
            .get_servo_info(ServoId::LeftHipPitch)
            .await
            .unwrap()
            .unwrap();

        let left_hip_yaw_info = self
            .client
            .get_servo_info(ServoId::LeftHipYaw)
            .await
            .unwrap()
            .unwrap();

        let right_hip_pitch_info = self
            .client
            .get_servo_info(ServoId::LeftHipPitch)
            .await
            .unwrap()
            .unwrap();

        let right_hip_yaw_info = self
            .client
            .get_servo_info(ServoId::LeftHipYaw)
            .await
            .unwrap()
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
            .right_hip_pitch_min(right_hip_pitch_info.min_position)
            .right_hip_pitch_max(right_hip_pitch_info.max_position)
            .right_hip_yaw_min(right_hip_yaw_info.min_position)
            .right_hip_yaw_max(right_hip_yaw_info.max_position);

        self.calibration = calibration_builder.build();

        Ok(())
    }

    async fn set_left_shoulder_yaw(&mut self, yaw: f32) -> eyre::Result<()> {
        let yaw = yaw
            * (self.calibration.left_shoulder_yaw_max - self.calibration.left_shoulder_yaw_min)
            / 90.0
            + self.calibration.left_shoulder_yaw_min;

        let _ = self
            .client
            .set_position(JointPosition {
                id: ServoId::LeftShoulderYaw,
                position: yaw,
                speed: 100.0,
            })
            .await;
        Ok(())
    }

    async fn set_left_elbow_yaw(&mut self, yaw: f32) -> eyre::Result<()> {
        let yaw = (yaw + 90.0)
            * (self.calibration.left_elbow_yaw_max - self.calibration.left_elbow_yaw_min)
            / 180.0
            + self.calibration.left_elbow_yaw_min;

        self.client
            .set_position(JointPosition {
                id: ServoId::LeftElbowYaw,
                position: yaw,
                speed: 100.0,
            })
            .await
            .unwrap();
        Ok(())
    }

    async fn set_right_elbow_yaw(&mut self, yaw: f32) -> eyre::Result<()> {
        let yaw = (90.0 - yaw)
            * (self.calibration.right_elbow_yaw_max - self.calibration.right_elbow_yaw_min)
            / 180.0
            + self.calibration.right_elbow_yaw_min;

        self.client
            .set_position(JointPosition {
                id: ServoId::RightElbowYaw,
                position: yaw,
                speed: 100.0,
            })
            .await
            .unwrap();
        Ok(())
    }

    async fn set_right_shoulder_yaw(&mut self, yaw: f32) -> eyre::Result<()> {
        let yaw = (90.0 - yaw)
            * (self.calibration.right_shoulder_yaw_max - self.calibration.right_shoulder_yaw_min)
            / 90.0
            + self.calibration.right_shoulder_yaw_min;

        let _ = self
            .client
            .set_position(JointPosition {
                id: ServoId::RightShoulderYaw, // Assuming ID 1 for right shoulder
                position: yaw,
                speed: 100.0,
            })
            .await;
        Ok(())
    }

    async fn set_left_hip_yaw(&mut self, yaw: f32) -> eyre::Result<()> {
        let yaw = yaw * (self.calibration.left_hip_yaw_max - self.calibration.left_hip_yaw_min)
            / 90.0
            + self.calibration.left_hip_yaw_min;

        let _ = self
            .client
            .set_position(JointPosition {
                id: ServoId::LeftHipYaw,
                position: yaw,
                speed: 100.0,
            })
            .await;
        Ok(())
    }

    async fn set_left_hip_pitch(&mut self, pitch: f32) -> eyre::Result<()> {
        let pitch = pitch
            * (self.calibration.left_hip_pitch_max - self.calibration.left_hip_pitch_min)
            / 90.0
            + self.calibration.left_hip_pitch_min;

        let _ = self
            .client
            .set_position(JointPosition {
                id: ServoId::LeftHipPitch,
                position: pitch,
                speed: 100.0,
            })
            .await;
        Ok(())
    }

    async fn set_right_hip_yaw(&mut self, yaw: f32) -> eyre::Result<()> {
        let yaw = yaw * (self.calibration.right_hip_yaw_max - self.calibration.right_hip_yaw_min)
            / 90.0
            + self.calibration.right_hip_yaw_min;

        let _ = self
            .client
            .set_position(JointPosition {
                id: ServoId::LeftHipYaw,
                position: yaw,
                speed: 100.0,
            })
            .await;
        Ok(())
    }

    async fn set_right_hip_pitch(&mut self, pitch: f32) -> eyre::Result<()> {
        let pitch = pitch
            * (self.calibration.right_hip_pitch_max - self.calibration.right_hip_pitch_min)
            / 90.0
            + self.calibration.right_hip_pitch_min;

        let _ = self
            .client
            .set_position(JointPosition {
                id: ServoId::LeftHipPitch,
                position: pitch,
                speed: 100.0,
            })
            .await;
        Ok(())
    }
}

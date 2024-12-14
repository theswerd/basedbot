use bon::Builder;
use tokio::io::Join;
use zeroth::{JointPosition, ServoId};

use crate::humanoid::Humanoid;

pub struct MiniRobot {
    client: zeroth::Client,
    calibration: Option<MiniRobotCalibration>,
}

#[derive(Builder, Clone)]
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
}

impl MiniRobot {
    pub fn new(client: zeroth::Client) -> Self {
        return MiniRobot {
            client,
            calibration: None,
        };
    }
}

impl Humanoid for MiniRobot {
    async fn calibrate(&mut self) -> Result<(), ()> {
        let left_shoulder_yaw_info = self
            .client
            .get_servo_info(ServoId::LeftElbowYaw)
            .await
            .unwrap()
            .unwrap();
        let left_elbow_yaw_info = self
            .client
            .get_servo_info(ServoId::LeftElbowYaw)
            .await
            .unwrap()
            .unwrap();
        let right_elbow_yaw_info = self
            .client
            .get_servo_info(ServoId::RightElbowYaw)
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
            .right_shoulder_pitch_min(right_shoulder_pitch_info.min_position);

        self.calibration = Some(calibration_builder.build());

        Ok(())
    }

    async fn set_left_shoulder_yaw(&mut self, yaw: f32) -> Result<(), ()> {
        let calibration = self.calibration.clone().unwrap();
        let yaw = yaw * (calibration.left_shoulder_yaw_max - calibration.left_shoulder_yaw_min)
            / 90.0
            + calibration.left_shoulder_yaw_min;

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

    async fn set_left_elbow_yaw(&mut self, yaw: f32) -> Result<(), ()> {
        let calibration = self.calibration.clone().unwrap();
        let yaw = (yaw + 90.0) * (calibration.left_elbow_yaw_max - calibration.left_elbow_yaw_min)
            / 180.0
            + calibration.left_elbow_yaw_min;

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

    async fn set_right_elbow_yaw(&mut self, yaw: f32) -> Result<(), ()> {
        let calibration = self.calibration.clone().unwrap();
        let yaw = (90.0 - yaw)
            * (calibration.right_elbow_yaw_max - calibration.right_elbow_yaw_min)
            / 180.0
            + calibration.right_elbow_yaw_min;

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

    async fn set_left_shoulder_pitch(&mut self, yaw: f32) -> Result<(), ()> {
        let calibration = self.calibration.clone().unwrap();

        let pitch = yaw * (calibration.left_shoulder_pitch_max - calibration.left_shoulder_pitch_min)
            / 90.0
            + calibration.left_shoulder_pitch_min;

        self.client
            .set_position(JointPosition {
                id: ServoId::LeftShoulderPitch,
                position: pitch,
                speed: 100.0,
            })
            .await
            .unwrap();
        Ok(())

    }
}

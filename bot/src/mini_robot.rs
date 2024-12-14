use bon::Builder;
use zeroth::JointPosition;

use crate::humanoid::Humanoid;

pub struct MiniRobot {
    client: zeroth::Client,
    calibration: Option<MiniRobotCalibration>,
}

#[derive(Builder, Clone)]
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

 impl Humanoid for MiniRobot {
       async  fn calibrate(&mut self) -> Result<(), ()> {
        let left_shoulder_yaw_info = self.client.get_servo_info(15).await.unwrap().unwrap();

        let calibration_builder = MiniRobotCalibration::builder()
            .left_shoulder_yaw_max(left_shoulder_yaw_info.max_position)
            .left_shoulder_yaw_min(left_shoulder_yaw_info.min_position);

        self.calibration = Some(calibration_builder.build());

        Ok(())
        // self.client.start_calibration(
        //     Request::new(
        //         CalibrationRequest {

        //         }
        //     )
        // )
    }

    async fn set_left_shoulder_yaw(&mut self, yaw: f32) -> Result<(), ()> {
        
        let calibration = self.calibration.clone().unwrap();
        let yaw = yaw * (calibration.left_shoulder_yaw_max - calibration.left_shoulder_yaw_min) / 90.0 + calibration.left_shoulder_yaw_min;


        let _ = self
            .client
            .set_position(JointPosition {
                id: 15,
                position: yaw,
                speed: 100.0,
            })
            .await;
        Ok(())
    }
}

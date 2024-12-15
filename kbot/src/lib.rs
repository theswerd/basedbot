#![allow(unknown_lints)]
#![allow(clippy::doc_lazy_continuation)]

// pub mod config;
mod grpc_interface;
// pub mod hal;
// pub mod services;
// pub mod telemetry;
// pub mod telemetry_types;

pub use grpc_interface::google as google_proto;
pub use grpc_interface::kos as kos_proto;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::kos_proto::actuator::GetActuatorsStateRequest;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServoInfo {
    pub id: ServoId,
    pub temperature: f32,
    pub current: f32,
    pub voltage: f32,
    pub speed: f32,
    pub current_position: f32,
    pub min_position: f32,
    pub max_position: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TorqueSetting {
    pub id: ServoId,
    pub torque: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TorqueEnableSetting {
    pub id: ServoId,
    pub enable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointPosition {
    pub id: ServoId,
    pub position: f32,
    pub speed: f32,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
    strum::EnumIter,
)]
#[repr(i32)]
pub enum ServoId {
    RightAnklePitch = 1,
    RightKneePitch = 2,
    RightHipRoll = 3,
    RightHipYaw = 4,
    RightHipPitch = 5,

    LeftAnklePitch = 6,
    LeftKneePitch = 7,
    LeftHipRoll = 8,
    LeftHipYaw = 9,
    LeftHipPitch = 10,

    RightElbowYaw = 11,
    RightShoulderYaw = 12,
    RightShoulderPitch = 13,

    LeftShoulderPitch = 14,
    LeftShoulderYaw = 15,
    LeftElbowYaw = 16,
}

// impl TryFrom<Joint> for ServoId {
//     type Error = Error;

//     fn try_from(joint: Joint) -> Result<Self, Self::Error> {
//         match joint {
//             Joint::LeftShoulderYaw => Ok(ServoId::LeftShoulderYaw),
//             Joint::LeftElbowYaw => Ok(ServoId::LeftElbowYaw),
//             Joint::RightElbowYaw => Ok(ServoId::RightElbowYaw),
//             Joint::RightShoulderPitch => Ok(ServoId::RightShoulderPitch),
//             Joint::LeftShoulderPitch => Ok(ServoId::LeftShoulderPitch),
//             Joint::RightShoulderYaw => Ok(ServoId::RightShoulderYaw),
//             Joint::LeftHipPitch => Ok(ServoId::LeftHipPitch),
//             Joint::LeftHipYaw => Ok(ServoId::LeftHipYaw),
//             Joint::RightHipPitch => Ok(ServoId::RightHipPitch),
//             Joint::RightHipYaw => Ok(ServoId::RightHipYaw),
//             Joint::LeftKneePitch => Ok(ServoId::LeftKneePitch),
//             Joint::RightKneePitch => Ok(ServoId::RightKneePitch),
//             Joint::LeftAnklePitch => Ok(ServoId::LeftAnklePitch),
//             Joint::RightAnklePitch => Ok(ServoId::RightAnklePitch),

//             _ => Err(()),
//         }
//     }
// }

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    Connection {
        #[snafu(source)]
        source: tonic::transport::Error,
    },

    #[snafu(display("{message}"))]
    Request { message: String },

    #[snafu(display("Invalid servo id"))]
    ServoNotFound,
}

impl From<tonic::Status> for Error {
    fn from(value: tonic::Status) -> Self {
        Error::Request {
            message: value.message().to_owned(),
        }
    }
}

pub struct Client {
    inner: kos_proto::actuator::actuator_service_client::ActuatorServiceClient<
        tonic::transport::Channel,
    >,
    imu: kos_proto::imu::imu_service_client::ImuServiceClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn connect(addr: impl AsRef<str>) -> Result<Self, Error> {
        let conn = kos_proto::actuator::actuator_service_client::ActuatorServiceClient::connect(
            addr.as_ref().to_string(),
        )
        .await
        .map_err(|source| Error::Connection { source })?;
        let imu_conn = kos_proto::imu::imu_service_client::ImuServiceClient::connect(
            addr.as_ref().to_string(),
        )
        .await
        .map_err(|source| Error::Connection { source })?;

        Ok(Self {
            inner: conn,
            imu: imu_conn,
        })
    }

    // pub async fn get_actuators_state(&mut self) -> Result<Vec<JointPosition>, Error> {
    //     let res = self
    //         .inner
    //         .get_actuators_state(GetActuatorsStateRequest {
    //             actuator_ids: ServoId::iter()
    //                 .map(|id| i32::from(id))
    //                 .map(|it| it as u32)
    //                 .collect(),
    //         })
    //         .await?;
    //
    //     let out: Vec<JointPosition> = res
    //         .into_inner()
    //         .states
    //         .iter()
    //         .filter_map(|v| {
    //             let position = match v.position {
    //                 Some(p) => p,
    //                 None => return None,
    //             };

    // pub async fn get_positions(&mut self) -> Result<Vec<JointPosition>, Error> {
    //     let res = self
    //         .inner
    //         .get_positions(kos_proto::actuator::Empty {})
    //         .await?;
    //     Ok(res
    //         .into_inner()
    //         .positions
    //         .into_iter()
    //         .map(|p| JointPosition {
    //             id: p.id.try_into().expect("valid servo id"),
    //             position: p.position,
    //             speed: p.speed,
    //         })
    //         .collect())
    // }
    //
    // pub async fn set_positions(&mut self, positions: Vec<JointPosition>) -> Result<(), Error> {
    //     self.inner
    //         .set_positions(kos_proto::actuator::JointPositions {
    //             positions: positions
    //                 .into_iter()
    //                 .map(|p| kos_proto::actuator::JointPosition {
    //                     id: p.id.into(),
    //                     speed: p.speed,
    //                     position: p.position,
    //                 })
    //                 .collect(),
    //         })
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn enable_movement(&mut self) -> Result<(), Error> {
    //     self.inner
    //         .enable_movement(kos_proto::actuator::Empty {})
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn disable_movement(&mut self) -> Result<(), Error> {
    //     self.inner
    //         .disable_movement(kos_proto::actuator::Empty {})
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn set_position(&mut self, pos: JointPosition) -> Result<(), Error> {
    //     self.inner
    //         .set_position(kos_proto::actuator::JointPosition {
    //             id: pos.id.into(),
    //             position: pos.position,
    //             speed: pos.speed,
    //         })
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn set_wifi_info(&mut self, wifi_info: WifiCredentials) -> Result<(), Error> {
    //     self.inner.set_wifi_info(wifi_info).await?;
    //     Ok(())
    // }
    //
    // pub async fn get_servo_info(&mut self, id: ServoId) -> Result<Option<ServoInfo>, Error> {
    //     let res = self
    //         .inner
    //         .get_servo_info(kos_proto::actuator::ServoId { id: id.into() })
    //         .await?;
    //
    //     let res = match res.into_inner().result.take() {
    //         Some(info) => info,
    //         None => return Ok(None),
    //     };
    //
    //     let info = match res {
    //         kos_proto::actuator::servo_info_response::Result::Info(info) => ServoInfo {
    //             id,
    //             temperature: info.temperature,
    //             current: info.current,
    //             voltage: info.voltage,
    //             speed: info.speed,
    //             current_position: info.current_position,
    //             min_position: info.min_position,
    //             max_position: info.max_position,
    //         },
    //         kos_proto::actuator::servo_info_response::Result::Error(err) => {
    //             return Err(Error::Request {
    //                 message: err.message,
    //             })
    //         }
    //     };
    //
    //     Ok(Some(info))
    // }
    //
    // pub async fn scan(&mut self) -> Result<Vec<i32>, Error> {
    //     let res = self.inner.scan(kos_proto::actuator::Empty {}).await?;
    //     Ok(res.into_inner().ids)
    // }
    //
    // pub async fn change_id(&mut self, from: u32, to: u32) -> Result<(), Error> {
    //     self.inner
    //         .change_id(kos_proto::actuator::IdChange {
    //             old_id: from as i32,
    //             new_id: to as i32,
    //         })
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn start_calibration(
    //     &mut self,
    //     servo: ServoId,
    //     speed: i32,
    //     current_threshold: f32,
    // ) -> Result<(), Error> {
    //     self.inner
    //         .start_calibration(kos_proto::actuator::CalibrationRequest {
    //             servo_id: servo as i32,
    //             calibration_speed: speed,
    //             current_threshold,
    //         })
    //         .await?;
    //
    //     Ok(())
    // }
    //
    // pub async fn cancel_calibration(&mut self, servo: ServoId) -> Result<(), Error> {
    //     self.inner
    //         .cancel_calibration(kos_proto::actuator::ServoId { id: servo as i32 })
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn start_video_stream(&mut self) -> Result<(), Error> {
    //     self.inner
    //         .start_video_stream(kos_proto::actuator::Empty {})
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn stop_video_stream(&mut self) -> Result<(), Error> {
    //     self.inner
    //         .stop_video_stream(kos_proto::actuator::Empty {})
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn get_video_stream_urls(&mut self) -> Result<VideoStreamUrls, Error> {
    //     let res = self
    //         .inner
    //         .get_video_stream_urls(kos_proto::Empty {})
    //         .await?;
    //     Ok(res.into_inner())
    // }
    //
    // pub async fn get_calibration_status(&mut self) -> Result<CalibrationStatus, Error> {
    //     let res = self
    //         .inner
    //         .get_calibration_status(kos_proto::Empty {})
    //         .await?;
    //     Ok(res.into_inner())
    // }
    //
    // pub async fn set_torque(&mut self, settings: Vec<TorqueSetting>) -> Result<(), Error> {
    //     let settings = settings
    //         .into_iter()
    //         .map(|s| kos_proto::actuator::TorqueSetting {
    //             id: s.id.into(),
    //             torque: s.torque,
    //         })
    //         .collect();
    //     self.inner
    //         .set_torque(kos_proto::actuator::TorqueSettings { settings })
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn set_torque_single(&mut self, servo: ServoId, torque: f32) -> Result<(), Error> {
    //     self.inner
    //         .set_torque(kos_proto::actuator::TorqueSettings {
    //             settings: vec![kos_proto::actuator::TorqueSetting {
    //                 id: servo as i32,
    //                 torque,
    //             }],
    //         })
    //         .await?;
    //     Ok(())
    // }
    // pub async fn set_torque_enable_single(
    //     &mut self,
    //     servo: ServoId,
    //     enable: bool,
    // ) -> Result<(), Error> {
    //     self.inner
    //         .set_torque_enable(kos_proto::actuator::TorqueEnableSettings {
    //             settings: vec![kos_proto::actuator::TorqueEnableSetting {
    //                 id: servo.into(),
    //                 enable,
    //             }],
    //         })
    //         .await?;
    //     Ok(())
    // }
    // pub async fn set_torque_enable(
    //     &mut self,
    //     settings: Vec<TorqueEnableSetting>,
    // ) -> Result<(), Error> {
    //     let settings = settings
    //         .into_iter()
    //             id: s.id.into(),
    //             enable: s.enable,
    //         })
    //         .collect();
    //     self.inner
    //         .set_torque_enable(kos_proto::actuator::TorqueEnableSettings { settings })
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn get_imu_data(&mut self) -> Result<ImuData, Error> {
    //     let res = self
    //         .inner
    //         .get_imu_data(kos_proto::actuator::Empty {})
    //         .await?;
    //     Ok(res.into_inner())
    // }
    // =======
    //                 let speed = match v.velocity {
    //                     Some(s) => s,
    //                     None => return None,
    //                 };
    //                 Some(JointPosition {
    //                     id: (ServoId::try_from(v.actuator_id as i32).unwrap()),
    //                     position: position as f32,
    //                     speed: speed as f32,
    //                 })
    //             })
    //             .collect();
    // >>>>>>> c3bf4f7 (merge conflicts be gone)
    //
    //         Ok(out)
    //     }

    pub async fn get_actuator_state(
        &mut self,
        servo_id: ServoId,
    ) -> Result<Vec<JointPosition>, Error> {
        let res = self
            .inner
            .get_actuators_state(GetActuatorsStateRequest {
                actuator_ids: vec![i32::from(servo_id) as u32],
            })
            .await?;

        let out: Vec<JointPosition> = res
            .into_inner()
            .states
            .iter()
            .filter_map(|v| {
                let position = match v.position {
                    Some(p) => p,
                    None => return None,
                };

                let speed = match v.velocity {
                    Some(s) => s,
                    None => return None,
                };
                Some(JointPosition {
                    id: (ServoId::try_from(v.actuator_id as i32).unwrap()),
                    position: position as f32,
                    speed: speed as f32,
                })
            })
            .collect();

        if out.len() == 0 {
            return Err(Error::ServoNotFound);
        }

        Ok(out)
    }
}

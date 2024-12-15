#![allow(unknown_lints)]
#![allow(clippy::doc_lazy_continuation)]

// pub mod config;
mod grpc_interface;
// pub mod hal;
// pub mod services;
// pub mod telemetry;
// pub mod telemetry_types;

use std::collections::BTreeMap;

pub use grpc_interface::google as google_proto;
pub use grpc_interface::kos as kos_proto;
use grpc_interface::kos::actuator::{ActuatorCommand, CommandActuatorsRequest};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::kos_proto::actuator::GetActuatorsStateRequest;
use tokio::sync::Mutex;

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
    inner: Mutex<
        kos_proto::actuator::actuator_service_client::ActuatorServiceClient<
            tonic::transport::Channel,
        >,
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
            inner: Mutex::new(conn),
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
    pub async fn set_positions(&self, positions: BTreeMap<ServoId, f32>) -> Result<(), Error> {
        self.inner
            .lock()
            .await
            .command_actuators(CommandActuatorsRequest {
                commands: positions
                    .into_iter()
                    .map(|(id, position)| ActuatorCommand {
                        actuator_id: Into::<i32>::into(id) as u32,
                        position: Some(position as f64),
                        velocity: None,
                        torque: None,
                    })
                    .collect(),
            })
            .await?;

        Ok(())
    }

    pub async fn get_actuator_state(
        &mut self,
        servo_id: ServoId,
    ) -> Result<Vec<JointPosition>, Error> {
        let res = self
            .inner
            .lock()
            .await
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

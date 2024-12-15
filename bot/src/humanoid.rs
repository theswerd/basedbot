use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Joint {
    LeftHipPitch,
    LeftHipYaw,

    RightHipPitch,
    RightHipYaw,

    LeftKneePitch,
    LeftKneeYaw,

    RightKneePitch,
    RightKneeYaw,

    LeftAnklePitch,
    LeftAnkleYaw,

    RightAnklePitch,
    RightAnkleYaw,

    LeftShoulderPitch,
    LeftShoulderYaw,

    RightShoulderPitch,
    RightShoulderYaw,

    LeftElbowPitch,
    LeftElbowYaw,

    RightElbowPitch,
    RightElbowYaw,

    LeftWristPitch,
    LeftWristYaw,

    RightWristPitch,
    RightWristYaw,

    NeckPitch,
    NeckYaw,
}

impl Into<Option<zeroth::ServoId>> for Joint {
    fn into(self) -> Option<zeroth::ServoId> {
        use zeroth::ServoId::*;
        Some(match self {
            Joint::LeftHipPitch => LeftHipPitch,
            Joint::LeftHipYaw => LeftHipYaw,
            Joint::RightHipPitch => RightHipPitch,
            Joint::RightHipYaw => RightHipYaw,
            Joint::LeftKneePitch => LeftKneePitch,
            Joint::LeftKneeYaw => None?,
            Joint::RightKneePitch => RightAnklePitch,
            Joint::RightKneeYaw => None?,
            Joint::LeftAnklePitch => LeftAnklePitch,
            Joint::LeftAnkleYaw => None?,
            Joint::RightAnklePitch => RightAnklePitch,
            Joint::RightAnkleYaw => None?,
            Joint::LeftShoulderPitch => LeftShoulderPitch,
            Joint::LeftShoulderYaw => LeftShoulderYaw,
            Joint::RightShoulderPitch => RightShoulderPitch,
            Joint::RightShoulderYaw => RightShoulderYaw,
            Joint::LeftElbowPitch => None?,
            Joint::LeftElbowYaw => LeftElbowYaw,
            Joint::RightElbowPitch => None?,
            Joint::RightElbowYaw => RightElbowYaw,
            Joint::LeftWristPitch
            | Joint::LeftWristYaw
            | Joint::RightWristPitch
            | Joint::RightWristYaw
            | Joint::NeckPitch
            | Joint::NeckYaw => None?,
        })
    }
}

pub trait Humanoid {
    fn calibrate(&mut self) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_joint(
        &mut self,
        joint: Joint,
        value: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;
}

use serde::{Deserialize, Serialize};
use zeroth::ServoId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<ServoId> for Joint {
    fn from(value: ServoId) -> Self {
        use zeroth::ServoId::*;
        match value {
            LeftHipPitch => Joint::LeftHipPitch,
            LeftHipYaw => Joint::LeftHipYaw,
            RightHipPitch => Joint::RightHipPitch,
            RightHipYaw => Joint::RightHipYaw,
            LeftKneePitch => Joint::LeftKneePitch,
            LeftShoulderPitch => Joint::LeftShoulderPitch,
            LeftShoulderYaw => Joint::LeftShoulderYaw,
            RightShoulderPitch => Joint::RightShoulderPitch,
            RightShoulderYaw => Joint::RightShoulderYaw,
            LeftElbowYaw => Joint::LeftElbowYaw,
            RightElbowYaw => Joint::RightElbowYaw,
            RightAnklePitch => Joint::RightAnklePitch,
            RightKneePitch => Joint::RightKneePitch,
            RightHipRoll => todo!(),
            LeftAnklePitch => Joint::LeftAnklePitch,
            LeftHipRoll => todo!(),
        }
    }
}

pub trait Humanoid {
    fn calibrate(&mut self) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn translate(&self, joint: crate::humanoid::Joint, value: f32) -> f32;


    fn get_joint(
        &self,
        joint: Joint,
    ) -> impl std::future::Future<Output = eyre::Result<zeroth::JointPosition>> + Send;

    fn set_joints(
        &mut self,
        joints: std::collections::BTreeMap<Joint, f32>,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_joint(
        &mut self,
        joint: Joint,
        value: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;
}

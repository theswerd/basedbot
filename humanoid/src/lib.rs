use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Joint {
    LeftHipPitch,
    LeftHipYaw,
    LeftHipRoll,

    RightHipPitch,
    RightHipYaw,
    RightHipRoll,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JointPosition {
    pub joint: Joint,
    pub position: f32,
    pub speed: f32,
}

pub trait Humanoid {
    fn calibrate(&mut self) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn translate(&self, joint: Joint, value: f32) -> f32;

    fn get_joint(
        &self,
        joint: Joint,
    ) -> impl std::future::Future<Output = eyre::Result<JointPosition>> + Send;

    fn set_joints(
        &mut self,
        joints: std::collections::BTreeMap<Joint, f32>,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;
}

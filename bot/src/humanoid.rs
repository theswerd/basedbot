use std::fmt::Display;

use humanoid::Joint;

pub struct ServoId(pub zeroth::ServoId);

impl Display for ServoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Debug for ServoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl TryFrom<Joint> for ServoId {
    type Error = eyre::Report;

    fn try_from(value: Joint) -> Result<Self, Self::Error> {
        Ok(ServoId(match value {
            Joint::LeftHipRoll => zeroth::ServoId::LeftHipRoll,
            Joint::LeftHipPitch => zeroth::ServoId::LeftHipPitch,
            Joint::LeftHipYaw => zeroth::ServoId::LeftHipYaw,
            Joint::RightHipPitch => zeroth::ServoId::RightHipPitch,
            Joint::RightHipYaw => zeroth::ServoId::RightHipYaw,
            Joint::RightHipRoll => zeroth::ServoId::RightHipRoll,
            Joint::LeftKneePitch => zeroth::ServoId::LeftKneePitch,
            Joint::LeftKneeYaw => return Err(eyre::eyre!("Unsupported joint: {:?}", value)),
            Joint::RightKneePitch => zeroth::ServoId::RightAnklePitch,
            Joint::RightKneeYaw => return Err(eyre::eyre!("Unsupported joint: {:?}", value)),
            Joint::LeftAnklePitch => zeroth::ServoId::LeftAnklePitch,
            Joint::LeftAnkleYaw => return Err(eyre::eyre!("Unsupported joint: {:?}", value)),
            Joint::RightAnklePitch => zeroth::ServoId::RightAnklePitch,
            Joint::RightAnkleYaw => return Err(eyre::eyre!("Unsupported joint: {:?}", value)),
            Joint::LeftShoulderPitch => zeroth::ServoId::LeftShoulderPitch,
            Joint::LeftShoulderYaw => zeroth::ServoId::LeftShoulderYaw,
            Joint::RightShoulderPitch => zeroth::ServoId::RightShoulderPitch,
            Joint::RightShoulderYaw => zeroth::ServoId::RightShoulderYaw,
            Joint::LeftElbowPitch => return Err(eyre::eyre!("Unsupported joint: {:?}", value)),
            Joint::LeftElbowYaw => zeroth::ServoId::LeftElbowYaw,
            Joint::RightElbowPitch => return Err(eyre::eyre!("Unsupported joint: {:?}", value)),
            Joint::RightElbowYaw => zeroth::ServoId::RightElbowYaw,
            Joint::LeftWristPitch
            | Joint::LeftWristYaw
            | Joint::RightWristPitch
            | Joint::RightWristYaw
            | Joint::NeckPitch
            | Joint::NeckYaw => return Err(eyre::eyre!("Unsupported joint: {:?}", value)),
        }))
    }
}

impl TryFrom<ServoId> for Joint {
    type Error = eyre::Report;

    fn try_from(value: ServoId) -> Result<Self, Self::Error> {
        Ok(match value.0 {
            zeroth::ServoId::LeftHipPitch => Joint::LeftHipPitch,
            zeroth::ServoId::LeftHipYaw => Joint::LeftHipYaw,
            zeroth::ServoId::RightHipPitch => Joint::RightHipPitch,
            zeroth::ServoId::RightHipYaw => Joint::RightHipYaw,
            zeroth::ServoId::LeftKneePitch => Joint::LeftKneePitch,
            zeroth::ServoId::LeftShoulderPitch => Joint::LeftShoulderPitch,
            zeroth::ServoId::LeftShoulderYaw => Joint::LeftShoulderYaw,
            zeroth::ServoId::RightShoulderPitch => Joint::RightShoulderPitch,
            zeroth::ServoId::RightShoulderYaw => Joint::RightShoulderYaw,
            zeroth::ServoId::LeftElbowYaw => Joint::LeftElbowYaw,
            zeroth::ServoId::RightElbowYaw => Joint::RightElbowYaw,
            zeroth::ServoId::RightAnklePitch => Joint::RightAnklePitch,
            zeroth::ServoId::RightKneePitch => Joint::RightKneePitch,
            zeroth::ServoId::RightHipRoll => Err(eyre::eyre!("Unsupported servo id: {:?}", value))?,
            zeroth::ServoId::LeftAnklePitch => Joint::LeftAnklePitch,
            zeroth::ServoId::LeftHipRoll => Err(eyre::eyre!("Unsupported servo id: {:?}", value))?,
        })
    }
}

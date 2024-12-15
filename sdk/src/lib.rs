mod proto {
    tonic::include_proto!("hal_pb");
}

use num_enum::{IntoPrimitive, TryFromPrimitive};
pub use proto::{AudioChunk, CalibrationStatus, ImuData, VideoStreamUrls, WifiCredentials};
use serde::{Deserialize, Serialize};
use tonic::{IntoStreamingRequest, Streaming};

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
)]
#[repr(i32)]
pub enum ServoId {
    RightAnklePitch = 1,
    RigntKneePitch = 2,
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
    inner: proto::servo_control_client::ServoControlClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn connect(addr: impl AsRef<str>) -> Result<Self, Error> {
        let conn =
            proto::servo_control_client::ServoControlClient::connect(addr.as_ref().to_string())
                .await
                .map_err(|source| Error::Connection { source })?;

        Ok(Self { inner: conn })
    }

    pub async fn get_positions(&mut self) -> Result<Vec<JointPosition>, Error> {
        let res = self.inner.get_positions(proto::Empty {}).await?;
        Ok(res
            .into_inner()
            .positions
            .into_iter()
            .map(|p| JointPosition {
                id: p.id.try_into().expect("valid servo id"),
                position: p.position,
                speed: p.speed,
            })
            .collect())
    }

    pub async fn set_positions(&mut self, positions: Vec<JointPosition>) -> Result<(), Error> {
        self.inner
            .set_positions(proto::JointPositions {
                positions: positions
                    .into_iter()
                    .map(|p| proto::JointPosition {
                        id: p.id.into(),
                        speed: p.speed,
                        position: p.position,
                    })
                    .collect(),
            })
            .await?;
        Ok(())
    }

    pub async fn enable_movement(&mut self) -> Result<(), Error> {
        self.inner.enable_movement(proto::Empty {}).await?;
        Ok(())
    }

    pub async fn disable_movement(&mut self) -> Result<(), Error> {
        self.inner.disable_movement(proto::Empty {}).await?;
        Ok(())
    }

    pub async fn set_position(&mut self, pos: JointPosition) -> Result<(), Error> {
        self.inner
            .set_position(proto::JointPosition {
                id: pos.id.into(),
                position: pos.position,
                speed: pos.speed,
            })
            .await?;
        Ok(())
    }

    pub async fn set_wifi_info(&mut self, wifi_info: WifiCredentials) -> Result<(), Error> {
        self.inner.set_wifi_info(wifi_info).await?;
        Ok(())
    }

    pub async fn get_servo_info(&mut self, id: ServoId) -> Result<Option<ServoInfo>, Error> {
        let res = self
            .inner
            .get_servo_info(proto::ServoId { id: id.into() })
            .await?;

        let res = match res.into_inner().result.take() {
            Some(info) => info,
            None => return Ok(None),
        };

        let info = match res {
            proto::servo_info_response::Result::Info(info) => ServoInfo {
                id,
                temperature: info.temperature,
                current: info.current,
                voltage: info.voltage,
                speed: info.speed,
                current_position: info.current_position,
                min_position: info.min_position,
                max_position: info.max_position,
            },
            proto::servo_info_response::Result::Error(err) => {
                return Err(Error::Request {
                    message: err.message,
                })
            }
        };

        Ok(Some(info))
    }

    pub async fn scan(&mut self) -> Result<Vec<i32>, Error> {
        let res = self.inner.scan(proto::Empty {}).await?;
        Ok(res.into_inner().ids)
    }

    pub async fn change_id(&mut self, from: u32, to: u32) -> Result<(), Error> {
        self.inner
            .change_id(proto::IdChange {
                old_id: from as i32,
                new_id: to as i32,
            })
            .await?;
        Ok(())
    }

    pub async fn start_calibration(
        &mut self,
        servo: ServoId,
        speed: i32,
        current_threshold: f32,
    ) -> Result<(), Error> {
        self.inner
            .start_calibration(proto::CalibrationRequest {
                servo_id: servo as i32,
                calibration_speed: speed,
                current_threshold,
            })
            .await?;

        Ok(())
    }

    pub async fn cancel_calibration(&mut self, servo: ServoId) -> Result<(), Error> {
        self.inner
            .cancel_calibration(proto::ServoId { id: servo as i32 })
            .await?;
        Ok(())
    }

    pub async fn start_video_stream(&mut self) -> Result<(), Error> {
        self.inner.start_video_stream(proto::Empty {}).await?;
        Ok(())
    }

    pub async fn stop_video_stream(&mut self) -> Result<(), Error> {
        self.inner.stop_video_stream(proto::Empty {}).await?;
        Ok(())
    }

    pub async fn get_video_stream_urls(&mut self) -> Result<VideoStreamUrls, Error> {
        let res = self.inner.get_video_stream_urls(proto::Empty {}).await?;
        Ok(res.into_inner())
    }

    pub async fn get_calibration_status(&mut self) -> Result<CalibrationStatus, Error> {
        let res = self.inner.get_calibration_status(proto::Empty {}).await?;
        Ok(res.into_inner())
    }

    pub async fn set_torque(&mut self, settings: Vec<TorqueSetting>) -> Result<(), Error> {
        let settings = settings
            .into_iter()
            .map(|s| proto::TorqueSetting {
                id: s.id.into(),
                torque: s.torque,
            })
            .collect();
        self.inner
            .set_torque(proto::TorqueSettings { settings })
            .await?;
        Ok(())
    }

    pub async fn set_torque_single(&mut self, servo: ServoId, torque: f32) -> Result<(), Error> {
        self.inner
            .set_torque(proto::TorqueSettings {
                settings: vec![proto::TorqueSetting {
                    id: servo as i32,
                    torque,
                }],
            })
            .await?;
        Ok(())
    }

    pub async fn set_torque_enable_single(
        &mut self,
        servo: ServoId,
        enable: bool,
    ) -> Result<(), Error> {
        self.inner
            .set_torque_enable(proto::TorqueEnableSettings {
                settings: vec![proto::TorqueEnableSetting {
                    id: servo.into(),
                    enable,
                }],
            })
            .await?;
        Ok(())
    }

    pub async fn set_torque_enable(
        &mut self,
        settings: Vec<TorqueEnableSetting>,
    ) -> Result<(), Error> {
        let settings = settings
            .into_iter()
            .map(|s| proto::TorqueEnableSetting {
                id: s.id.into(),
                enable: s.enable,
            })
            .collect();
        self.inner
            .set_torque_enable(proto::TorqueEnableSettings { settings })
            .await?;
        Ok(())
    }

    pub async fn get_imu_data(&mut self) -> Result<ImuData, Error> {
        let res = self.inner.get_imu_data(proto::Empty {}).await?;
        Ok(res.into_inner())
    }

    pub async fn upload_audio(
        &mut self,
        stream: impl IntoStreamingRequest<Message = AudioChunk>,
    ) -> Result<String, Error> {
        let res = self.inner.upload_audio(stream).await?;
        Ok(res.into_inner().audio_id)
    }

    pub async fn play_audio(&mut self, audio_id: String, volume: f32) -> Result<(), Error> {
        self.inner
            .play_audio(proto::PlayRequest { audio_id, volume })
            .await?;
        Ok(())
    }

    pub async fn start_recording(
        &mut self,
        sample_rate: i32,
        format: String,
        channels: i32,
    ) -> Result<(), Error> {
        self.inner
            .start_recording(proto::RecordingConfig {
                sample_rate,
                format,
                channels,
            })
            .await?;
        Ok(())
    }

    pub async fn stop_recording(&mut self) -> Result<(), Error> {
        self.inner.stop_recording(proto::Empty {}).await?;
        Ok(())
    }

    pub async fn get_recorded_audio(&mut self) -> Result<Streaming<AudioChunk>, Error> {
        let res = self.inner.get_recorded_audio(proto::Empty {}).await?;

        Ok(res.into_inner())
    }
}

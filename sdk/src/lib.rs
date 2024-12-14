pub mod proto {
    tonic::include_proto!("hal_pb");
}

use proto::{
    servo_control_client::ServoControlClient, AudioChunk, CalibrationStatus, ImuData,
    JointPosition, JointPositions, ServoInfo, TorqueEnableSetting, TorqueSetting, VideoStreamUrls,
    WifiCredentials,
};
use tonic::{IntoStreamingRequest, Streaming};

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    Connection {
        #[snafu(source)]
        source: tonic::transport::Error,
    },

    #[snafu(display("{message}"))]
    Request { message: String },
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
        let conn = ServoControlClient::connect(addr.as_ref().to_string())
            .await
            .map_err(|source| Error::Connection { source })?;

        Ok(Self { inner: conn })
    }

    pub async fn get_positions(&mut self) -> Result<JointPositions, Error> {
        let res = self.inner.get_positions(proto::Empty {}).await?;
        Ok(res.into_inner())
    }

    pub async fn set_positions(&mut self, positions: JointPositions) -> Result<(), Error> {
        self.inner.set_positions(positions).await?;
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
        self.inner.set_position(pos).await?;
        Ok(())
    }

    pub async fn set_wifi_info(&mut self, wifi_info: WifiCredentials) -> Result<(), Error> {
        self.inner.set_wifi_info(wifi_info).await?;
        Ok(())
    }

    pub async fn get_wifi_info(&mut self, id: u32) -> Result<Option<ServoInfo>, Error> {
        let res = self
            .inner
            .get_servo_info(proto::ServoId { id: id as i32 })
            .await?;

        let res = match res.into_inner().result.take() {
            Some(info) => info,
            None => return Ok(None),
        };

        let info = match res {
            proto::servo_info_response::Result::Info(info) => info,
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
        servo: u32,
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

    pub async fn cancel_calibration(&mut self, servo: u32) -> Result<(), Error> {
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
        self.inner
            .set_torque(proto::TorqueSettings { settings })
            .await?;
        Ok(())
    }

    pub async fn set_torque_single(&mut self, servo: u32, torque: f32) -> Result<(), Error> {
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
        servo: u32,
        enable: bool,
    ) -> Result<(), Error> {
        self.inner
            .set_torque_enable(proto::TorqueEnableSettings {
                settings: vec![proto::TorqueEnableSetting {
                    id: servo as i32,
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

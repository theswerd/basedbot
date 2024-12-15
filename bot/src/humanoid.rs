pub trait Humanoid {
    fn calibrate(&mut self) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_left_shoulder_yaw(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_left_elbow_yaw(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_right_shoulder_yaw(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_right_elbow_yaw(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_left_hip_yaw(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_left_hip_pitch(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_right_hip_yaw(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_right_hip_pitch(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_right_shoulder_pitch(
        &mut self,
        pitch: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

    fn set_left_shoulder_pitch(
        &mut self,
        pitch: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;

}

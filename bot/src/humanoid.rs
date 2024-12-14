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

    fn set_right_eblow_yaw(
        &mut self,
        yaw: f32,
    ) -> impl std::future::Future<Output = eyre::Result<()>> + Send;
}

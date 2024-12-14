
pub trait Humanoid {
    async fn calibrate(&mut self) -> Result<(), ()>;

    async fn set_left_shoulder_yaw(&mut self, yaw: f32) -> Result<(), ()>;

    async fn set_left_elbow_yaw(&mut self, yaw: f32) -> Result<(), ()>;

    async fn set_right_eblow_yaw(&mut self, yaw: f32) -> Result<(), ()>;

}
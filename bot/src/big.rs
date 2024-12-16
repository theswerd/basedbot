use std::time::Duration;

use humanoid::Humanoid;

pub mod k_bot;

#[tokio::main]
async fn main() {
    let url = "";
    let client = kbot::Client::connect(url).await.unwrap();

    let mut kbot = k_bot::KBot::new(client);

    kbot.set_joint(humanoid::Joint::RightShoulderPitch, 20.0)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;
}

use std::net::TcpStream;

#[tokio::main]
async fn main() {
    
    let client = zeroth::servo_control_client::ServoControlClient::connect(
        "grpc://192.168.42.1"
    ).await;

    
    



    
    println!("Hello, world: {:?}", client);

}

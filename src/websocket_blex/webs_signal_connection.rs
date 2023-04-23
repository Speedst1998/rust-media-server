use websocket::{ClientBuilder, OwnedMessage};
use log::{debug, error, info};

pub fn connect_to_signaling() {
    let mut client = ClientBuilder::new("ws://signal-service-m7vo.onrender.com/connect/v1/mediaServer/tatatest")
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();

    debug!("Connected to the server");
    let message = OwnedMessage::Text("Hello, server!".to_string());
    for message in client.incoming_messages() {
        let message = message.unwrap();
        match message {
            OwnedMessage::Text(text) => {
                println!("Received: {}", text)
            },
            _ => (),
        }
    }

    client.send_message(&message).unwrap();
}
use log::{debug, info};
use tungstenite::{client, Message};
use url::Url;

pub fn connect_to_signaling() {
    let url = Url::parse("wss://signal-service-m7vo.onrender.com/connect/v1/mediaServer/tatatest")
        .unwrap();
    // connect_with_config
    let (mut socket, response) = client::connect_with_config(url, None, 1).expect("Can't connect");

    info!("Connected to the server");
    info!("Response HTTP code: {}", response.status());
    info!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        info!("* {}", header);
    }

    loop {
        debug!("Waiting for message");
        let msg = socket.read_message().unwrap();
        debug!("Received: {}", msg);
        socket
            .write_message(Message::Text(msg.to_string()))
            .unwrap();
    }
}

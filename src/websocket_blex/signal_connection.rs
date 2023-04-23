use log::{debug, error, info};
use tungstenite::{connect, Message};
use url::Url;

pub fn connect_to_signaling() {
let url = Url::parse("ws://signal-service-m7vo.onrender.com/connect/v1/mediaServer/tatatest").unwrap();
let (mut socket, response) =
        connect(url).expect("Can't connect");
    // let (mut socket, response) = connect(
    //     Url::parse("ws://signal-service-m7vo.onrender.com/connect/v1/mediaServer/tatatest")
    //         .unwrap()
    // )
    // .expect("Can't connect");

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

    // loop {
    //     let msg = socket.read_message().expect("Error reading message");
    //     info!("Received: {}", msg);
    // }
    // socket.close(None);
}

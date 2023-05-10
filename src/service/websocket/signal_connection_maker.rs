use std::net::TcpStream;

use log::info;
use tungstenite::{client, stream::MaybeTlsStream, Result, WebSocket};
use url::Url;

//"wss://signal-service-m7vo.onrender.com/connect/v1/mediaServer/tatatest"
pub struct SignalConnectionMaker {}

impl SignalConnectionMaker {
    pub fn connect_to_signaling(&self) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
        let url =
            Url::parse("wss://signal-service-m7vo.onrender.com/connect/v1/mediaServer/tatatest")
                .unwrap();
        // connect_with_config
        match client::connect_with_config(url, None, 1) {
            Ok((socket, response)) => {
                info!("Connected to the server");
                info!("Response HTTP code: {}", response.status());
                info!("Response contains the following headers:");
                for (ref header, _value) in response.headers() {
                    info!("* {}", header);
                }
                return Ok(socket);
            }
            Err(err) => {
                return {
                    log::error!(
                        "Unable to open connection to signal server because error: {}",
                        err
                    );
                    Err(err)
                }
            }
        };
    }

    pub fn test() {
        print!("in here")
    }
}

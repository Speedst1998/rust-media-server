use super::messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob};
use crate::service::websocket::signal_connection_maker::SignalConnectionMaker;
use async_std::task;
use serde::Deserialize;
use std::net::TcpStream;
use std::time::Duration;
use tungstenite::{stream::MaybeTlsStream, Error::Io, WebSocket};

pub struct SocketManager<'a> {
    answer_generator: Option<AnswerGenerator<'a>>,
    pinger_job: Option<PingerJob<'a>>,
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    socket_maker: SignalConnectionMaker,
}

#[derive(Debug, Deserialize)]
struct SDPOffer {
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ReceivedMessage {
    #[serde(rename = "offer")]
    SDPOffer(SDPOffer),
    #[serde(rename = "pong")]
    Pong,
}

impl<'a> SocketManager<'a> {
    pub fn new(socket_maker: SignalConnectionMaker) -> SocketManager<'a> {
        // TODO keep trying to connect if it results in an error
        let socket = socket_maker.connect_to_signaling().unwrap();
        SocketManager {
            answer_generator: None,
            pinger_job: None,
            socket,
            socket_maker,
        }
    }

    pub fn set_answer_generator(
        &mut self,
        answer_generator: AnswerGenerator<'a>,
    ) -> &mut SocketManager<'a> {
        self.answer_generator = Some(answer_generator);
        self
    }

    pub fn set_pinger_job(&mut self, pinger_job: PingerJob<'a>) -> &mut SocketManager<'a> {
        self.pinger_job = Some(pinger_job);
        self
    }

    pub async fn listen(&mut self) {
        //TODO : Have a wrapper that converts the websocket Message to our MessageType Enum
        loop {
            match SocketManager::blocking_listen(&mut self.socket) {
                Ok(_) => panic!("SocketManager Listener returned unexpected OK"),
                Err(_) => {
                    task::sleep(Duration::from_secs(5)).await;
                    let socket_result = self.socket_maker.connect_to_signaling();
                    if socket_result.is_err() {
                        continue;
                    }
                    self.socket = socket_result.unwrap();
                }
            }
        }
    }

    fn blocking_listen(
        socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    ) -> Result<(), std::io::Error> {
        loop {
            let msg = socket.read_message();
            if msg.is_err() {
                let err = msg.unwrap_err();
                match err {
                    Io(e) => return Err(e),
                    _ => continue,
                }
            }
            let deserialized: ReceivedMessage =
                serde_json::from_str(&msg.unwrap().to_string()).unwrap();

            match deserialized {
                ReceivedMessage::SDPOffer(offer) => {
                    log::info!("{}", offer.description)
                }
                ReceivedMessage::Pong => log::info!("pong"),
            }
        }
    }
}

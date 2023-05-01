use super::messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob};
use clap::ErrorKind;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use tungstenite::Error as WsError;
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};

pub struct SocketManager<'a> {
    answer_generator: Option<&'a AnswerGenerator<'a>>,
    pinger_job: Option<&'a PingerJob<'a>>,
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
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
    pub fn new(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> SocketManager<'a> {
        SocketManager {
            answer_generator: None,
            pinger_job: None,
            socket,
        }
    }
    pub fn set_answer_generator(
        &mut self,
        answer_generator: &'a AnswerGenerator,
    ) -> &mut SocketManager<'a> {
        self.answer_generator = Some(answer_generator);
        self
    }

    pub fn set_pinger_job(&mut self, pinger_job: &'a PingerJob) -> &mut SocketManager<'a> {
        self.pinger_job = Some(pinger_job);
        self
    }

    pub async fn listen(&mut self) {
        //TODO : Have a wrapper that converts the websocket Message to our MessageType Enum
        SocketManager::blocking_listen(&mut self.socket);
    }

    pub fn blocking_listen(
        socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    ) -> tungstenite::Result<(), WsError> {
        loop {
            let msg = socket.read_message();
            if msg.is_err() {
                let err = msg.unwrap_err();
                log::error!("Error is {}", err);
                match err {
                    Io => return Err(err),
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

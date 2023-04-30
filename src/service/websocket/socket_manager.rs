use super::messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::net::TcpStream;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

// enum MessageType {
//     Offer(String),
//     Ping,
//     Pong,
// }

pub struct SocketManager<'a> {
    answer_generator: Option<&'a AnswerGenerator<'a>>,
    pinger_job: Option<&'a PingerJob<'a>>,
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

#[derive(Debug, Deserialize)]
struct SDPOfferStruct {
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Message {
    #[serde(rename = "offer")]
    SDPOffer(SDPOfferStruct),
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

    pub fn listen(&mut self) {
        //TODO : Have a wrapper that converts the websocket Message to our MessageType Enum
        let msg = self.socket.read_message();
        match msg {
            Ok(unwrapped_message) => {
                log::info!("{}", unwrapped_message);
                let message: Message =
                    serde_json::from_str(&unwrapped_message.to_string()).unwrap();
                match message {
                    Message::SDPOffer(offer) => log::info!("{}", offer.description),
                    Message::Pong => log::info!("pong"),
                }
            }
            Err(err) => {
                log::error!("Error is {}", err);
            }
        };
    }
}

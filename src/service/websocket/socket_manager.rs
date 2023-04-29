use crate::service::websocket::socket_manager::MessageType::Offer;
use std::{error::Error, net::TcpStream};
use tungstenite::{stream::MaybeTlsStream, WebSocket};

use super::messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob};

enum MessageType {
    Offer(String),
    Ping,
    Pong,
}

pub struct SocketManager<'a> {
    answer_generator: Option<&'a AnswerGenerator<'a>>,
    pinger_job: Option<&'a PingerJob<'a>>,
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
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
            Ok(unwrapped_message) => self
                .answer_generator
                .unwrap()
                .notify(unwrapped_message.to_string()),
            Err(err) => {
                log::error!("Error is {}", err);
            }
        };

        // match msg {
        //     Offer(offer) => self.answer_generator?.notify(offer),
        // };
    }
}


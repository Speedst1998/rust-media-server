use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::service::websocket::socket_manager::{OutgoingMessage, OutgoingType, SocketManager};

pub struct AnswerGenerator<'a> {
    pub socket_manager: Option<Arc<Mutex<SocketManager<'a>>>>,
    pub answer_service: String,
}

impl<'a> AnswerGenerator<'a> {
    pub fn notify(&self, msg: String) {
        log::info!("Notified AnswerGenerator with msg {}", msg);
    }
    pub fn set_socket_manager(
        &mut self,
        socket_manager: Arc<Mutex<SocketManager<'a>>>,
    ) -> &mut AnswerGenerator<'a> {
        self.socket_manager = Some(socket_manager);
        self
    }

    pub fn generate_answer(&self, offer: String) -> OutgoingMessage {
        let generated_answer = Ok(offer + "answer");
        let outgoing_message = match generated_answer {
            Ok(answer) => {
                info!("Generating media server sdp answer : {}", answer);
                OutgoingMessage {
                    message_type: OutgoingType::Answer,
                    message: answer,
                }
            }
            Err(err) => OutgoingMessage {
                message_type: OutgoingType::Error,
                message: err,
            },
        };

        outgoing_message
    }
}

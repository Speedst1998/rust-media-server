use std::sync::{Arc, RwLock};

use crate::service::websocket::socket_manager::{OutgoingMessage, OutgoingType, SocketManager};

pub struct AnswerGenerator<'a> {
    pub socket_manager: Option<Arc<RwLock<SocketManager<'a>>>>,
    pub answer_service: String,
}

impl<'a> AnswerGenerator<'a> {
    pub fn notify(&self, msg: String) {
        log::info!("Notified AnswerGenerator with msg {}", msg);
    }
    pub fn set_socket_manager(
        &mut self,
        socket_manager: Arc<RwLock<SocketManager<'a>>>,
    ) -> &mut AnswerGenerator<'a> {
        self.socket_manager = Some(socket_manager);
        self
    }

    pub fn generate_answer(&self, offer: String) {
        let generated_answer = Ok(offer + "answer");
        match generated_answer {
            Ok(answer) => self
                .socket_manager
                .as_ref()
                .unwrap()
                .write()
                .unwrap()
                .send_message_to_signal_sever(OutgoingMessage {
                    message_type: OutgoingType::Answer,
                    message: answer,
                }),
            Err(err) => self
                .socket_manager
                .as_ref()
                .unwrap()
                .write()
                .unwrap()
                .send_message_to_signal_sever(OutgoingMessage {
                    message_type: OutgoingType::Error,
                    message: err,
                }),
        }
    }
}

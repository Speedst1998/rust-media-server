use log::info;

use crate::service::websocket::socket_manager::{OutgoingMessage, OutgoingType};

pub struct AnswerGenerator {
    pub answer_service: String,
}

impl AnswerGenerator {
    pub fn notify(&self, msg: String) {
        log::info!("Notified AnswerGenerator with msg {}", msg);
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

use crate::service::websocket::socket_manager::SocketManager;

pub struct AnswerGenerator<'a>{
    pub socket_manager: Option<&'a SocketManager<'a>>,
    pub answer_service: String
}

impl<'a> AnswerGenerator<'a>{
    pub fn set_socket_manager(&mut self, socket_manager: &'a SocketManager<'a>) -> &mut AnswerGenerator<'a> {
        self.socket_manager = Some(socket_manager);
        self
    }
}


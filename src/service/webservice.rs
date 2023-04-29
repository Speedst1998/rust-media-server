use super::websocket::{
    self,
    messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob},
    socket_manager::SocketManager,
};
use crate::service::websocket::signal_connection_maker;

pub struct WebService {}
use websocket::signal_connection_maker::test as nottest;

pub fn test() {
    nottest();
    let answer_generator: AnswerGenerator = AnswerGenerator {
        socket_manager: None,
        answer_service: "string".to_owned(),
    };
    let pinger_job: PingerJob = PingerJob::new(None);

    let socket_connection = signal_connection_maker::connect_to_signaling();
    let mut socket_manager: SocketManager = SocketManager::new(socket_connection);

    socket_manager
        .set_answer_generator(&answer_generator)
        .set_pinger_job(&pinger_job);

    socket_manager.listen();
}

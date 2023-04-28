use super::websocket::{
    self,
    messaging::{
        self,
        answer_generator::{self, AnswerGenerator},
        pinger_job::PingerJob,
    },
    socket_manager::SocketManager,
};

pub struct WebService {}
use websocket::signal_connection_maker::test as nottest;

pub fn test() {
    nottest();
    let answer_generator: AnswerGenerator = AnswerGenerator {
        socket_manager: None,
        answer_service: "string".to_owned(),
    };
    let pinger_job: PingerJob = PingerJob::new(None);

    let mut socket_manager: SocketManager = SocketManager {
        answer_generator: Some(&answer_generator),
        pinger_job: Some(&pinger_job),
    };

    socket_manager
        .set_answer_generator(&answer_generator)
        .set_pinger_job(&pinger_job);
}

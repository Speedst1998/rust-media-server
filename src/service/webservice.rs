use std::sync::{Arc, Mutex, RwLock};

use super::websocket::{
    messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob},
    socket_manager::SocketManager,
};
use crate::service::websocket::signal_connection_maker;

pub struct WebService {}

pub async fn test() {
    let mut answer_generator: AnswerGenerator = AnswerGenerator {
        socket_manager: None,
        answer_service: "string".to_owned(),
    };
    let pinger_job: PingerJob = PingerJob::new(None);

    let connection_maker = signal_connection_maker::SignalConnectionMaker {};
    let mut socket_manager: SocketManager = SocketManager::new(connection_maker);
    let socket_manager_arc = Arc::new(RwLock::new(socket_manager));

    answer_generator.set_socket_manager(Arc::clone(&socket_manager_arc));

    socket_manager_arc
        .write()
        .unwrap()
        .set_answer_generator(answer_generator)
        .set_pinger_job(pinger_job);

    tokio::spawn(async move {
        let t = socket_manager_arc.read().unwrap();
        t.listen()
    });
    // socket_manager_arc.write().unwrap().listen().await;
    loop {}
}

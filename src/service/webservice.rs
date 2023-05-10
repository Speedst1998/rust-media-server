use std::sync::Arc;

use super::websocket::{
    messaging::{answer_generator::AnswerGenerator, pinger_job::PingerJob},
    socket_manager::SocketManager,
};
use crate::service::websocket::signal_connection_maker;
use tokio::sync::Mutex;

pub struct WebService {}

pub async fn init() {
    let mut answer_generator: AnswerGenerator = AnswerGenerator {
        socket_manager: None,
        answer_service: "string".to_owned(),
    };
    let pinger_job: PingerJob = PingerJob::new(None);

    let connection_maker = signal_connection_maker::SignalConnectionMaker {};
    let socket_manager: SocketManager = SocketManager::new(connection_maker);
    // let socket_manager_arc = Arc::new(RwLock::new(socket_manager));
    let socket_manager_arc = Arc::new(Mutex::new(socket_manager));

    answer_generator.set_socket_manager(socket_manager_arc.clone());

    socket_manager_arc
        .lock()
        .await
        .set_answer_generator(answer_generator)
        .set_pinger_job(pinger_job);

    let socket_manager_clone = socket_manager_arc.clone();
    tokio::spawn(async move {
        socket_manager_clone.lock().await.listen().await;
    });
    // socket_manager_arc.write().unwrap().listen().await;
    loop {}
}

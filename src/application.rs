use crate::service;

pub async fn start() {
    service::webservice::init().await;
}

use crate::service;

pub async fn test() {
    service::webservice::test().await;
}

use crate::api::routes;
use crate::folder_watcher::watcher;
use crate::server_constants;
use crate::service;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use log::info;

pub async fn start() -> std::io::Result<()> {
    service::webservice::init().await;

    let mut watcher: watcher::FolderWatcher = watcher::FolderWatcher::new().unwrap();
    watcher
        .async_watch(std::path::Path::new("/home/taharmustapha/dev/"))
        .await
        .unwrap();
    info!(
        "Starting web api at {}:{}",
        server_constants::SERVER_IP,
        server_constants::SERVER_PORT
    );

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .service(routes::hello::world)
            .service(routes::hello::movie)
            .service(routes::hello::play_movie)
    })
    .bind((server_constants::SERVER_IP, server_constants::SERVER_PORT))?
    .run()
    .await
}

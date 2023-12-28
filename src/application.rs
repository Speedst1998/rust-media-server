use crate::api::routes;
use crate::db;
use crate::db::databasehandler::DatabaseHandler;
use crate::folder_watcher::watcher;
use crate::gui::page;
use crate::server_constants;
use crate::service;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use log::info;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

pub async fn start() -> std::io::Result<()> {
    service::webservice::init().await;

    DatabaseHandler::new(SqliteConnectionManager::file("file.db"));
    page::start();

    let mut watcher: watcher::FolderWatcher = watcher::FolderWatcher::new().unwrap();
    watcher.async_watch(Path::new("./videos")).await.unwrap();
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

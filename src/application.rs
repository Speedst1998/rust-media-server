use crate::api::routes;
use crate::db::databasehandler::DatabaseHandler;
use crate::db::watched_folders_table::WatchedFoldersDb;
use crate::folder_watcher::watcher;
use crate::gui::page;
use crate::gui::page::Flags;
use crate::server_constants;
use crate::service;
use actix_cors::Cors;

use actix_web::{App, HttpServer};
use anyhow::Ok;
use iced::futures::TryFutureExt;
use log::info;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;
use std::path::Path;

pub async fn start() -> std::io::Result<()> {
    service::webservice::init().await;

    let db_handler = DatabaseHandler::new(SqliteConnectionManager::file("file.db"));
    //Pooled connection probably should not be in the db_handler if we just get the connection from it
    //Maybe the WatchedFoldersDb should take a db handler
    let conn: r2d2::PooledConnection<SqliteConnectionManager> = db_handler.get_connection();
    let watched_folders_db = WatchedFoldersDb::new(conn);
    let created_watched_folder = watched_folders_db
        .create(&"./videos".to_owned())
        .map(|succ| info!("{}", succ.path))
        .map_err(|err| info!("Problem parsing arguments: {err}"));

    let ui = task::spawn(async move {
        info!("Starting ui.");
        page::start(Flags { watched_folders_db }).unwrap();
    }).map_err(anyhow::Error::from);

    let mut watcher: watcher::FolderWatcher = watcher::FolderWatcher::new().unwrap();

    let watch_task = task::spawn(async move {
        info!("Starting folder watcher.");
        watcher.async_watch(Path::new("./videos")).await.unwrap();
    }).map_err(anyhow::Error::from);

    let server= HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .service(routes::hello::world)
            .service(routes::hello::movie)
            .service(routes::hello::play_movie)
    })
    .bind((server_constants::SERVER_IP, server_constants::SERVER_PORT))?
    .run();

    let server_handler = server.handle();

    let server = tokio::spawn(async move {
        info!(
            "Starting web api at {}:{}.",
            server_constants::SERVER_IP,
            server_constants::SERVER_PORT
        );
    }).map_err(anyhow::Error::from);

    let _ = tokio::try_join!(watch_task, ui, server);

    server_handler.stop(false).await;

    std::io::Result::Ok(())

}

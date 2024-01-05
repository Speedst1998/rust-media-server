use crate::api::routes;
use crate::db;
use crate::db::databasehandler::DatabaseHandler;
use crate::db::watched_folders_table::WatchedFoldersDb;
use crate::folder_watcher::watcher;
use crate::gui::page;
use crate::gui::page::Flags;
use crate::server_constants;
use crate::service;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use log::error;
use log::info;
use r2d2_sqlite::SqliteConnectionManager;
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
    info!("test");
    page::start(Flags { watched_folders_db }).unwrap();

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

use crate::service::api::routes;
use actix_web::{App, HttpServer};
use constants::server_constants;
use env_logger::Env;
use log::info;

pub mod application;
pub mod constants;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Launching app!");
    application::start().await;
    info!("Starting web api");
    HttpServer::new(|| App::new().service(routes::hello::hello))
        .bind((server_constants::SERVER_IP, server_constants::SERVER_PORT))?
        .run()
        .await
}

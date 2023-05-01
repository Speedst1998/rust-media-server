use env_logger::Env;
use log::info;
pub mod application;
pub mod service;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Launching app!");
    // application::run();
    application::test().await;
}

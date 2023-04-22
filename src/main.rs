
use log::{debug, error, info};
use env_logger::Env;
mod websocket;
use websocket::signal_connection::connect_to_signaling;


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    connect_to_signaling();
    info!("Hello, world!");
}

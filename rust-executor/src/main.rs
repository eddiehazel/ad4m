mod globals;
mod graphql;
mod js_core;
mod utils;
mod wallet;

pub mod init;

use log::{error, info};
use std::env;

use graphql::start_server;
use js_core::JsCore;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting js_core...");
    let mut js_core_handle = JsCore::start();
    js_core_handle.initialized().await;
    info!("js_core initialized.");

    info!("Starting GraphQL...");
    match start_server(js_core_handle).await {
        Ok(_) => {
            info!("GraphQL server stopped.");
            std::process::exit(0);
        }
        Err(err) => {
            error!("GraphQL server stopped with error: {}", err);
            std::process::exit(1);
        }
    }
}

#[macro_use]
extern crate lazy_static;

pub mod config;
mod globals;
pub mod graphql;
mod holochain_service;
mod js_core;
mod prolog_service;
mod utils;
mod wallet;
//use tokio;

pub mod init;
mod pubsub;
mod dapp_server;
pub mod agent;
mod db;
pub mod types;
pub mod perspectives;
pub mod languages;
mod neighbourhoods;
#[cfg(test)]
mod test_utils;


use tokio;
use std::{env, thread::JoinHandle};
use log::{info, warn};

use js_core::JsCore;

pub use config::Ad4mConfig;
pub use holochain_service::run_local_hc_services;

use crate::{dapp_server::serve_dapp, db::Ad4mDb, prolog_service::init_prolog_service, languages::LanguageController};

/// Runs the GraphQL server and the deno core runtime
pub async fn run(mut config: Ad4mConfig) -> JoinHandle<()> {
    env::set_var("RUST_LOG", "holochain=warn,wasmer_compiler_cranelift=warn,rust_executor=debug,warp::server");
    let _ = env_logger::try_init();
    config.prepare();

    info!("Initializing Ad4mDb...");
    
    Ad4mDb::init_global_instance(
        config.app_data_path
            .as_ref()
            .map(|path| std::path::Path::new(path).join("ad4m_db.sqlite").to_string_lossy().into_owned())    
            .expect("App data path not set in Ad4mConfig")
            .as_str()
    ).expect("Failed to initialize Ad4mDb");

    agent::capabilities::apps_map::set_data_file_path(
        config.app_data_path
            .as_ref()
            .map(|path| std::path::Path::new(path).join("apps_data.json").to_string_lossy().into_owned())
            .expect("App data path not set in Ad4mConfig")
        );

    if let Some(admin_credential) = &config.admin_credential {
        if admin_credential.is_empty() {
            warn!("adminCredential is not set or empty, empty token will possess admin capabilities.");
        }
    } else {
        warn!("adminCredential is not set or empty, empty token will possess admin capabilities.");
    }

    info!("Initializing Prolog service...");
    init_prolog_service().await;

    info!("Starting js_core...");
    let mut js_core_handle = JsCore::start(config.clone()).await;
    js_core_handle.initialized().await;
    info!("js_core initialized.");

    LanguageController::init_global_instance(js_core_handle.clone());

    info!("Starting GraphQL...");

    if config.run_dapp_server.unwrap() {
        std::thread::spawn(|| {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .thread_name(String::from("dapp_server"))
                .enable_all()
                .build()
                .unwrap();
            runtime.block_on(serve_dapp(8080)).unwrap();
        });
    };

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .thread_name(String::from("graphql_server"))
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(graphql::start_server(
            js_core_handle,
            config
        )).unwrap();
    })
}
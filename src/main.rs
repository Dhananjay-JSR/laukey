use std::fs;
use std::path::Path;
use std::process::exit;
use std::sync::Arc;
use axum::{routing::get, Router, ServiceExt, Json};
use axum::extract::State;
use axum::routing::post;
use simple_logger::SimpleLogger;
use tokio::signal;
use log::{debug, info, trace, warn};
use tantivy::Index;
use tantivy::schema::{Schema, STORED, STRING, TEXT};
use laukey::{ INDEX_PATH, INDEX_FOLDER, IndexPathManager, engine_init, AppState, RouterCreation};

#[tokio::main]
 async fn main() {
    simple_logger::init().unwrap();
    IndexPathManager();
    let (Index,Schema) = engine_init();
    info!("Engine Initialisation Successful");
    let shared_state = (AppState{
        engine_schema:Schema,
        engine_index:Index
    });
    info!("Starting Laukey Instance");

    match axum::Server::try_bind(&"0.0.0.0:3000".parse().unwrap()){
        Ok(ServerBuilder) => {
            let App =RouterCreation(shared_state);
            info!("Router Creation Successful");
            ServerBuilder.serve(App.into_make_service()).with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
        }
        Err(E) => {
warn!("Unable to Bind Ports , Please Kill the Process Acquiring Port 3000 and Try again");
            exit(1);
        }
    }


}



async fn shutdown_signal() {
    let ctrl_c = async {

        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
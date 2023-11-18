use std::fs;
use std::path::Path;
use std::process::exit;
use axum::{routing::get, Router, ServiceExt};
use simple_logger::SimpleLogger;
use tokio::signal;
use log::{info, trace, warn};
use laukey::{AppLayer, INDEX_PATH,INDEX_FOLDER};


#[tokio::main]
 async fn main() {
    simple_logger::init().unwrap();
    info!("Starting Laukey Instance");
    // fs::read_dir(INDEX_FOLDER)
   if !Path::new(INDEX_FOLDER).exists() {

      let FolderPath = Path::new(INDEX_FOLDER).join(INDEX_PATH);
       match fs::create_dir_all(FolderPath) {
           Ok(T)=>{
               info!("Path Creation Successful")

           }
           Err(E)=> warn!("Unable to Create Path {}",E),
       }
   }else {
       info!("Path Found Reusing Index Path")
   }


    match axum::Server::try_bind(&"0.0.0.0:3000".parse().unwrap()){
        Ok(ServerBuilder) => {


            let App = AppLayer();
            ServerBuilder.serve(App.into_make_service()).with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
        }
        Err(E) => {
warn!("Unable to Bind Ports , Please Kill the Process Acquiring Port 3000 and Try again");
            exit(1);
        }
    }
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));



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
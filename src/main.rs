use std::process::exit;
use axum::{routing::get, Router, ServiceExt};
use simple_logger::SimpleLogger;
use log::{info, trace, warn};
#[tokio::main]
 async fn main() {
     SimpleLogger::new().init().unwrap();
   info!("Starting Laukey Instance"); // trace!("Commencing yak shaving");
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    match axum::Server::try_bind(&"0.0.0.0:3000".parse().unwrap()){
        Ok(ServerBuilder) => {
            ServerBuilder.serve(app.into_make_service())
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
use axum::{Router, routing::get};
use axum::routing::post;
use axum::extract::{Path, Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MetaData{
    parentResourceId:String
}
#[derive(Serialize, Deserialize)]
struct LogsTypes {
    level: String,
    message: String,
    resourceId: String,
    timestamp: String,
    traceId:String,
    spanId:String,
    commit:String,
    metadata:MetaData

}

pub fn AppLayer() -> Router {
    let app = Router::new()
        .route("/", post(LogsInjestor));

    app
}
async fn LogsInjestor(Json(payload):Json<serde_json::Value>) {
    let PayloadData:LogsTypes = serde_json::from_value(payload).unwrap();
println!("{}",PayloadData.metadata.parentResourceId);
}


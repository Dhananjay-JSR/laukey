use std::process::exit;
use axum::{Router, routing::get};
use axum::routing::post;
use axum::extract::{Path, Query, Json};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
// use tempfile::TempDir;


// Generate a ulid

pub const INDEX_FOLDER:&'static str = "data";
pub const INDEX_PATH:&'static str = "indexData";

const INDEX_HEAP_SIZE:usize = 50_000_000;

#[derive(Serialize, Deserialize,Debug)]
struct MetaData{
    parentResourceId:String
}
#[derive(Serialize, Deserialize,Debug)]
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
        .route("/", post(logs_injestor));

    app
}
async fn logs_injestor(Json(payload):Json<serde_json::Value>) {
    let mut payload_data:LogsTypes = serde_json::from_value(payload).unwrap();
    // payload_data.uniqueIdentifier = Ulid::new().to_string();
    // println!("{:?}", payload_data);
    // The Engine Require us to define Schema Ahead of Time
    let mut schema_builder = Schema::builder();
    // STORED -> Reconstructs the Document on retrieval
    // TEXT -> Perform TTokenization
    // String -< Field Remains UnTTokenized
    schema_builder.add_text_field("level",STRING|STORED);
    schema_builder.add_text_field("message",TEXT|STORED);
    schema_builder.add_text_field("resourceId",TEXT|STORED);
    schema_builder.add_date_field("timestamp",STORED);
    schema_builder.add_text_field("traceId",STRING|STORED);
    schema_builder.add_text_field("spanId",STRING|STORED);
    schema_builder.add_text_field("commit",STRING|STORED);
    schema_builder.add_text_field("parentResourceId",STRING|STORED);
    schema_builder.add_text_field("uniqueID",STRING|STORED);
    let schema = schema_builder.build();
    match Index::create_in_dir(std::path::Path::new(INDEX_FOLDER).join(INDEX_PATH), schema.clone()) {
        Ok(Index)=>{
            match Index.writer(INDEX_HEAP_SIZE) {
                Ok(Writer)=>{
                debug!("EVERYTHING Ok !!!!!")

                }
                Err(E)=>{
                    error!("Index Write Generation Failed {}",E);
                }
            }
        },
        Err(E)=>{
            error!("Index Creation Failed {} , Exiting Program ",E);
            exit(1)
        }
    }



}



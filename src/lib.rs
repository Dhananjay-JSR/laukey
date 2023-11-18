use std::fs;
use std::process::exit;
use std::sync::Arc;
use axum::{Router, routing::get};
use axum::routing::post;
use axum::extract::{Path, Query, Json, State};
use chrono::TimeZone;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, Opstamp, TantivyError};
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
#[derive(Clone)]
pub struct AppState {
    pub engine_index:tantivy::Index,
    pub engine_schema:tantivy::schema::Schema
}

pub fn RouterCreation(shared_state:AppState) -> Router {
    let app = Router::new()
        .route("/", post(logs_injestor)).with_state(shared_state);
    app
}
async fn logs_injestor(  State(state): State<AppState>,
                        Json(payload):Json<serde_json::Value>) {
    let mut payload_data:LogsTypes = serde_json::from_value(payload).unwrap();
    let STATE = state;
    FlushToEngine(STATE.engine_index,STATE.engine_schema,payload_data).unwrap();
    info!("Commit Successful")

}

fn FlushToEngine(EngineIndex:tantivy::Index, Schema:tantivy::schema::Schema, payload_data:LogsTypes) -> tantivy::Result<Opstamp> {

    let mut index_writer = EngineIndex.writer(INDEX_HEAP_SIZE).expect("HEAP ALLOCATION FAILED");
                          // get Fields from Schema Defined in Index
                          let level = Schema.get_field("level").unwrap();
                          let message = Schema.get_field("message").unwrap();
                          let resourceId = Schema.get_field("resourceId").unwrap();
                          let timestamp = Schema.get_field("timestamp").unwrap();
                          let traceId = Schema.get_field("traceId").unwrap();
                          let spanId= Schema.get_field("spanId").unwrap();
                          let commit = Schema.get_field("commit").unwrap();
                          let parentResourceId = Schema.get_field("parentResourceId").unwrap();
                          let UniqueIdentifier = Schema.get_field("uniqueID").unwrap();
                          //   Prepare a Document to Commit
                          let mut LogDoc = Document::default();
                          LogDoc.add_text(level,payload_data.level);
                          LogDoc.add_text(message,payload_data.message);
                          LogDoc.add_text(resourceId,payload_data.resourceId);
                          // Parse the input string into a DateTime<Utc>
                          // OLD: Not Needed as Date is Already in predefined Format
                          // let datetime_utc = chrono::DateTime::parse_from_str(&payload_data.timestamp, "%Y-%m-%dT%H:%M:%SZ").unwrap();
                          let datetime_utc = chrono::DateTime::parse_from_rfc3339(&payload_data.timestamp).unwrap();
                          //  Convert the DateTime<Utc> to a DateTime<FixedOffset> with the desired time zone INGNORED AS INPUT IS IN ZULU FORMAT
                          // let datetime_with_timezone = datetime_utc.with_timezone(&chrono::FixedOffset::east_opt(5 * 3600).unwrap()); // Assuming the Server Measures Time in IST TImezone
                          // Convert the DateTime<FixedOffset> to a UNIX timestamp
                          let Parsedtimestamp = datetime_utc.timestamp();
                          // IMP: Chrono Library By Default Set in Second Format
                          LogDoc.add_date(timestamp,tantivy::DateTime::from_timestamp_secs(Parsedtimestamp));
                          LogDoc.add_text(traceId,payload_data.traceId);
                          LogDoc.add_text(spanId,payload_data.spanId);
                          LogDoc.add_text(commit,payload_data.commit);
                          LogDoc.add_text(parentResourceId,payload_data.metadata.parentResourceId);
                          LogDoc.add_text(UniqueIdentifier,Ulid::new().to_string());
    index_writer.add_document(LogDoc).expect("Unable to Add Document for Indexing");
    info!("Adding DOcument Suucessfull");
    index_writer.commit()
}


pub fn IndexPathManager(){
    // This Handles the File Creating for Indexing Values
    if !std::path::Path::new(INDEX_FOLDER).exists() {
        let FolderPath = std::path::Path::new(INDEX_FOLDER).join(INDEX_PATH);
        match fs::create_dir_all(FolderPath) {
            Ok(T)=>{
                info!("Path Creation Successful")
            }
            Err(E)=> {warn!("Unable to Create Path {} Exiting Program",E);
            std::process::exit(1);
            }

        }
    }else {
        info!("Path EXIST Reusing Index Path")
    }
}

pub  fn engine_init() -> (Index, Schema) {
    // Starts Initialize the Index Engine
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

    let DIRPath = std::path::Path::new(INDEX_FOLDER).join(INDEX_PATH);
    // Index::open_or_create(DIR,schema.clone());
    let Index = Index::open_or_create(tantivy::directory::MmapDirectory::open(DIRPath).unwrap(), schema.clone()).unwrap();
    info!("Index Allocation Successful ");
     return (Index,schema);

}



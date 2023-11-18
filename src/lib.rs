use std::process::exit;
use axum::{Router, routing::get};
use axum::routing::post;
use axum::extract::{Path, Query, Json};
use chrono::TimeZone;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, TantivyError};
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
    // payload_data.uniqueIdentifier =
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
                debug!("EVERYTHING Ok !!!!!");

                }
                Err(E)=>{
                    error!("Index Write Generation Failed {}",E);
                }
            }
        },
        Err(ErrTan)=>{
          match ErrTan {

              TantivyError::IndexAlreadyExists => {
                  debug!("Index Already Exist , Reusing It");


                  let mut Index = Index::open_in_dir(std::path::Path::new(INDEX_FOLDER).join(INDEX_PATH)).expect("TODO: panic message");
                  match Index.writer(INDEX_HEAP_SIZE){
                      Ok(mut Writer)=>{



                          let level = schema.get_field("level").unwrap();
                          let message = schema.get_field("message").unwrap();
                          let resourceId = schema.get_field("resourceId").unwrap();
                          let timestamp = schema.get_field("timestamp").unwrap();
                          let traceId = schema.get_field("traceId").unwrap();
                          let spanId= schema.get_field("spanId").unwrap();
                          let commit = schema.get_field("commit").unwrap();
                          let parentResourceId = schema.get_field("parentResourceId").unwrap();
                          let UniqueIdentifier = schema.get_field("uniqueID").unwrap();

                          let mut LogDoc = Document::default();
                          LogDoc.add_text(level,payload_data.level);
                          LogDoc.add_text(message,payload_data.message);
                          LogDoc.add_text(resourceId,payload_data.resourceId);

                          // OLD IMPLEMENTATION
                          // let parsed_datetime = chrono::NaiveDateTime::parse_from_str(&payload_data.timestamp, "%Y-%m-%dT%H:%M:%SZ").unwrap();
                          // // Create a DateTime<Utc> from the parsed NaiveDateTime
                          // let datetime_utc = chrono::Utc.from_utc_datetime(&parsed_datetime);
                          // // Convert the DateTime<Utc> to a UNIX timestamp
                          // let timestamp = datetime_utc.timestamp();


                          // Parse the input string into a DateTime<Utc>
                          // 2023-09-15T08:00:00Z
                          // let datetime_utc = chrono::DateTime::parse_from_str(&payload_data.timestamp, "%Y-%m-%dT%H:%M:%SZ").unwrap();
                          let datetime_utc = chrono::DateTime::parse_from_rfc3339(&payload_data.timestamp).unwrap();

                          //
                          // // Convert the DateTime<Utc> to a DateTime<FixedOffset> with the desired time zone INGNORED AS INPUT IS IN ZULU FORMAT
                          // let datetime_with_timezone = datetime_utc.with_timezone(&chrono::FixedOffset::east_opt(5 * 3600).unwrap()); // Assuming the Server Measures Time in IST TImezone
                          // // Convert the DateTime<FixedOffset> to a UNIX timestamp
                          let Parsedtimestamp = datetime_utc.timestamp();
                          // // Chrono Library By Default Set in Second FOrmat
                          LogDoc.add_date(timestamp,tantivy::DateTime::from_timestamp_secs(Parsedtimestamp));
                          LogDoc.add_text(traceId,payload_data.traceId);
                          LogDoc.add_text(spanId,payload_data.spanId);
                          LogDoc.add_text(commit,payload_data.commit);
                          LogDoc.add_text(parentResourceId,payload_data.metadata.parentResourceId);
                          LogDoc.add_text(UniqueIdentifier,Ulid::new().to_string());
                          Writer.add_document(LogDoc).expect("Unable to Add Document for Indexing");
                          info!("Indexing Suucessfull");
                          Writer.commit().unwrap();
                          info!("Commit Suucessfull");

                      }
                      Err(E)=>todo!()
                  }







              }
              (E)=>{
                  //   Incase of error Other than Index Alread Exist , We want to wind Up Program
                  error!("Out of Bound Error Occured {}",E);
                  panic!();
              }

          }
        }
    }





}



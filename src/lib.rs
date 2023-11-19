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
use std::collections::HashMap;
use rusqlite::Connection;
use serde_json::json;
use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, Occur, QueryClone, QueryParser};
use tantivy::schema::*;
use tantivy::{Index, Opstamp, TantivyError};
use tantivy::columnar::ColumnType::DateTime;
use tantivy::ReloadPolicy;
use tower_http::cors::{Any, CorsLayer};

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

    // let cors = CorsLayer::new()
    //     // allow `GET` and `POST` when accessing the resource
    //     .allow_methods(vec![Method::GET, Method::POST])
    //     // allow requests from any origin
    //     .allow_origin(any());


    let app = Router::new()
        .route("/", post(logs_injestor))
        .route("/search",get(SearchHandler))
        .route("/state",get(GetServerState))
        .route("/adminpass",post(UpdateRootPass))
        .layer(CorsLayer::new().allow_origin(Any)).with_state(shared_state);
    app
}



#[derive(Debug)]
struct Profiles {
    // UserID: i32,
    UserName: String,
    Password: String,
    FirstSetup:bool,
}


async fn UpdateRootPass(Json(payload):Json<serde_json::Value>){
    #[derive(Serialize,Deserialize,Debug)]
    struct AdminCred {
        userName:String,
        passWord:String
    }

    let AdminRequest:AdminCred = serde_json::from_value(payload).unwrap();
    println!("{:?}",AdminRequest)
}

async fn GetServerState() -> Json<serde_json::Value> {
    let conn = Connection::open(std::path::Path::new(INDEX_FOLDER).join("data.db")).unwrap();
    let mut stmt = conn.prepare("SELECT UserName, Password, FirstSetup FROM Profiles").unwrap();
    let User_Iter = stmt.query_map([],|row|{
        Ok(Profiles{
            UserName:row.get(0).unwrap(),
            Password:row.get(1).unwrap(),
            FirstSetup:row.get(2).unwrap()
        })
    }).unwrap();
  let MappedValue = User_Iter.map(|User|User.unwrap());
    let Test:Vec<Profiles> = MappedValue.collect();
    // println!("{}",)
    if (Test.len()==1){
        Json(json!({"newSetup":true}))
    }else {
        Json(json!({"newSetup":false}))
    }
}

async fn SearchHandler(Query(params): Query<HashMap<String, String>>, State(state): State<AppState>)->Json<serde_json::Value>{
    let StateLocal = state;
    // Creates a Reader , Set Policy such that Reader Only Reloads On Writer Commit
    let reader = StateLocal.engine_index.reader_builder().reload_policy(ReloadPolicy::OnCommit).try_into().unwrap();
// acquire a searcher -> immutable version of the index.
    let searcher = reader.searcher();
    // Final JSON response
    #[derive(Serialize, Deserialize,Debug)]
    struct JSONResponse {
        level: String,
        message: String,
        resourceId: String,
        timestamp: String,
        traceId:String,
        spanId:String,
        commit:String,
        parentResourceId:String
    }

    // Result Storage
    let mut ResultVec:Vec<JSONResponse> = vec![];

    // Store All Querries
    let mut QuerriesVec:Vec<Box<dyn tantivy::query::Query>> = vec![];


    if let Some(TimeStampQuerry) = params.get("timestamp"){
             let messageFIeld = StateLocal.engine_schema.get_field("timestamp").unwrap();
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
            // if TimeStampQuerry.contains("TO"){
                let TimeQueryString =  "timestamp:[".to_string()+TimeStampQuerry+"]";
                let Localquery =query_parser.parse_query(&TimeQueryString).unwrap();
                QuerriesVec.push(Localquery)
            // }else {
            //     let DateString = "timestamp:\"".to_string()+  TimeStampQuerry+ "\"";
            //     let Localquery =query_parser.parse_query(&DateString).unwrap();
            //     QuerriesVec.push(Localquery)
            //
            // }
    }


    // Get Individual Querries Based on Params
    if let Some(messageQuerry) = params.get("message"){
        // Field to Querry
        let messageFIeld = StateLocal.engine_schema.get_field("message").unwrap();
        // Generate Parser
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
        // Evaluate Querry
        let Localquery =query_parser.parse_query(messageQuerry).unwrap();
        QuerriesVec.push(Localquery)
   }



    if let Some(levelQuerry) = params.get("level"){
        let messageFIeld = StateLocal.engine_schema.get_field("level").unwrap();
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
        let Localquery =query_parser.parse_query(levelQuerry).unwrap();
        QuerriesVec.push(Localquery)
    }

    if let Some(resourceIdQuerry) = params.get("resourceId"){
        let messageFIeld = StateLocal.engine_schema.get_field("resourceId").unwrap();
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
        let Localquery =query_parser.parse_query(resourceIdQuerry).unwrap();
        QuerriesVec.push(Localquery)
    }

    if let Some(traceIdQuerry) = params.get("traceId"){
        let messageFIeld = StateLocal.engine_schema.get_field("traceId").unwrap();
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
        let Localquery =query_parser.parse_query(traceIdQuerry).unwrap();
        QuerriesVec.push(Localquery)
    }

    if let Some(spanIdQuerry) = params.get("spanId"){
        let messageFIeld = StateLocal.engine_schema.get_field("spanId").unwrap();
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
        let Localquery =query_parser.parse_query(spanIdQuerry).unwrap();
        QuerriesVec.push(Localquery)
    }

    if let Some(commitQuerry) = params.get("commit"){
        let messageFIeld = StateLocal.engine_schema.get_field("commit").unwrap();
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
        let Localquery =query_parser.parse_query(commitQuerry).unwrap();
        QuerriesVec.push(Localquery)
    }

    if let Some(parentResourceIdQuerry) = params.get("parentResourceId"){
        let messageFIeld = StateLocal.engine_schema.get_field("parentResourceId").unwrap();
        let query_parser = QueryParser::for_index(&StateLocal.engine_index,vec![messageFIeld]);
        let Localquery =query_parser.parse_query(parentResourceIdQuerry).unwrap();
        QuerriesVec.push(Localquery)
    }


    let BoooleanQuery = BooleanQuery::new(
        QuerriesVec.iter().map(|value|{
            (Occur::Must, value.box_clone())
        }).collect()
    );


    let top_docs = searcher.search(&BoooleanQuery, &TopDocs::with_limit(10)).unwrap();

    for (_score, doc_address) in top_docs {
        // Temp Storage Struct
        #[derive(Serialize, Deserialize,Debug)]
        struct LocalResponse {
            level: Vec<String>,
            message: Vec<String>,
            resourceId: Vec<String>,
            timestamp: Vec<String>,
            traceId:Vec<String>,
            spanId:Vec<String>,
            commit:Vec<String>,
            parentResourceId:Vec<String>
        }
        // Flatten Result
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        // COnvert to JSON serialized from STRING
        let mut LocalResponseS:LocalResponse = serde_json::from_str(&StateLocal.engine_schema.to_json(&retrieved_doc)).unwrap();
        //  Push to Result After Modification
        ResultVec.push(JSONResponse{
             level:LocalResponseS.level.join(""),
             message:LocalResponseS.message.join(""),
             parentResourceId:LocalResponseS.parentResourceId.join(""),
             resourceId:LocalResponseS.resourceId.join(""),
             commit:LocalResponseS.commit.join(""),
             spanId:LocalResponseS.spanId.join(""),
             traceId:LocalResponseS.traceId.join(""),
             timestamp:LocalResponseS.timestamp.join(""),
         })
    }
    // Send Response
    Json(json!(ResultVec))
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
    info!("Adding Document Successful");
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
    schema_builder.add_date_field("timestamp",tantivy::schema::DateOptions::from(INDEXED)
        .set_stored()
        .set_fast()
        .set_precision(tantivy::DateTimePrecision::Seconds));
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



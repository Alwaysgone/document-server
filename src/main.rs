#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

use crate::diesel::RunQueryDsl;
use actix_web::{App, HttpServer, Error, web, HttpResponse };
use actix_web::middleware;
use actix_web::error::BlockingError;
//use log::{debug, error, log_enabled, info, Level};
use std::env;
use uuid::Uuid;

use dotenv::dotenv;

//use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel::dsl::{insert_into};
use self::models::*;
use self::schema::documents::dsl::*;


pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
/*
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct CmolsDocument {
    pub id: Uuid,
    pub name: String,
    pub content: Bytes,
}
*/
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    //let hello = "Hello, world!";
    //let database_url: &'static str = "URL";
    let database_url = env::var("DATABASE_URL")
        .expect(&"DATABASE_URL must be set");
        //.to_owned();
    return SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}.", database_url));
}

async fn get_documents(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_documents(pool))
        .await
        .map(|document| HttpResponse::Ok().json(document))
        .map_err(|_| HttpResponse::InternalServerError())?)
/*
    HttpResponse::Ok()
    .header(header::CONTENT_TYPE, "text/plain")
    .body("Currently no documents .")
*/
}

fn get_all_documents(pool: web::Data<Pool>) -> Result<Vec<Document>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let all_documents = documents.load::<Document>(&conn)?;
    Ok(all_documents)
}

//curl -X POST -H "Content-Type: application/json" http://localhost:9090/documents -d "{\"name\":\"sample\"}"
//curl -X GET http://localhost:9090/documents

async fn send_document(pool: web::Data<Pool>, document: web::Json<NewDocument>) -> Result<HttpResponse, Error>  {
    //TODO also send document
    match web::block(move || store_document(pool, document)).await {
            Ok(document) => Ok(HttpResponse::Ok().json(document)),
            Err(BlockingError::Canceled) => todo!("handle canceled"),
            Err(BlockingError::Error(diesel_error)) => Ok(HttpResponse::InternalServerError().json(format!("{:?}", diesel_error))),
        }
        //.map(|row| HttpResponse::Ok().json(row))
        //.map_err(|_| HttpResponse::InternalServerError()))
    /*
    HttpResponse::Ok()
    .header(header::CONTENT_TYPE, "text/plain")
    .body(format!("Accepted id {}.", document.id))
    */
}

fn store_document(pool: web::Data<Pool>, document: web::Json<NewDocument>) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_document = NewDocument {
        name: document.name.clone(),
    };
    let rows = insert_into(documents).values(&new_document).execute(&conn)?;
    Ok(rows)
}


/*
fn get_first_cmols_document() -> CmolsDocument {
    let connection = establish_connection();
    let result = cmolsDocuments
    
    .first(&connection).expect("Cannot load first Cmols Document.");
    result
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect(&"DATABASE_URL must be set");
    let connection = establish_connection();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    //let result = cmolsDocuments.first(&connection).expect("Cannot load first Cmols Document.");
    //

    HttpServer::new(move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            // register HTTP requests handlers
            .route("/documents", web::get().to(get_documents))
            .route("/documents", web::post().to(send_document))
            //.service(tweet::list)
            //.service(tweet::get)
            //.service(tweet::create)
            //.service(tweet::delete)
            //.service(like::list)
            //.service(like::plus_one)
            //.service(like::minus_one)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
    
}

use actix_web::{web, App, HttpServer};
use sqlx::{self, migrate::MigrateDatabase, Sqlite, SqlitePool};
use dotenv;

mod models;

mod routes {
    pub mod index;
    pub mod query;
    pub mod static_files;
}

use routes::{
    index::index,
    query::query,
    static_files::static_files
};

const DB_URL: &str = "./tnea.sqlite";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        panic!("Database @{} does not exist", DB_URL);
    }
    
    let db = SqlitePool::connect(DB_URL).await.expect("Database Connection Failed");

    let host : String = dotenv::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port : u16 = dotenv::var("PORT").unwrap_or("8080".to_string()).parse().expect("Given PORT is not valid.");
    
    println!( "Listening at {}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(query)
        .service(static_files)
        .app_data(web::Data::new(db.to_owned()))
    })
    .bind((host, port))?
    .run()
    .await
}

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use sqlx::{self, migrate::MigrateDatabase, Sqlite, SqlitePool};
use dotenv;

mod models;

mod routes {
    pub mod index;
    pub mod tnea;
    pub mod josaa;
    pub mod bundle;
    pub mod not_found;
}

use routes::{
    index::index,
    tnea,
    josaa,
    bundle,
    not_found::not_found
};

const DB_URL: &str = "./db.sqlite";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        panic!("Database at '{}' does not exist", DB_URL);
    }
    
    let db = SqlitePool::connect(DB_URL).await.expect("Database Connection Failed");

    let host : String = dotenv::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port : u16 = dotenv::var("PORT").unwrap_or("8080".to_string()).parse().expect("Given PORT is not valid.");
    
    println!( "Listening at http://{}:{}", host, port);
    
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();
        
        App::new()
        .wrap(cors)
        .service(index)
        .service(tnea::query)
        .service(josaa::query)
        .service(bundle::static_files)
        .app_data(web::Data::new(db.to_owned()))
        .default_service(
            web::route().to(not_found)
        )
    })
    .bind((host, port))?
    .run()
    .await
}

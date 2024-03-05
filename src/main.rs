use actix_web::{web, App, HttpServer};
use sqlx::{self, migrate::MigrateDatabase, Sqlite, SqlitePool};

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

const DB_URL: &str = "sqlite://TNEA.db";

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        panic!("Database @{} does not exist", DB_URL);
    }
    
    let db = SqlitePool::connect(DB_URL).await.expect("Database Connection Failed");

    
    println!( "Listening at {}:{}", HOST, PORT);

    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(query)
        .service(static_files)
        .app_data(web::Data::new(db.to_owned()))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}

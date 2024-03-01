use actix_web::{middleware::Logger, web, App, HttpServer};

mod utils;
mod routes {
    pub mod index;
    pub mod query;
    pub mod static_files;
}

use utils::{Ranklist, init_db};
use routes::{ index::index, query::query, static_files::static_files };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT : u16 = 8080;

    let db : Ranklist = init_db().unwrap();
    
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .service(index)
        .service(query)
        .service(static_files)
        .app_data(web::Data::new(db.to_owned()))
    })
    .bind(("127.0.0.1", PORT))
    ?.run()
    .await
}
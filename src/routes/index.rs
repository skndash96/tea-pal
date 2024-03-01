use actix_web::{get, web, HttpResponse, Responder};
use web::Data;
use std::fs::read_to_string;

use crate::utils::Ranklist;

#[get("/")]
pub async fn index(db: Data<Ranklist>) -> impl Responder {
    if let Ok(html) = read_to_string("./src/views/index.html") {
        let body = html.replace("<!--data_length-->", db.len().to_string().as_str());
        
        return HttpResponse::Ok().body(
            body
        );
    }
    
    HttpResponse::InternalServerError().body("500: Something went wrong")
}
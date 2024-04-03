use std::fs::read_to_string;

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    println!("GET /");

    if let Ok(content) = read_to_string("./src/views/index.html") {
        return HttpResponse::Ok().content_type("text/html").body(content);
    } else {
        return HttpResponse::InternalServerError().body("File Not Found");
    }
}
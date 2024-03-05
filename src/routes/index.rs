use actix_web::{get, HttpResponse, Responder};
use std::fs::read_to_string;

#[get("/")]
pub async fn index() -> impl Responder {
    println!("GET at /");
    
    if let Ok(html) = read_to_string("./src/views/index.html") {
        let body = html.replace("<!--data_length-->", "14211");
        
        return HttpResponse::Ok().body(
            body
        );
    }
    
    HttpResponse::InternalServerError().body("500: Something went wrong")
}
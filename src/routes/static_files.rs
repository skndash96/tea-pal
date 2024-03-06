use actix_web::{self, Error};
use actix_web::{ get, HttpRequest };
use std::fs::read_to_string;

#[get("/{filename:.*}")]
pub async fn static_files(req: HttpRequest) -> actix_web::Result<String, Error> {
    let path = format!("./src/views/{}", req.match_info().query("filename"));
    let content = read_to_string(path)?;

    Ok(content)
}
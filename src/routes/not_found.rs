use actix_web::{http::StatusCode, HttpResponse, Result};

pub async fn not_found() -> Result<HttpResponse> {
    Ok(
        HttpResponse::build(StatusCode::NOT_FOUND).content_type("text/html; charset=utf-8").body("<html><body><a href='/'>  You are lost. Go HOME.  </a></body></html>")
    )
}
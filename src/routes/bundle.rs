use actix_web::{self, Result,};
use actix_web::{ get, HttpRequest };
use actix_files::NamedFile;

use std::path::PathBuf;

#[get("/{filename:_app/.*}")]
async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    println!("{:?}", req);
    let path = String::from("src/views/") + req.match_info().query("filename");
    println!("{}", path);
    
    let path: PathBuf = path.parse().unwrap();

    Ok(NamedFile::open(path)?)
}
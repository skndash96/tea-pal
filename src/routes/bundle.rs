use actix_web::{self, Result,};
use actix_web::{ get, HttpRequest };
use actix_files::NamedFile;

use std::path::PathBuf;

#[get("/{filename:_app/.*}")]
async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename");
    let path = String::from("views/") + filename;

    println!("GET /{}", path);
    
    let path: PathBuf = path.parse().unwrap();

    println!("{:?}", path);

    Ok(NamedFile::open(path)?)
}
use actix_files::NamedFile;
use actix_web::{self, Result};
use actix_web::{get, HttpRequest};

use std::path::PathBuf;

#[get("/{filename:.+}")]
pub async fn files(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename");
    let path =
        String::from("views/") + filename + (if filename.contains("/") { "" } else { ".html" });

    println!("GET /{}", path);

    let path: PathBuf = path.parse().unwrap();

    println!("{:?}", path);

    Ok(NamedFile::open(path)?)
}

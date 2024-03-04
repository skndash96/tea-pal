use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use web::{ Query, Data };
use sqlx::{self, sqlite::SqlitePool};
use crate::models::Ranking;

#[derive(Debug, Deserialize)]
struct Options {
    limit: Option<u32>,
    cutoff: Option<f32>,
    category: Option<String>
}

#[get("/query")]
pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
    let q = q.into_inner();

    println!("Get at /query: {:?}", q);
    
    if !q.cutoff.is_some() {
        return HttpResponse::BadRequest().body("Cutoff or Rank is required for query.");
    }

    //Order of selection does not matter.
    let fetch = sqlx::query_as::<_, Ranking>(
        format!(
        "SELECT
            name,
            category,
            cutoff,
            rank,
            college,
            (SELECT CONCAT_WS(', ', name, city) FROM College WHERE id = college) as college_name,
            CONCAT_WS(', ', branch, (SELECT name FROM Branch WHERE id = branch)) as branch_name
        FROM ranklist
        WHERE cutoff <= {} AND category = {}
        LIMIT {}",
        q.cutoff.unwrap(),
        if q.category.is_some() { format!("'{}'" , q.category.unwrap()) } else { String::from("category") },
        q.limit.unwrap_or(50)
    ).as_str())
    .fetch_all(&**db)
    .await;

    if let Ok(data) = fetch {
        HttpResponse::Ok().json(data)
    } else {
        println!("{:?}", fetch.err());
        HttpResponse::InternalServerError().body("500, Something went wrong")
    }
}

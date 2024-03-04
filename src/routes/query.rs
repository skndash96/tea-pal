use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use web::{ Query, Data };
use sqlx::{self, sqlite::SqlitePool};
use crate::models::Ranking;

#[derive(Debug, Deserialize)]
struct Options {
    limit: Option<u32>,
    cutoff: Option<f32>,
    rank: Option<u32>
}

#[get("/query")]
pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
    println!("Get at /query: {:?}", q);
    
    if !q.cutoff.is_some() && !q.rank.is_some() {
        return HttpResponse::BadRequest().body("Cutoff or Rank is required for query.");
    }

    //Order of selection does not matter.
    let fetch = sqlx::query_as::<_, Ranking>(
        "SELECT name,
            category,
            cutoff,
            rank,
            college,
            (SELECT (name || ', ' || city) FROM College WHERE id = college) as college_name,
            branch || ', ' || (SELECT name FROM Branch WHERE id = branch) as branch_name
        FROM ranklist
        WHERE cutoff <= $1 LIMIT $2",
    )
    .bind(q.cutoff)
    .bind(q.limit.unwrap_or(50))
    .fetch_all(&**db)
    .await;

    if let Ok(data) = fetch {
        HttpResponse::Ok().json(data)
    } else {
        println!("{:?}", fetch.err());
        HttpResponse::InternalServerError().body("500, Something went wrong")
    }
}

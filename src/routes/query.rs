use actix_web::{get, web, HttpResponse, Responder};
use web::{ Query, Data };

use crate::utils::{ Options, Ranklist };

#[get("/query")]
pub async fn query(q: Query<Options>, db: Data<Ranklist>) -> impl Responder {
    println!("Get at /query: {:?}", q);
    
    if !q.cutoff.is_some() && !q.rank.is_some() {
        return HttpResponse::BadRequest().body("Cutoff or Rank is required for query.");
    }

    let data = db.query(q.into_inner()).unwrap_or(&[]);
 
    HttpResponse::Ok().json(data)
}

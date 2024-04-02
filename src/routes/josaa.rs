use crate::models::JosaaItem;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{self, sqlite::SqlitePool};
use web::{Data, Query};

//changes
//fill '' (blank) gender to NA, for 3 records
//change gender 'F' to 'FemaleOnly(including Supernumerary), for 1 record
//change 2019 NIT Goa GO category to HS because it's discontinued since (probably), for 50 records

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Options {
    limit: Option<u32>,
    name: Option<String>,
    rank: (Option<u32>, Option<u32>),
    course: Option<String>,//100+: Mech, Civil,...
    quota: Option<String>,//HS, OS, AI (IITs and IIITs have not HS and OS but AI, All-India), GO (NIT Goa), JK (NIT Srinagar), LA (NIT Srinagar), AP (Andhra only for NIT Warangal)
    gender: Option<String>,//3only: Neutral, FemaleOnly(including Supernumerary), NA
    seat: Option<String>, //OPEN, EWS, OBC-NCL, SC, ST, PwD(subclasses of previous 5 seat types like OPEN(PwD)...)
}

#[get("/josaa")]
pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
    let json : Vec<u32> = vec![];
    HttpResponse::Ok().json(json)
}
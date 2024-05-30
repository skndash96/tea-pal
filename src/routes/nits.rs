// use serde::{Deserialize, Serialize};
// use sqlx::{self, prelude::FromRow, sqlite::SqlitePool};
// use actix_web::{get, web::{ Data, Query }, HttpResponse, Responder};

// // use crate::models::JosaaItem;

// #[derive(serde::Deserialize)]
// struct Options {
//     name: Option<String>
// }

// #[derive(FromRow, Serialize, Deserialize)]
// struct Name {
//     name: String
// }

// #[derive(FromRow, Serialize, Deserialize)]
// struct JosaaItem2 {
//     name: String,
//     course: String,
//     year: u32,
//     cr: u32
// }

// #[get("/api/nits")]
// pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
//     println!("GET /nits");

//     let query = q.into_inner();

//     let res : Result<HttpResponse, sqlx::Error> = if let Some(name) = query.name {
//         let q = format!(
//             "SELECT * FROM josaa WHERE name LIKE '%National Institute of Technology%' AND name LIKE '%{}%' AND quota = 'OS' AND seat = 'OPEN' AND round = 6 AND gender LIKE '%neutral%' --case-insensitive;",
//             name
//         );

//         sqlx::query_as::<_, JosaaItem2>(q.as_str())
//             .fetch_all(&**db)
//             .await
//             .map(|x| HttpResponse::Ok().json(x))
//     } else {
//         sqlx::query_as::<_, Name>("SELECT distinct name FROM josaa WHERE name LIKE '%National Institute of Technology%';")
//             .fetch_all(&**db)
//             .await
//             .map(|x| x.into_iter().map(|x| x.name).collect::<Vec<String>>())
//             .map(|x| HttpResponse::Ok().json(x))
//     };

//     match res {
//         Ok(content) => content,
//         Err(error) => HttpResponse::InternalServerError().body(format!("{}", error))
//     }
// }
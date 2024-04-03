use std::{collections::HashMap, fmt::Display};

use crate::models::JosaaItem;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{self, sqlite::SqlitePool};
use web::{Data, Query};

//changes
//fill '' (blank) gender to NA, for 3 records
//change gender 'F' to 'FemaleOnly(including Supernumerary), for 1 record
//change 2019 NIT Goa GO category to HS because it's discontinued since (probably), for 50 records
//change or = cr = 0 for about 4856 records because it had or = cr = nP (NaN) (preparatory list)

type Options = HashMap<String, String>;
enum CMP {
    GTE,
    LTE,
    EQ,
    LIKE,
}
impl Display for CMP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GTE => ">=",
            Self::LTE => "<=",
            Self::EQ => "=",
            Self::LIKE => "LIKE",
        };
        f.write_str(s)
    }
}

#[get("/josaa")]
pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
    let q = q.into_inner();

    let fields = HashMap::from([
        ("name", (CMP::LIKE, true)),
        ("course", (CMP::LIKE, true)),
        ("quota", (CMP::LIKE, true)),
        ("seat", (CMP::LIKE, true)),
        ("gender", (CMP::LIKE, true)),
        ("rank", (CMP::GTE, false)),
        ("year", (CMP::EQ, false)),
        ("round", (CMP::EQ, false)),
    ]);

    let limit = if let Some(l) = q.get("limit") {
        l.clone()
    } else {
        "500".to_string()
    };

    let mut query = "SELECT * FROM JOSAA WHERE year > 2000 ".to_string();

    for ((cmp, pad_apos), key, val) in q
        .into_iter()
        .map(|x| (fields.get(x.0.as_str()), x.0, x.1))
        .filter(|x| x.0.is_some())
        .map(|x| (x.0.unwrap(), x.1, x.2))
    {
        query += format!(
            "AND {} {cmp} {} ",
            if key == "rank" { String::from("\"or\"") } else { key },
            if *pad_apos { format!("'%{}%'", val) } else { val }
        )
        .as_str();
    }

    query += format!("LIMIT {};", limit).as_str();

    println!("{}", query);
    let fetch = sqlx::query_as::<_, JosaaItem>(query.as_str())
        .fetch_all(&**db)
        .await;

    if let Ok(data) = fetch {
        return HttpResponse::Ok().json(data);
    } else {
        return sql_err(fetch.err().unwrap());
    }
}

fn sql_err(e: sqlx::Error) -> HttpResponse {
    println!("SQL ERROR during Query: {:?}", e);

    HttpResponse::InternalServerError().body("500, Something went wrong")
}

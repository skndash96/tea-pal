use std::{collections::HashMap, fmt::Display, time::Instant};

use crate::models::JosaaItem;
use actix_web::{
    get,
    web,
    HttpResponse, Responder,
};
use sqlx::{self, sqlite::SqlitePool};
use web::{Data, Query};

//changes
//fill '' (blank) gender to NA, for 3 records
//change gender 'F' to 'FemaleOnly(including Supernumerary), for 1 record
//change 2019 NIT Goa GO category to HS because it's discontinued since (probably), for 50 records
//change or = cr = 0 for about 4856 records because it had or = cr = nP (NaN) (preparatory list)

type Options = HashMap<String, String>;

enum Chain {
    AND,
    OR,
}
impl Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::AND => "AND",
            Self::OR => "OR"
        };
        f.write_str(s)
    }
}

#[derive(Clone, Copy)]
enum CMP {
    GTE,
    EQ,
    LIKE,
}

impl Display for CMP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GTE => ">=",
            Self::EQ => "=",
            Self::LIKE => "LIKE",
        };
        f.write_str(s)
    }
}
impl CMP {
    fn enclose<'a>(&self) -> &'a str {
        match self {
            Self::LIKE => "'",
            _ => "",
        }
    }
}

#[get("/api/josaa")]
pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
    let q = q.into_inner();

    let mut query = String::from("SELECT * FROM josaa WHERE true");

    let fields = [
        //field compare multiple(sql OR)
        ("name", CMP::LIKE, true),
        ("course", CMP::LIKE, true),
        ("quota", CMP::LIKE, true),
        ("seat", CMP::LIKE, false),
        ("gender", CMP::LIKE, false),
        ("cr", CMP::GTE, false),
        ("year", CMP::EQ, true),
        ("round", CMP::EQ, true),
    ];

    for (f, cmp, multiple) in fields {
        if let Some(val) = q.get(f) {
            query += get_sql_str(Chain::AND, f, val, cmp, multiple).as_str();
        }
    }

    fn get_sql_str<'a>(
        chain: Chain,
        field: &'a str,
        val: &String,
        cmp: CMP,
        multiple: bool,
    ) -> String {
        let is_like = match cmp {
            CMP::LIKE => true,
            _ => false,
        };
        
        if multiple {
            let val = val.split(";");
            
            let mut s = String::from(" AND (false");
            
            for v in val {
                s += get_sql_str(Chain::OR, field, &v.to_string(), cmp, false)
                    .as_str();
            }
            s += ")";
            
            return s;
        } else {
            let enclose = cmp.enclose();

            let s = format!(
                " {} {field} {cmp} {}{val}{}",
                chain,
                if is_like { "'%" } else { enclose },
                if is_like { "%'" } else { enclose }
            );

            return s;
        }
    }

    let limit = if let Some(l) = q.get("limit") {
        l.clone()
    } else {
        "500".to_string()
    };

    let by_rank = q.get("cr").is_some();

    query += format!(
        "{} LIMIT {} --case-insensitive;",
        if by_rank { " ORDER BY cr DESC" } else { "" },
        limit
    )
    .as_str();

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

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

enum Chain {
    AND,
    OR,
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

#[get("/api/josaa")]
pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
    let q = q.into_inner();

    let mut query = String::from("SELECT * FROM josaa WHERE 1 = 1");

    query += get_sql_str(Chain::AND, "name", q.get("name"), CMP::LIKE, "'", true).as_str();
    query += get_sql_str(Chain::AND, "course", q.get("course"), CMP::LIKE, "'", true).as_str();
    query += get_sql_str(Chain::AND, "quota", q.get("quota"), CMP::LIKE, "'", true).as_str();
    query += get_sql_str(Chain::AND, "seat", q.get("seat"), CMP::LIKE, "'", false).as_str();
    query += get_sql_str(Chain::AND, "gender", q.get("gender"), CMP::LIKE, "'", false).as_str();
    query += get_sql_str(Chain::AND, "\"or\"", q.get("rank"), CMP::GTE, "", false).as_str();
    query += get_sql_str(Chain::AND, "year", q.get("year"), CMP::EQ, "", true).as_str();
    query += get_sql_str(Chain::AND, "round", q.get("round"), CMP::EQ, "", true).as_str();

    fn get_sql_str<'a>(
        chain: Chain,
        field: &'a str,
        val: Option<&String>,
        cmp: CMP,
        enclose: &'a str,
        many: bool,
    ) -> String {
        let is_like = match cmp {
            CMP::LIKE => true,
            _ => false,
        };

        if let Some(val) = val {
            if many {
                let val = val.split(";");
                
                let mut s = String::from(" AND (1=2");

                for v in val {
                    s += get_sql_str(Chain::OR, field, Some(&v.to_string()), cmp, enclose, false).as_str();
                }
                s += ")";

                return s;
            } else {
                let s = format!(
                    " {} {field} {cmp} {}{val}{}",
                    match chain {
                        Chain::AND => "AND",
                        Chain::OR => "OR",
                    },
                    if is_like { "'%" } else { enclose },
                    if is_like { "%'" } else { enclose }
                );

                return s;
            }
        } else {
            return String::new();
        }
    }

    let limit = if let Some(l) = q.get("limit") {
        l.clone()
    } else {
        "500".to_string()
    };
    
    let by_rank = q.get("rank").is_some();

    query += format!(
        "{} LIMIT {} --case-insensitive;",
        if by_rank { " ORDER BY \"or\" ASC" } else { "" },
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

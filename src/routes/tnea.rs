use crate::models::{College, Ranking};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{self, sqlite::SqlitePool};
use web::{Data, Query};

#[derive(Debug, Deserialize)]
struct Options {
    limit: Option<u32>,
    cutoff: Option<f32>,
    rank: Option<u32>,
    category: Option<String>,
    college: Option<String>,
    coll_code: Option<String>,
}

#[get("/api/tnea")]
pub async fn query(q: Query<Options>, db: Data<SqlitePool>) -> impl Responder {
    let q = q.into_inner();

    let limit = q.limit.unwrap_or(100);

    println!("Get at /query: {:?}", q);

    /*LIST MATCHING COLLEGES*/
    if let Some(name) = q.college {
        let fetch = sqlx::query_as::<_, College>(
            format!(
                "SELECT *
            FROM college
            WHERE
                name LIKE '%{name}%'
                OR city LIKE '%{name}%'
            LIMIT {limit}
            --case-insensitive"
            )
            .as_str(),
        )
        .fetch_all(&**db)
        .await;

        if let Ok(data) = fetch {
            return HttpResponse::Ok().json(data);
        }

        return sql_err(fetch.unwrap_err());
    }

    /*LIST CUTOFF OF GIVEN COLLEGE CODE*/
    if let Some(code) = q.coll_code {
        let fetch = sqlx::query_as::<_, Ranking>(
            format!(
                "SELECT
                    Ranklist.name,
                    category,
                    cutoff,
                    rank,
                    college,
                    CONCAT_WS(', ', College.name, College.city) as college_name,
                    CONCAT_WS(', ', branch, Branch.name) as branch_name
                FROM Ranklist, College, Branch
                WHERE
                    College.id={code}
                    AND Ranklist.college=College.id
                    AND Ranklist.branch=Branch.id
                GROUP BY Ranklist.branch
                LIMIT 100
                "
            )
            .as_str(),
        )
        .fetch_all(&**db)
        .await;

        if let Ok(data) = fetch {
            return HttpResponse::Ok().json(data);
        }

        return sql_err(fetch.unwrap_err());
    }

    /*LIST PEOPLE WITH NEAR CUTOFF/RANK*/
    let (p, pv) = {
        if let Some(v) = q.cutoff {
            (Some("cutoff"), v as u32)
        } else if let Some(v) = q.rank {
            (Some("rank"), v)
        } else {
            (None, 0)
        }
    };

    if let Some(param) = p {
        let fetch = sqlx::query_as::<_, Ranking>(
            format!(
                "SELECT
                Ranklist.name,
                category,
                cutoff,
                rank,
                college,
                CONCAT_WS(', ', College.name, College.city) as college_name,
                CONCAT_WS(', ', branch, Branch.name) as branch_name
            FROM Ranklist, College, Branch
            WHERE
                Ranklist.college=College.id
                AND Ranklist.branch=Branch.id
                AND {param} <= {pv}
                AND category = {}
            ORDER BY {param} DESC
            LIMIT {limit}",
                if q.category.is_some() {
                    format!("'{}'", q.category.unwrap())
                } else {
                    String::from("category")
                }
            )
            .as_str(),
        )
        .fetch_all(&**db)
        .await;

        if let Ok(data) = fetch {
            return HttpResponse::Ok().json(data);
        }

        return sql_err(fetch.unwrap_err());
    }

    return HttpResponse::BadRequest().body("Invalid Query: No cutoff or college parameter found.");
}

fn sql_err(e: sqlx::Error) -> HttpResponse {
    println!("SQL ERROR during Query: {:?}", e);

    HttpResponse::InternalServerError().body("500, Something went wrong")
}

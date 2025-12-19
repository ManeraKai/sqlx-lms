use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, http::StatusCode, post, web};
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::{
    routes::{GetRecordByIdPath, GetRecordQuery, RemoveRecordPath},
    schema::Book,
};

#[get("/api/book")]
async fn get(_req: HttpRequest, query: web::Query<GetRecordQuery>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let page = query.page.unwrap_or(0);
        let limit = query.limit.unwrap_or(1);
        let record = sqlx::query_as!(Book, "SELECT * FROM book LIMIT ? OFFSET ?", limit, page)
            .fetch_all(&*pool)
            .await;
        if let Ok(record) = record {
            return HttpResponse::Ok().body(json!(record).to_string());
        }
        return HttpResponse::Ok().body(json!(serde_json::Value::Null).to_string());
    }
    return HttpResponse::Ok().status(StatusCode::NOT_FOUND).body("");
}

// #[get("/api/book/{id}")]
// async fn get_by_id(_req: HttpRequest, path: web::Path<GetRecordByIdPath>) -> impl Responder {
//     let app_data = _req.app_data::<Pool<Sqlite>>();
//     if let Some(pool) = app_data {
//         let book = sqlx::query_as!(Book, "SELECT * FROM book WHERE book.id = ?", path.id)
//             .fetch_one(&*pool)
//             .await;
//         if let Ok(book) = book {
//             return HttpResponse::Ok().body(json!(book).to_string());
//         }
//         return HttpResponse::Ok().body(json!(serde_json::Value::Null).to_string());
//     }
//     return HttpResponse::Ok()
//         .status(StatusCode::INTERNAL_SERVER_ERROR)
//         .body("Error");
// }

#[derive(serde::Deserialize)]
struct PostRecordBook {
    name: String,
}

#[post("/api/book")]
pub async fn insert(_req: HttpRequest, body: web::Json<PostRecordBook>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let resp = sqlx::query!("INSERT INTO book (name) VALUES (?)", body.name)
            .execute(&*pool)
            .await;
        if let Err(err) = resp {
            return HttpResponse::Ok()
                .status(StatusCode::BAD_REQUEST)
                .body(err.to_string());
        }
        return HttpResponse::Ok().body("");
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

#[post("/api/book/{id}")]
pub async fn edit(
    _req: HttpRequest,
    path: web::Path<GetRecordByIdPath>,
    body: web::Json<PostRecordBook>,
) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let resp = sqlx::query!(
            "UPDATE book SET name = ? WHERE book.id = ?",
            body.name,
            path.id
        )
        .execute(&*pool)
        .await;
        if let Err(err) = resp {
            return HttpResponse::Ok()
                .status(StatusCode::BAD_REQUEST)
                .body(err.to_string());
        }
        return HttpResponse::Ok().body("");
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

#[delete("/api/book/{id}")]
pub async fn delete(_req: HttpRequest, path: web::Path<RemoveRecordPath>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let res = sqlx::query!("DELETE FROM book WHERE id = ?", path.id)
            .execute(&*pool)
            .await;
        match res {
            Ok(_) => return HttpResponse::Ok().body(""),
            Err(err) => {
                return HttpResponse::Ok()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string());
            }
        };
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

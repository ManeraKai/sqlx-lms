use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, http::StatusCode, post, web};
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::{routes::RemoveRecordPath, schema::Borrow};

#[get("/api/borrow")]
async fn get(_req: HttpRequest) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let books = sqlx::query_as!(Borrow, "SELECT * FROM borrow")
            .fetch_all(&*pool)
            .await
            .unwrap();
        return HttpResponse::Ok().body(json!(books).to_string());
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

#[derive(serde::Deserialize)]
struct PostRecordBorrow {
    pub book_id: String,
    pub customer_id: String,
    pub duration: String,
}

#[post("/api/borrow")]
pub async fn insert(_req: HttpRequest, body: web::Json<PostRecordBorrow>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        sqlx::query!(
            "INSERT INTO borrow (book_id, customer_id, duration) VALUES (?, ?, ?)",
            body.book_id,
            body.customer_id,
            body.duration
        )
        .execute(&*pool)
        .await
        .unwrap();
        return HttpResponse::Ok().body("");
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

#[delete("/api/borrow/{id}")]
pub async fn delete(_req: HttpRequest, path: web::Path<RemoveRecordPath>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        sqlx::query!("DELETE FROM borrow WHERE id = ?", path.id)
            .execute(&*pool)
            .await
            .unwrap();
        return HttpResponse::Ok().body("");
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

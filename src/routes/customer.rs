use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, http::StatusCode, post, web};
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::{routes::RemoveRecordPath, schema::Customer};

#[get("/api/customer")]
async fn get(_req: HttpRequest) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let books = sqlx::query_as!(Customer, "SELECT * FROM customer")
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
struct PostRecordCustomer {
    name: String,
    age: String,
    sex: String
}

#[post("/api/customer")]
pub async fn insert(_req: HttpRequest, body: web::Json<PostRecordCustomer>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        sqlx::query!(
            "INSERT INTO customer (name, age, sex) VALUES (?, ?, ?)",
            body.name,
            body.age,
            body.sex
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

#[delete("/api/customer/{id}")]
pub async fn delete(_req: HttpRequest, path: web::Path<RemoveRecordPath>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        sqlx::query!("DELETE FROM customer WHERE id = ?", path.id)
            .execute(&*pool)
            .await
            .unwrap();
        return HttpResponse::Ok().body("");
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

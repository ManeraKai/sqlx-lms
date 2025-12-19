use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, http::StatusCode, post, web};
use serde_json::json;
use sqlx::{Pool, Sqlite};

use crate::{
    routes::{GetRecordByIdPath, GetRecordQuery, RemoveRecordPath},
    schema::Customer,
};

#[get("/api/customer")]
async fn get(_req: HttpRequest, query: web::Query<GetRecordQuery>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let page = query.page.unwrap_or(0);
        let limit = query.limit.unwrap_or(1);
        let record = sqlx::query_as!(
            Customer,
            "SELECT * FROM customer LIMIT ? OFFSET ?",
            limit,
            page
        )
        .fetch_all(&*pool)
        .await;
        if let Ok(record) = record {
            return HttpResponse::Ok().body(json!(record).to_string());
        }
        return HttpResponse::Ok().body(json!(serde_json::Value::Null).to_string());
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

#[derive(serde::Deserialize)]
struct PostRecordCustomer {
    name: String,
    age: String,
    sex: String,
    crimes: String,
}

#[post("/api/customer")]
pub async fn insert(_req: HttpRequest, body: web::Json<PostRecordCustomer>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let response = sqlx::query!(
            "INSERT INTO customer (name, age, sex, crimes) VALUES (?, ?, ?, ?)",
            body.name,
            body.age,
            body.sex,
            body.crimes,
        )
        .execute(&*pool)
        .await;
        if let Err(err) = response {
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

#[post("/api/customer/{id}")]
pub async fn edit(
    _req: HttpRequest,
    path: web::Path<GetRecordByIdPath>,
    body: web::Json<PostRecordCustomer>,
) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let response = sqlx::query!(
            "UPDATE customer SET name = ?, age = ?, sex = ?, crimes = ? WHERE id = ?",
            body.name,
            body.age,
            body.sex,
            body.crimes,
            path.id
        )
        .execute(&*pool)
        .await;
        if let Err(err) = response {
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

#[delete("/api/customer/{id}")]
pub async fn delete(_req: HttpRequest, path: web::Path<RemoveRecordPath>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        let resp = sqlx::query!("DELETE FROM customer WHERE id = ?", path.id)
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

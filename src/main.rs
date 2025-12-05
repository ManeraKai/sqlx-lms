mod schema;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, delete, get, http::StatusCode, post, web,
};
use serde_json::json;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

use crate::schema::{Book, Borrow, Customer};

#[derive(serde::Deserialize)]
struct TableInfo {
    name: String,
}

#[get("/api/{name}")]
async fn get_books(_req: HttpRequest, info: web::Path<TableInfo>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        match info.name.as_str() {
            "customer" => {
                let customer = sqlx::query_as!(Customer, "SELECT * FROM customer")
                    .fetch_all(&*pool)
                    .await
                    .unwrap();
                return HttpResponse::Ok().body(json!(customer).to_string());
            }
            "borrow" => {
                let borrow = sqlx::query_as!(Borrow, "SELECT * FROM borrow")
                    .fetch_all(&*pool)
                    .await
                    .unwrap();
                return HttpResponse::Ok().body(json!(borrow).to_string());
            }
            _ => {
                let books = sqlx::query_as!(Book, "SELECT * FROM book")
                    .fetch_all(&*pool)
                    .await
                    .unwrap();
                return HttpResponse::Ok().body(json!(books).to_string());
            }
        }
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

#[derive(serde::Deserialize)]
enum PostQueryValues {
    Customer(PostRecordsCustomer),
    Borrow(PostRecordBorrow),
    Book(PostRecordBook),
}

#[derive(serde::Deserialize)]
struct PostRecordsCustomer {
    name: String,
    age: i64,
    gender: i64,
    crimes: i64,
}

#[derive(serde::Deserialize)]
struct PostRecordBorrow {
    book_id: i64,
    customer_id: i64,
    duration: i64,
}

#[derive(serde::Deserialize)]
struct PostRecordBook {
    name: String,
}

#[post("/api/book")]
async fn insert_book(_req: HttpRequest, body: web::Json<PostRecordBook>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        sqlx::query!("INSERT INTO book (name) VALUES (?)", body.name)
            .execute(&*pool)
            .await
            .unwrap();
        return HttpResponse::Ok().body("");
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

#[derive(serde::Deserialize)]
struct RemoveBookPath {
    id: i64,
}

#[delete("/api/book/{id}")]
async fn delete_book(_req: HttpRequest, path: web::Path<RemoveBookPath>) -> impl Responder {
    let app_data = _req.app_data::<Pool<Sqlite>>();
    if let Some(pool) = app_data {
        sqlx::query!("DELETE FROM book WHERE id = ?", path.id)
            .execute(&*pool)
            .await
            .unwrap();
        return HttpResponse::Ok().body("");
    }
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Error");
}

// "customer" => match &query.values {
//     PostQueryValues::Customer(record) => {
//         sqlx::query!(
//             "INSERT INTO customer (name, age, gender, crimes) VALUES (?, ?, ?, ?)",
//             record.name,
//             record.age,
//             record.gender,
//             record.crimes
//         )
//         .execute(&*pool)
//         .await
//         .unwrap();
//         return HttpResponse::Ok().body("");
//     }
//     _ => {
//         return HttpResponse::Ok()
//             .status(StatusCode::INTERNAL_SERVER_ERROR)
//             .body("Error");
//     }
// },
// "borrow" => match &query.values {
//     PostQueryValues::Borrow(record) => {
//         sqlx::query!(
//             "INSERT INTO borrow (book_id, customer_id, duration) VALUES (?, ?, ?)",
//             record.book_id,
//             record.customer_id,
//             record.duration
//         )
//         .execute(&*pool)
//         .await
//         .unwrap();
//         return HttpResponse::Ok().body("");
//     }
//     _ => {
//         return HttpResponse::Ok()
//             .status(StatusCode::INTERNAL_SERVER_ERROR)
//             .body("Error");
//     }
// },

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://data.db")
        .await
        .unwrap();

    schema::init_db(&mut pool).await.unwrap();

    println!("http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(get_books)
            .service(insert_book)
            .service(delete_book)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

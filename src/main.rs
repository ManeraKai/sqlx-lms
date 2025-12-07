pub mod routes;
mod schema;

use actix_web::{App, HttpServer};

use sqlx::sqlite::SqlitePoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://data.db")
        .await
        .unwrap();

    println!("http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(routes::book::get)
            .service(routes::book::insert)
            .service(routes::book::delete)
            .service(routes::customer::get)
            .service(routes::customer::insert)
            .service(routes::customer::delete)
            .service(routes::borrow::get)
            .service(routes::borrow::insert)
            .service(routes::borrow::delete)
            .service(routes::borrow::advance_in_time)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

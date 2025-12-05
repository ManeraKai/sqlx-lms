use core::fmt;

use sqlx::{Pool, Sqlite, prelude::FromRow};

#[derive(Debug, FromRow, serde::Serialize)]
pub struct Book {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow, serde::Serialize)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub gender: i64,
    pub crimes: i64,
}

#[derive(Debug, FromRow, serde::Serialize)]
pub struct Borrow {
    pub id: i64,
    pub book_id: i64,
    pub customer_id: i64,
    pub duration: i64,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Book(id: {}, name: {})", self.id, self.name)
    }
}
impl fmt::Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Customer(id: {}, name: {}, age: {}, gender: {}, crimes: {})",
            self.id, self.name, self.age, self.gender, self.crimes
        )
    }
}

impl fmt::Display for Borrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Customer(id: {}, book_id: {}, customer_id: {}, duration: {})",
            self.id, self.book_id, self.customer_id, self.duration,
        )
    }
}

pub async fn init_db(pool: &mut Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS book(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS customer(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                age INTEGER NOT NULL,
                gender INTEGER NOT NULL,
                crimes INTEGER NOT NULL DEFAULT 0 
            );
            CREATE TABLE IF NOT EXISTS borrow(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                book_id INTEGER NOT NULL,
                customer_id INTEGER NOT NULL,
                duration INTEGER NOT NULL,
                FOREIGN KEY(book_id) REFERENCES book(id),
                FOREIGN KEY(customer_id) REFERENCES customer(id)
            );
            ",
    )
    .execute(&*pool)
    .await?;
    println!("Initialized database");
    Ok(())
}

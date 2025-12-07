use core::fmt;

use sqlx::{prelude::FromRow};

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
    pub sex: i64,
    pub crimes: i64,
}

#[derive(Debug, FromRow, serde::Serialize)]
pub struct Borrow {
    pub id: i64,
    pub book_id: i64,
    pub customer_id: i64,
    pub duration: i64,
}

#[derive(Debug, FromRow, serde::Serialize)]
pub struct BorrowJoined {
    pub id: i64,
    pub book_name: String,
    pub customer_name: String,
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
            "Customer(id: {}, name: {}, age: {}, sex: {}, crimes: {})",
            self.id, self.name, self.age, self.sex, self.crimes
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

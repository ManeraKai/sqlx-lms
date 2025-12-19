pub mod book;
pub mod borrow;
pub mod customer;
pub mod raw_sql;

#[derive(serde::Deserialize)]
struct RemoveRecordPath {
    id: i64,
}

#[derive(serde::Deserialize)]
struct GetRecordQuery {
    page: Option<i64>,
    limit: Option<i64>,
}

#[derive(serde::Deserialize)]
struct GetRecordByIdPath {
    id: i64,
}

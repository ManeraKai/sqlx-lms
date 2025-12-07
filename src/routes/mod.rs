pub mod book;
pub mod borrow;
pub mod customer;

#[derive(serde::Deserialize)]
struct RemoveRecordPath {
    id: i64,
}
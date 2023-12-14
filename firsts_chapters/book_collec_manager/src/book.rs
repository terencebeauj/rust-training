use crate::status::Status;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub status: Status,
}
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    id: String,
    document_title: String,
    document_size_in_bytes: i32,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Ready,
    Processing,
    Done,
    Failed,
}

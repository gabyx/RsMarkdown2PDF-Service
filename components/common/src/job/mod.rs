use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Job {
    pub id: Uuid,
    pub document_title: String,
}

impl Job {
    pub fn new(title: &str) -> Self {
        return Job {
            id: uuid::Uuid::new_v4(),
            document_title: title.to_owned(),
        };
    }
}

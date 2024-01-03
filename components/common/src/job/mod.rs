use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct JobBundle {
    pub id: Uuid,

    pub name: String,
    pub digest: String,
}

impl JobBundle {
    pub fn new(name: &str, digest: &str) -> Self {
        return JobBundle {
            id: uuid::Uuid::new_v4(),
            name: name.to_owned(),
            digest: digest.to_owned(),
        };
    }
}

use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct JobBundle {
    /// The name of the job.
    pub name: String,

    /// Job id.
    pub id: Uuid,

    /// The blob id corresponding to this job.
    pub blob_digest: String,
    pub blob_content_type: String,
}

impl JobBundle {
    pub fn new(name: &str, blob_digest: &str, blob_content_type: &str) -> Self {
        return JobBundle {
            id: uuid::Uuid::new_v4(),
            name: name.to_owned(),
            blob_digest: blob_digest.to_owned(),
            blob_content_type: blob_content_type.to_owned(),
        };
    }
}

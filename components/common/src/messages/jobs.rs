/// Messages sent on the jobs queue.
///
use rocket::serde::Serialize;
use uuid::Uuid;

use crate::job::JobBundle;

#[derive(Debug, Serialize)]
pub struct JobMessage {
    /// The name of the job.
    pub name: String,

    /// Job id.
    pub id: Uuid,

    /// The blob id corresponding to this job.
    pub blob_digest: String,
    pub blob_content_type: String,
}

impl JobMessage {
    pub fn new(name: String, blob_digest: String, blob_content_type: String) -> Self {
        return JobMessage {
            id: uuid::Uuid::new_v4(),
            name,
            blob_digest,
            blob_content_type,
        };
    }
}

impl From<JobBundle> for JobMessage {
    fn from(value: JobBundle) -> Self {
        return JobMessage::new(value.name, value.blob_digest, value.blob_content_type);
    }
}

impl From<&JobBundle> for JobMessage {
    fn from(value: &JobBundle) -> Self {
        return JobMessage::new(
            value.name.to_owned(),
            value.blob_digest.to_owned(),
            value.blob_content_type.to_owned(),
        );
    }
}

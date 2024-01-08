use rocket::{
    fs::TempFile,
    serde::{json::Json, Deserialize, Serialize},
    FromForm,
};

use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct SubmittedJob {
    pub id: Uuid,
    pub digest: String,
}

#[derive(FromForm, Deserialize, Debug)]
pub struct JobMetaData {
    pub name: String,
}

#[derive(FromForm, Debug)]
pub struct JobUpload<'r> {
    pub metadata: Json<JobMetaData>,
    pub file: TempFile<'r>,
}

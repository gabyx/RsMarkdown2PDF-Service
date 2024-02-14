#![allow(clippy::blocks_in_conditions)]

use rocket::{
    fs::TempFile,
    serde::{json::Json, Deserialize, Serialize},
    FromForm,
};

use uuid::Uuid;

#[allow(renamed_and_removed_lints)]
#[derive(Debug, Serialize)]
pub struct SubmittedJob {
    pub id: Uuid,
    pub digest: String,
}

#[allow(renamed_and_removed_lints)]
#[derive(FromForm, Deserialize, Debug)]
pub struct JobMetaData {
    pub name: String,
}

#[derive(FromForm, Debug)]
pub struct JobUpload<'r> {
    pub metadata: Json<JobMetaData>,
    pub file: TempFile<'r>,
}

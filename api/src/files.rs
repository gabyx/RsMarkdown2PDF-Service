use rocket::fs::TempFile;
use std::fs::File;
use std::io::Error;

use uuid::Uuid;

#[derive(Debug)]
pub struct JobBundle {
    pub id: Uuid,
    pub name: String,
    pub stylesheet: String,
}

pub fn create_job_bundle(
    file: &mut TempFile,
    name: String,
    stylesheet: String,
) -> Result<JobBundle, Error> {
    let job_id = Uuid::new_v4();
    let file = persist_file(file, job_id);

    Ok(JobBundle {
        id: job_id,
        name,
        stylesheet,
    })
}

async fn persist_file(file: &mut TempFile<'_>, id: Uuid) -> Result<(), Error> {
    let path = format!("/tmp/pdf-jobs/{}", id);
    file.persist_to(path).await
}

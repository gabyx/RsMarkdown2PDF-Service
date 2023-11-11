use std::env;
use rocket::fs::TempFile;
use std::fs::File;
use std::fs;
use std::io::Error;

use uuid::Uuid;

#[derive(Debug)]
pub struct JobBundle {
    pub id: Uuid,
    pub name: String,
    pub stylesheet: String,
}

pub async fn create_job_bundle(
    file: &mut TempFile<'_>,
    name: String,
    stylesheet: String,
) -> Result<JobBundle, Error> {
    let job_id = Uuid::new_v4();
    let file = persist_file(file, job_id).await;

    Ok(JobBundle {
        id: job_id,
        name,
        stylesheet,
    })
}

async fn persist_file(file: &mut TempFile<'_>, id: Uuid) -> Result<(), Error> {
    let path_prefix = env::var("BLOB_STORAGE_PATH").unwrap_or(String::from("/tmp/markdown-to-pdf"));
    let bundle_prefix = format!("{path_prefix}/{}", id);
    fs::create_dir_all(&bundle_prefix).expect("Could not create bundle directory");

    info!("persisting file to: {}", bundle_prefix);
    let bundle_path = format!("{}/document.md", bundle_prefix);
    file.persist_to(&bundle_path).await
}

use common::{
    job::JobBundle,
    log::{self, info},
    response,
    response::Error,
    storage::BlobStorage,
};
use rocket::{fs::TempFile, http::Status};
use std::{env, path::Path, sync::Arc};
use uuid::Uuid;

pub async fn create_job_bundle(
    log: &log::Logger,
    file: &mut TempFile<'_>,
    name: &str,
    storage: Arc<dyn BlobStorage>,
) -> Result<JobBundle, response::Error> {
    let tmp_file = Path::join(&env::temp_dir(), Uuid::new_v4().to_string());

    info!(log, "Persist upload to temporary file '{:?}'.", tmp_file);
    file.persist_to(&tmp_file).await?;

    let content_type = match file.content_type() {
        Some(c) => c.to_string(),
        None => {
            return Err(response::error!(
                Status::BadRequest,
                "No content type given.",
            ))
        }
    };

    match content_type.as_str() {
        "text/markdown" => (),
        "application/x-zip" | "application/x-tar" => {
            return Err(response::error!(
                Status::BadRequest,
                "Content type '{}' files are not yet supported.",
                content_type
            ));
        }
        _ => {
            return Err(response::error!(
                Status::BadRequest,
                "Content type '{}' files are not supported.",
                content_type
            ));
        }
    };

    info!(log, "Store upload in storage.");
    let (_, digest) = storage.store_blob(&log, &tmp_file, &content_type).await?;

    return Ok(JobBundle::new(&name, &digest));
}

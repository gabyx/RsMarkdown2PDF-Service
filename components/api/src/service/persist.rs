use common::{
    job::JobBundle,
    log::{self, info},
    response,
    response::Status,
    result::{self, ResultExt},
    storage::BlobStorage,
};
use rocket::fs::TempFile;
use snafu::ResultExt as _;
use std::sync::Arc;

pub async fn create_job_bundle(
    log: &log::Logger,
    file: &mut TempFile<'_>,
    name: &str,
    storage: Arc<dyn BlobStorage>,
) -> Result<JobBundle, response::Error> {
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
        "text/markdown" | "application/gzip" => (),
        _ => {
            return Err(response::error!(
                Status::BadRequest,
                "Content type '{}' files are not supported (only 'text/markdown' and 'application/gzip').",
                content_type
            ));
        }
    };

    let path = storage.pre_store();
    info!(log, "Persist upload to temporary file '{:?}'.", path.path());

    file.copy_to(path.path())
        .await
        .log(log)
        .context(result::IOErrorCtx)?;

    let digest = storage
        .store(&log, path.finalize())
        .await
        .log(log)
        .context(result::IOErrorCtx)?;

    return Ok(JobBundle::new(&name, &digest, &content_type));
}

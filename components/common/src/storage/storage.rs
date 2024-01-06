use rocket;
use std::{io, path::Path};

use crate::log::Logger;

pub type Digest = String;

/// Simple interface which provides access to blob storage
/// in different components of the application.
/// Must be thread-safe.
#[rocket::async_trait]
pub trait BlobStorage: Sync + Send {
    /// Stores a file in a folder named by the files SHA256 digest.
    /// TODO: If the file is an archive it will be expanded into the folder.
    async fn store_blob(
        &self,
        log: &Logger,
        src: &Path,
        content_type: &str,
    ) -> Result<(String, Digest), io::Error>;

    /// Get the URL of blob with SHA256 `digest`.
    fn get_blob(&self, digest: &str) -> Result<String, io::Error>;
}

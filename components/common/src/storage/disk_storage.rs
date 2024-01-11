use crate::{
    log::{info, Logger},
    storage::{digest::get_digest, BlobStorage, Digest},
};
use rocket::{
    self, tokio,
    tokio::{fs::File, io::BufReader, sync},
};
use std::{
    io,
    path::{Path, PathBuf},
};

/// Simple disk storage.
/// Sharing a persistent volume between api and converter is
/// not really safe, but its a simple solution.
pub struct DiskStorage {
    pub path: PathBuf,

    lock: sync::Mutex<()>,
}

impl DiskStorage {
    pub fn new(path: &str) -> DiskStorage {
        return DiskStorage {
            path: PathBuf::from(&path),
            lock: sync::Mutex::new(()),
        };
    }

    fn get_blob_path(
        &self,
        sha256: &str,
    ) -> PathBuf {
        return Path::join(&self.path, sha256);
    }
}

#[rocket::async_trait]
impl BlobStorage for DiskStorage {
    async fn store_blob(
        &self,
        log: &Logger,
        src: &Path,
        content_type: &str,
    ) -> Result<(String, Digest), io::Error> {
        let f = File::open(src).await?;
        let buf = BufReader::new(f);

        info!(log, "Compute digest ...");
        let digest = get_digest(buf).await?;
        let dest = self.get_blob_path(&digest);

        if content_type == "application/x-zip" || content_type == "application/x-tar" {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!(
                    "Content type '{}' files are not yet supported.",
                    content_type
                ),
            ));
        }

        if !dest.exists() {
            let _l = self.lock.lock();

            info!(log, "Store into storage from {:?} -> {:?}.", src, dest);

            tokio::fs::create_dir_all(&dest).await?;
            tokio::fs::rename(src, &dest).await?;
        }

        return Ok((dest.to_string_lossy().to_string(), digest));
    }

    fn get_blob(
        &self,
        digest: &str,
    ) -> Result<String, io::Error> {
        let p = self.get_blob_path(digest);

        return match p.exists() {
            true => Ok(p.to_string_lossy().to_string()),
            false => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Blob {} not found.", digest),
            )),
        };
    }
}

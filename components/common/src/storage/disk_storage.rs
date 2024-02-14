use crate::{
    log::{info, Logger},
    storage::{digest::get_digest, BlobStorage, Digest},
};
use rocket::{
    self,
    tokio::{self, fs::File, io::BufReader, runtime, sync, task},
};
use scopeguard;
use std::{
    io,
    path::{Path, PathBuf},
};

/// Simple disk storage.
/// Sharing a persistent volume between api and converter is
/// not really safe, but its a simple solution.
use super::{ExistingFilePath, NonExistingFilePath};

pub struct DiskStorage {
    pub path: PathBuf,

    lock: sync::Mutex<()>,
}

impl DiskStorage {
    pub fn new(path: &str) -> DiskStorage {
        return DiskStorage {
            lock: sync::Mutex::new(()),
            path: PathBuf::from(&path),
        };
    }

    /// Gets the blobs storage path (directory).
    fn get_blob_path(&self, sha256: &str) -> PathBuf {
        return Path::join(&self.path, sha256);
    }
}

fn delete_path(path: &ExistingFilePath) {
    task::block_in_place(|| {
        runtime::Handle::current()
            .block_on(async { path.delete().await })
            .expect("File should have been deleted.");
    });
}

#[rocket::async_trait]
impl BlobStorage for DiskStorage {
    fn pre_store(&self) -> NonExistingFilePath {
        return NonExistingFilePath::new(self.get_blob_path(&uuid::Uuid::new_v4().to_string()));
    }

    async fn store(&self, log: &Logger, path: ExistingFilePath) -> Result<String, io::Error> {
        scopeguard::defer!(delete_path(&path));

        let _l = self.lock.lock().await;

        let digest = {
            let f = File::open(&path).await?;
            let buf = BufReader::new(f);

            info!(log, "Compute digest ...");
            get_digest(buf).await?
        };

        let dest = self.get_blob_path(&digest);

        if !dest.exists() {
            info!(log, "Store into storage from {:?} -> {:?}.", path, dest);
            tokio::fs::copy(&path, &dest).await?;
        } else {
            info!(log, "Blob with digest '{}' already exists.", digest);
        }

        return Ok(digest);
    }

    async fn delete(&self, log: &Logger, digest: Digest) -> Result<bool, io::Error> {
        info!(log, "Deleting blob with digest '{}'.", digest);
        let p = self.get_blob_path(&digest);

        let _l = self.lock.lock().await;
        if !p.exists() {
            return Ok(false);
        }

        tokio::fs::remove_file(p).await?;
        return Ok(true);
    }

    async fn get_url(&self, digest: &str) -> Option<String> {
        let p = self.get_blob_path(digest);

        if !p.exists() {
            return None;
        }

        return Some(format!(
            "file://{}",
            p.join("file").to_str().expect("Should be valid path.")
        ));
    }
}

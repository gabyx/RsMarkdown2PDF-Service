use rocket::tokio;
use std::{
    io,
    path::{Path, PathBuf},
};

use crate::log::Logger;

/// Simple interface which provides access to blob storage
/// in different components of the application.
/// The storage must be an interface (just out of good architecture reason)
/// and should not depend on types from rocket.
///
/// - DiskStorage: Implementation which stores the blobs on to a local disk path.
/// - S3 Storage or Bucket: Implementation which stores
///
/// I wanted to use a write-functor (Writer trait) to hand over to `store_blob`
/// to efficiently store the blob at a specific location **directly**.
/// Without that we would receive a `path` to an already stored file
/// which we need to copy to our storage
/// because moving is not possible over logical device boundaries.
/// That makes 2 copies which is inefficient.
///
/// However it turnes out that accepting a function (async) in [`BlobStorage::store`] is really tricky
/// and with current Rust not impossible, but it boils down to lots of `Box`ed values:
/// the writer function is type-erased `Box<dyn WriterFn<'a>>` and allocated on the heap thats ok,
/// plus the only way to work with `async` traits is with `#[rocket:async_trait]` so far which
/// makes every future `Pin<Box<..>>`ed as well. The topic is covered here:
///
/// - Knowledge: https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hardhttps://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard
///
/// - Solution with generic BlobStorage<W>: https://users.rust-lang.org/t/passing-fn-closure-to-async-trait-method/105248?u=gabyx
///     - Allocates all futures on the heap.
///
/// - Solution with functor is very ugly: https://users.rust-lang.org/t/passing-fn-closure-to-async-trait-method/105248/5?u=gabyx
///     - Allocates the functor and all futures on the heap. Lot of lifetime hacking.
///
/// To resolve these uglyness we make the inteface such that a temporary path with `get_store_path`
/// is returned which we can then store the blob into and then call [`BlobStorage::store`]}
/// We make it such that we request a special type in `store`
/// to be a little type safer.
///
/// Must be thread-safe.

pub type Digest = String;

#[rocket::async_trait]
pub trait BlobStorage: Sync + Send {
    /// Get a store path in which to write the file.
    /// Call `finalize()` on the result to hand it over to `store`.
    /// This path is guaranteed to work with `store` which will do the rest.
    fn pre_store(&self) -> NonExistingFilePath;

    /// Stores a file in a folder named by the file's SHA256 digest
    /// and returns the digest.
    /// The `file` is taken as moved `ExistingFilePath`, and `delete()` should
    /// be called if appropriate (if its not moved away).
    ///
    /// Note: `ExistingFilePath` has
    /// Drop implemented which will delete the file if not done already
    /// (unfortunately this is then
    /// not async, because `async drop` is not availbale yet.
    async fn store(&self, log: &Logger, file: ExistingFilePath) -> Result<Digest, io::Error>;

    /// Deletes the blob with digest `digest` and returns if it got deleted or
    /// was not existing.
    async fn delete(&self, log: &Logger, digest: Digest) -> Result<bool, io::Error>;

    /// Get the URL of blob with SHA256 `digest`.
    async fn get_url(&self, digest: &str) -> Option<String>;
}

#[derive(Debug)]
pub struct ExistingFilePath {
    path: PathBuf,
}

impl ExistingFilePath {
    pub fn path(&self) -> &Path {
        return &self.path;
    }

    pub(super) async fn delete(&self) -> Result<(), io::Error> {
        tokio::fs::remove_file(&self.path()).await
    }
}

impl AsRef<std::path::Path> for ExistingFilePath {
    fn as_ref(&self) -> &std::path::Path {
        return self.path();
    }
}

impl Drop for ExistingFilePath {
    fn drop(&mut self) {
        // If the path is still existing, remove it.
        if self.path.exists() {
            std::fs::remove_file(&self.path)
                .unwrap_or_else(|_| panic!("Could not delete file '{:?}'", &self.path));
        }
    }
}

#[derive(Debug)]
pub struct NonExistingFilePath {
    path: PathBuf,
}

impl NonExistingFilePath {
    // Only constructible in this module.
    pub(super) fn new(path: impl AsRef<Path>) -> NonExistingFilePath {
        return NonExistingFilePath {
            path: path.as_ref().to_path_buf(),
        };
    }

    pub fn path(&self) -> &Path {
        return &self.path;
    }

    /// Call this function when you have created this path.
    pub fn finalize(self) -> ExistingFilePath {
        // Move ourself and return a new thing.
        return ExistingFilePath { path: self.path };
    }
}

impl AsRef<std::path::Path> for NonExistingFilePath {
    fn as_ref(&self) -> &std::path::Path {
        return self.path();
    }
}

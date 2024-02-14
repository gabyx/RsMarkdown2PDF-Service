use std::{env, sync::Arc};

pub mod blob_storage;
pub use blob_storage::*;

pub mod disk_storage;
pub use disk_storage::*;

pub mod digest;

pub fn get_storage() -> Arc<dyn BlobStorage> {
    let path = env::var("BLOB_STORAGE_PATH").expect("Could not get blob storage path from env.");
    return Arc::new(DiskStorage::new(&path));
}

use common::{log::Logger, queue::JobQueue, storage::BlobStorage};
use diesel::pg::PgConnection;
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

pub struct AppState {
    // TODO: Abstract away the log. Make a simple interface in `common`.
    pub log: Arc<Logger>,

    // TODO: Abstract away db connection, if possible: Make an interface in `common`
    // such that only converter/api use the same interface and dont need to know if its postgres or
    // something else.
    pub db: Mutex<PgConnection>,

    pub job_queue: JobQueue,

    pub storage: Arc<dyn BlobStorage>,
}

impl AppState {
    pub fn new(
        log: Arc<Logger>,
        db_conn: PgConnection,
        job_queue: JobQueue,
        storage: Arc<dyn BlobStorage>,
    ) -> AppState {
        return AppState {
            log: log.clone(),
            db: Mutex::new(db_conn),
            job_queue,
            storage: storage.clone(),
        };
    }
}

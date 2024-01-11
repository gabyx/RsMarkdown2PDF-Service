use std::sync::Arc;

use common::{
    log::{create_logger, info, Logger},
    queue::{get_job_queue_config, setup_job_queue, JobQueue},
    storage::get_storage,
};
use converter::consumer::DefaultConsumer;
use dotenv::dotenv;
use tokio::sync::Notify;

async fn install_consumer(
    log: &Arc<Logger>,
    job_queue: &JobQueue,
) {
    info!(log, "Installing consumer on the queue");

    job_queue
        .subscribe(|args| DefaultConsumer::new(log.clone(), args.no_ack))
        .await
        .expect("Could not install consumer.");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let log = create_logger();
    info!(log, "Configuring 'converter' service.");

    info!(log, "Loading environment variables.");
    dotenv().ok();

    info!(log, "Initialize blob storage.");
    let _storage = get_storage();

    let (creds, queue_config) = get_job_queue_config();

    let job_queue = setup_job_queue(&log, creds, queue_config).await;
    install_consumer(&log, &job_queue).await;

    info!(log, "Consume from queue '{}'...", &job_queue.config.name);
    let guard = Notify::new();
    guard.notified().await;
}

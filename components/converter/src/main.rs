mod consumer;
use std::sync::Arc;

use common::{
    log::{create_logger, info, Logger},
    queue::{get_job_queue_config, setup_queues, JobQueue, StatusQueue},
    storage::get_storage,
};

use dotenv::dotenv;
use tokio::sync::Notify;

async fn install_consumer(log: &Arc<Logger>, job_queue: &JobQueue, status_queue: StatusQueue) {
    info!(log, "Installing consumer on the jobs queue.");

    job_queue
        .subscribe(|args| consumer::DefaultConsumer::new(log.clone(), status_queue, args.no_ack))
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

    let (job_queue, _status_queue) = setup_queues(&log, creds, queue_config).await;

    // for i in 0..10 {
    //     _status_queue
    //         .publish_completion(&status::create_log(
    //             Uuid::new_v4(),
    //             "WTF",
    //             JobLogLevel::Info,
    //         ))
    //         .await;
    // }

    install_consumer(&log, &job_queue, _status_queue).await;

    info!(log, "Consume from queue '{}'...", &job_queue.config.name);
    let guard = Notify::new();
    guard.notified().await;
}

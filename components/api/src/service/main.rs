// Include all modules.
mod consumer;
mod database;
mod handlers;
mod messages;
mod persist;
mod state;

use std::sync::Arc;

use common::{
    config::get_env_var,
    log::{create_logger, info, Logger},
    queue::{get_job_queue_config, setup_queues, JobQueue},
    storage::{get_storage, BlobStorage},
};

use handlers::install_handlers;
use state::AppState;

use dotenvy::dotenv;
use rocket::{
    config::{Config, LogLevel},
    tokio::sync::{oneshot, Mutex},
    Ignite, Rocket,
};

async fn create_rocket(
    log: Arc<Logger>,
    database_url: &str,
    job_queue: JobQueue,
    storage: Arc<dyn BlobStorage>,
) -> Result<(Rocket<Ignite>, consumer::WaitForShutdown), rocket::Error> {
    let db_conn = database::connect(&log, &database_url);
    let app_state = AppState::new(log.clone(), db_conn, job_queue, storage);

    info!(log, "Start rocket.");

    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    // Create a channel
    let (tx_consumer_done, rx_consumer_done) = oneshot::channel();
    let wait_for_status_consumer = consumer::WaitForConsumerDone {
        log: log.clone(),
        rx_consumer_done: Mutex::new(Some(rx_consumer_done)),
    };

    let rocket = rocket::custom(config)
        .attach(common::rocket::LogFairing(log))
        .attach(common::rocket::GuardInternalErrors())
        .attach(wait_for_status_consumer)
        .manage(app_state);

    let rocket = install_handlers(rocket).ignite().await?;

    let wait_for_shutdown = consumer::WaitForShutdown {
        shutdown: rocket.shutdown(),
        tx_consumer_done,
    };

    return Ok((rocket, wait_for_shutdown));
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let log = create_logger();
    info!(log, "Configuring 'API' service.");

    info!(log, "Load environment variables.");
    dotenv().ok();

    info!(log, "Initialize blob storage.");
    let storage: Arc<dyn BlobStorage> = get_storage();

    let (creds, config) = get_job_queue_config();
    let (job_queue, status_queue) = setup_queues(&log, creds, config).await;

    let database_url = get_env_var("DATABASE_URL").take();

    let (rocket, consumer_wait_for_shutdown) =
        create_rocket(log.clone(), &database_url, job_queue, storage).await?;

    consumer::spawn_status_consumer(
        log.clone(),
        status_queue,
        &database_url,
        consumer_wait_for_shutdown,
    )
    .await;

    rocket.launch().await?;

    info!(&log, "Application terminated.");
    return Ok(());
}

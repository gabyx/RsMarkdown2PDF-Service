use amqprs::{
    channel::{BasicConsumeArguments, Channel},
    consumer::DefaultConsumer,
};
use common::{
    log::{create_logger, info},
    queue::{get_queue_config, setup_queue_connection},
};

use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Notify;

async fn install_consumer(log: &slog::Logger, channel: &Channel, queue_name: &str) {
    info!(log, "Installing consumer on the queue");

    let args = BasicConsumeArguments::new(&queue_name, "converter")
        .manual_ack(true)
        .finish();

    channel
        .basic_consume(DefaultConsumer::new(args.no_ack), args)
        .await
        .unwrap();
}

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let log = Arc::new(create_logger());
    info!(log, "Configuring 'converter' service.");

    info!(log, "Loading environment variables.");
    dotenv().ok();

    let (creds, queue_config) = get_queue_config();
    let (_connection, channel) = setup_queue_connection(&log, &creds, &queue_config).await;

    install_consumer(&log, &channel, &queue_config.name).await;

    info!(log, "Consume from queue '{}'...", &queue_config.name);
    let guard = Notify::new();
    guard.notified().await;
}

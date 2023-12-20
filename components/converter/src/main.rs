use amqprs::{
    callbacks::DefaultConnectionCallback,
    connection::{Connection, OpenConnectionArguments},
};
use common::log::{create_logger, info};

use async_std::task;
use dotenv::dotenv;
use std::{env, sync::Arc, time::Duration};

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let log = Arc::new(create_logger());
    info!(log, "Configuring 'converter' service.");

    info!(log, "Loading environment variables.");
    dotenv().ok();

    // Open a connection to RabbitMQ server.
    let connection = Connection::open(&OpenConnectionArguments::new(
        &env::var("RABBITMQ_HOST").expect("Host must be set."),
        5672,
        &env::var("RABBITMQ_USERNAME").expect("Username must be set."),
        &env::var("RABBITMQ_PASSWORD").expect("Password must be set."),
    ))
    .await
    .unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    loop {
        info!(log, "Waiting for stuff...");
        task::sleep(Duration::from_secs(1)).await;
    }
}

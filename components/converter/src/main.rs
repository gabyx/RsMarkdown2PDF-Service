use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
};
use common::log;

use dotenv::dotenv;
use std::{env, sync::Arc};

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let log = Arc::new(log::create_logger());
    log::info!(log, "Configuring 'converter' service.");

    dotenv().ok();

    // Open a connection to RabbitMQ server.
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        &env::var("RABBITMQ_USERNAME").expect("Username must be set."),
        &env::var("RABBITMQ_USERNAME").expect("Password must be set."),
    ))
    .await
    .unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    // // open a channel on the connection
    // let channel = connection.open_channel(None).await.unwrap();
    // channel
    //     .register_callback(DefaultChannelCallback)
    //     .await
    //     .unwrap();
    //
    // // declare a server-named transient queue
    // let (queue_name, _, _) = channel
    //     .queue_declare(QueueDeclareArguments::default())
    //     .await
    //     .unwrap()
    //     .unwrap();
    //
    // // bind the queue to exchange
    // let routing_key = "amqprs.example";
    // let exchange_name = "amq.topic";
    // channel
    //     .queue_bind(QueueBindArguments::new(
    //         &queue_name,
    //         exchange_name,
    //         routing_key,
    //     ))
    //     .await
    //     .unwrap();
    //
    // //////////////////////////////////////////////////////////////////////////////
    // // start consumer, auto ack
    // let args = BasicConsumeArguments::new(&queue_name, "basic_consumer")
    //     .manual_ack(false)
    //     .finish();
    //
    // channel
    //     .basic_consume(DefaultConsumer::new(args.no_ack), args)
    //     .await
    //     .unwrap();
    //
    // // consume forever
    // println!("consume forever..., ctrl+c to exit");
    // let guard = Notify::new();
    // guard.notified().await;
}

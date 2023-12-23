use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{Channel, ExchangeDeclareArguments, QueueBindArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
};

use crate::{config::get_env_var, log::info};

pub struct RabbitMQCredentials {
    pub host: String,
    pub password: String,
    pub username: String,
}

pub struct QueueConfig {
    pub name: String,
    pub durable: bool,
    pub routing_key: String,
    pub exchange: String,
}

pub fn get_queue_config() -> (RabbitMQCredentials, QueueConfig) {
    return (
        RabbitMQCredentials {
            host: get_env_var("RABBITMQ_HOST").take(),
            username: get_env_var("RABBITMQ_USERNAME").take(),
            password: get_env_var("RABBITMQ_PASSWORD").take(),
        },
        QueueConfig {
            name: get_env_var("CONVERTER_QUEUE").take(),
            durable: get_env_var("CONVERTER_QUEUE_DURABLE").bool().take(),
            routing_key: get_env_var("CONVERTER_QUEUE_ROUTING_KEY").take(),
            exchange: get_env_var("CONVERTER_EXCHANGE").take(),
        },
    );
}

pub async fn setup_queue_connection(
    log: &slog::Logger,
    credentials: &RabbitMQCredentials,
    config: &QueueConfig,
) -> (Connection, Channel) {
    let connection = Connection::open(&OpenConnectionArguments::new(
        &credentials.host,
        5672,
        &credentials.username,
        &credentials.password,
    ))
    .await
    .expect("Could not create a connection to rabbitmq");

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .expect("Could not register a default callback on the connection.");

    let channel = connection
        .open_channel(None)
        .await
        .expect("Could not create a channel.");

    channel
        .register_callback(DefaultChannelCallback)
        .await
        .expect("Could not register a default callback on the channel.");

    let queue_args = if config.durable {
        QueueDeclareArguments::durable_client_named(&config.name)
    } else {
        QueueDeclareArguments::transient_autodelete(&config.name)
    };

    let (_, msg_count, consumer_count) = channel
        .queue_declare(queue_args)
        .await
        .expect(&std::format!(
            "Could not create a '{}' queue [durable: {}].",
            &config.name,
            &config.durable
        ))
        .unwrap();

    info!(
        log,
        "Opened queue '{}' [message count: '{}', consumer count: '{}']",
        config.name,
        msg_count,
        consumer_count
    );

    channel
        .exchange_declare(ExchangeDeclareArguments::new(&config.exchange, "direct"))
        .await
        .expect(&std::format!(
            "Could not declare direct exchange '{}'.",
            &config.exchange,
        ));

    info!(log, "Declared direct exchange '{}'.", &config.exchange);

    // Bind the queue to an exchange, which pushes the messages
    // from consumer with `routing_key`
    // to this queue.
    channel
        .queue_bind(QueueBindArguments::new(
            &config.name,
            &config.exchange,
            &config.routing_key,
        ))
        .await
        .expect(&std::format!(
            "Could not bind queue '{}' to exchange '{}'.",
            &config.name,
            &config.exchange
        ));

    info!(
        log,
        "Bound queue '{}' to exchange '{}'", &config.name, &config.exchange
    );

    return (connection, channel);
}

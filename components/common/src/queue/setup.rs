use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, ExchangeDeclareArguments, QueueBindArguments,
        QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    BasicProperties,
};
use snafu::ResultExt;

use crate::{
    config::get_env_var,
    job::JobBundle,
    log::{info, log_panic, Logger},
    result,
    result::QueueErrorCtx,
};
use rocket::serde::json::serde_json;

#[derive(Clone, Debug)]
pub struct Credentials {
    pub host: String,
    pub password: String,
    pub username: String,
}

#[derive(Clone, Debug)]
pub struct QueueConfig {
    pub name: String,
    pub durable: bool,
    pub routing_key: String,
    pub exchange: String,
}

pub struct JobQueue {
    _connection: amqprs::connection::Connection,
    channel: amqprs::channel::Channel,
    pub config: QueueConfig,
}

impl JobQueue {
    /// Publish a job `job` such that it gets converted by a consumer.
    pub async fn publish(&self, job: &JobBundle) -> Result<(), result::Error> {
        let props = BasicProperties::default()
            .with_content_encoding("utf-8")
            .with_persistence(true)
            .with_content_type("application/json")
            .finish();

        let args = BasicPublishArguments::new(&self.config.exchange, &self.config.routing_key);

        let data = serde_json::to_vec(&job).expect("Could not serialize ");
        self.channel
            .basic_publish(props, data, args)
            .await
            .context(QueueErrorCtx {
                message: format!("Could not publish job id '{}'.", job.id),
            })?;

        return Ok(());
    }

    /// Subscribes a consumer `consumer_creator(args)` to receive jobs.
    pub async fn subscribe<F, T>(&self, consumer_creator: F) -> Result<(), result::Error>
    where
        T: AsyncConsumer + Send + 'static,
        F: FnOnce(&BasicConsumeArguments) -> T,
    {
        let args = BasicConsumeArguments::new(&self.config.name, "job-consumer")
            .manual_ack(true)
            .finish();

        let creator = consumer_creator(&args);

        self.channel
            .basic_consume(creator, args)
            .await
            .context(QueueErrorCtx {
                message: "Subscribe failed.",
            })
            .map(|_| ())?;

        return Ok(());
    }
}

pub fn get_job_queue_config() -> (Credentials, QueueConfig) {
    return (
        Credentials {
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

pub async fn setup_job_queue(
    log: &Logger,
    credentials: Credentials,
    config: QueueConfig,
) -> JobQueue {
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
        .unwrap_or_else(|_| {
            log_panic!(
                log,
                "Could not create a '{}' queue [durable: {}].",
                &config.name,
                &config.durable
            );
        })
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
        .unwrap_or_else(|_| panic!("Could not declare direct exchange '{}'.", &config.exchange,));

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
        .unwrap_or_else(|_| {
            panic!(
                "Could not bind queue '{}' to exchange '{}'.",
                &config.name, &config.exchange
            )
        });

    info!(
        log,
        "Bound queue '{}' to exchange '{}' with routing key '{}'.",
        &config.name,
        &config.exchange,
        &config.routing_key
    );

    return JobQueue {
        _connection: connection,
        channel,
        config,
    };
}

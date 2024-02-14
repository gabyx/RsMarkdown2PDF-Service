use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicCancelArguments, BasicConsumeArguments, BasicPublishArguments,
        ExchangeDeclareArguments, QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    BasicProperties,
};
use snafu::ResultExt;

use crate::{
    config::get_env_var,
    log::{info, log_panic, Logger},
    messages::{jobs::JobMessage, status::StatusMessage},
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

const JOB_QUEUE_CONSUMER_TAG: &str = "job-consumer";

pub struct JobQueue {
    _connection: amqprs::connection::Connection,
    channel: amqprs::channel::Channel,
    pub config: QueueConfig,
}

const STATUS_QUEUE_CONSUMER_TAG: &str = "status-consumer";

pub struct StatusQueue {
    _connection: amqprs::connection::Connection,
    channel: amqprs::channel::Channel,
    pub config: QueueConfig,
}

/// Subscribes a consumer `consumer_creator(args)` to consume messages.
pub async fn subscribe<F, T>(
    channel: &amqprs::channel::Channel,
    queue: &str,
    consumer_creator: F,
    consumer_tag: &str,
) -> Result<(), result::Error>
where
    T: AsyncConsumer + Send + 'static,
    F: FnOnce(&BasicConsumeArguments) -> T,
{
    let args = BasicConsumeArguments::new(queue, consumer_tag)
        .manual_ack(true)
        .finish();

    let creator = consumer_creator(&args);

    channel
        .basic_consume(creator, args)
        .await
        .context(QueueErrorCtx {
            message: "Subscribe failed.",
        })
        .map(|_| ())?;

    return Ok(());
}

pub async fn unsubscribe(channel: &amqprs::channel::Channel, consumer_tag: &str) {
    let args = BasicCancelArguments::new(consumer_tag);
    _ = channel.basic_cancel(args).await;
}

impl JobQueue {
    /// Publish a job `job` such that it gets converted by a consumer.
    pub async fn publish<J>(&self, job: J) -> Result<(), result::Error>
    where
        J: Into<JobMessage>,
    {
        let job: JobMessage = job.into();

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

    pub async fn subscribe<F, T>(&self, consumer_creator: F) -> Result<(), result::Error>
    where
        T: AsyncConsumer + Send + 'static,
        F: FnOnce(&BasicConsumeArguments) -> T,
    {
        return subscribe(
            &self.channel,
            &self.config.name,
            consumer_creator,
            JOB_QUEUE_CONSUMER_TAG,
        )
        .await;
    }

    pub async fn unsubscribe(&self) {
        unsubscribe(&self.channel, STATUS_QUEUE_CONSUMER_TAG).await;
    }
}

impl StatusQueue {
    /// Publish a job `job` such that it gets converted by a consumer.
    pub async fn publish(&self, status: &StatusMessage<&str>) -> Result<(), result::Error> {
        let props = BasicProperties::default()
            .with_content_encoding("utf-8")
            .with_persistence(true)
            .with_content_type("application/json")
            .finish();

        let args = BasicPublishArguments::new(&self.config.exchange, &self.config.routing_key);

        let data = serde_json::to_vec(&status).expect("Could not serialize ");

        self.channel
            .basic_publish(props, data, args)
            .await
            .context(QueueErrorCtx {
                message: format!(
                    "Could not publish status message job id '{}'.",
                    status.job_id()
                ),
            })?;

        return Ok(());
    }

    pub async fn subscribe<F, T>(&self, consumer_creator: F) -> Result<(), result::Error>
    where
        T: AsyncConsumer + Send + 'static,
        F: FnOnce(&BasicConsumeArguments) -> T,
    {
        return subscribe(
            &self.channel,
            &self.config.name,
            consumer_creator,
            STATUS_QUEUE_CONSUMER_TAG,
        )
        .await;
    }

    pub async fn unsubscribe(&self) {
        unsubscribe(&self.channel, STATUS_QUEUE_CONSUMER_TAG).await;
    }
}

pub struct QueuesConfig {
    jobs: QueueConfig,
    status: QueueConfig,
}

pub fn get_job_queue_config() -> (Credentials, QueuesConfig) {
    return (
        Credentials {
            host: get_env_var("RABBITMQ_HOST").take(),
            username: get_env_var("RABBITMQ_USERNAME").take(),
            password: get_env_var("RABBITMQ_PASSWORD").take(),
        },
        QueuesConfig {
            jobs: QueueConfig {
                name: get_env_var("QUEUE_JOBS").take(),
                durable: get_env_var("QUEUE_JOBS_DURABLE").bool().take(),
                routing_key: get_env_var("QUEUE_JOBS_ROUTING_KEY").take(),
                exchange: get_env_var("EXCHANGE_NAME").take(),
            },
            status: QueueConfig {
                name: get_env_var("QUEUE_STATUS").take(),
                durable: get_env_var("QUEUE_STATUS_DURABLE").bool().take(),
                routing_key: get_env_var("QUEUE_STATUS_ROUTING_KEY").take(),
                // So far both queus use the same exchange.
                exchange: get_env_var("EXCHANGE_NAME").take(),
            },
        },
    );
}

pub async fn setup_queues(
    log: &Logger,
    credentials: Credentials,
    configs: QueuesConfig,
) -> (JobQueue, StatusQueue) {
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

    for cfg in [&configs.jobs, &configs.status] {
        let queue_args = if cfg.durable {
            QueueDeclareArguments::durable_client_named(&cfg.name)
        } else {
            QueueDeclareArguments::transient_autodelete(&cfg.name)
        };

        let (_, msg_count, consumer_count) = channel
            .queue_declare(queue_args)
            .await
            .unwrap_or_else(|_| {
                log_panic!(
                    log,
                    "Could not create a '{}' queue [durable: {}].",
                    &cfg.name,
                    &cfg.durable
                );
            })
            .unwrap();

        info!(
            log,
            "Opened queue '{}' [message count: '{}', consumer count: '{}']",
            cfg.name,
            msg_count,
            consumer_count
        );

        channel
            .exchange_declare(ExchangeDeclareArguments::new(&cfg.exchange, "direct"))
            .await
            .unwrap_or_else(|_| panic!("Could not declare direct exchange '{}'.", &cfg.exchange,));
        info!(log, "Declared direct exchange '{}'.", &cfg.exchange);

        // Bind the queue to an exchange, which pushes the messages
        // from consumer with `routing_key`
        // to this queue.
        channel
            .queue_bind(QueueBindArguments::new(
                &cfg.name,
                &cfg.exchange,
                &cfg.routing_key,
            ))
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Could not bind queue '{}' to exchange '{}'.",
                    &cfg.name, &cfg.exchange
                )
            });

        info!(
            log,
            "Bound queue '{}' to exchange '{}' with routing key '{}'.",
            &cfg.name,
            &cfg.exchange,
            &cfg.routing_key
        );
    }

    return (
        JobQueue {
            _connection: connection.clone(),
            channel: channel.clone(),
            config: configs.jobs,
        },
        StatusQueue {
            _connection: connection.clone(),
            channel: channel.clone(),
            config: configs.status,
        },
    );
}

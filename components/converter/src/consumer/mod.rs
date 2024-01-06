use std::sync::Arc;

use amqprs::{
    channel::{BasicAckArguments, Channel},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use async_trait::async_trait;
use common::log::{self, info, Logger};
use std::str;

/// Default type implements the [`AsyncConsumer`].
///
/// It is used for demo and debugging purposes only.
pub struct DefaultConsumer {
    log: Arc<Logger>,
    no_ack: bool,
}

impl DefaultConsumer {
    /// Return a new consumer.
    ///
    /// See [Acknowledgement Modes](https://www.rabbitmq.com/consumers.html#acknowledgement-modes)
    ///
    /// no_ack = [`true`] means automatic ack and should NOT send ACK to server.
    ///
    /// no_ack = [`false`] means manual ack, and should send ACK message to server.
    pub fn new(log: Arc<Logger>, no_ack: bool) -> Self {
        Self { log, no_ack }
    }
}

#[async_trait]
impl AsyncConsumer for DefaultConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        info!(
            self.log,
            "Consume delivery {} on channel {}, content size: {}, content: \n{}",
            deliver,
            channel,
            content.len(),
            str::from_utf8(content.as_slice()).expect("Could not decode.")
        );

        // ack explicitly if manual ack
        if !self.no_ack {
            info!(
                self.log,
                "Ack to delivery {} on channel {}", deliver, channel
            );

            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}

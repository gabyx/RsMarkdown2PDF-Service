use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
    BasicProperties,
};
use tokio::time;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use dotenvy::dotenv;
use std::env;


fn env(name: &str) -> String {
    let message = format!("{:} must be set!", name);
    env::var(name).expect(&message)
}


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    dotenv().ok();


    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();
    
    let host = env("AMQP_HOST");
    let port = env("AMQP_PORT");
    let iport = u16::from_str_radix(&port, 10).expect("AMQP_PORT must be a number");
    let user = env("AMQP_USER");
    let password = env("AMQP_PASSWORD");

    let args = OpenConnectionArguments::new(&host, iport, &user, &password);

    
    let connection = Connection::open(&args)
        .await
        .unwrap();
}

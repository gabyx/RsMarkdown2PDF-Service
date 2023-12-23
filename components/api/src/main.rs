use common::{
    config::get_env_var,
    log::{create_logger, info},
    queue::{get_queue_config, setup_queue_connection},
};

use diesel::{pg::PgConnection, prelude::*};
use dotenvy::dotenv;

use rocket::{
    config::{Config, LogLevel},
    routes,
    serde::{json::Json, Serialize},
};

use std::sync::Arc;

#[derive(Debug, Serialize)]
struct Job {
    id: String,
    document_title: String,
}

#[rocket::get("/api/jobs")]
fn get_all_jobs() -> Json<Vec<Job>> {
    let result = vec![];
    Json(result)
}

#[rocket::post("/api/debug/publish-job")]
fn send_job() {}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let log = Arc::new(create_logger());
    info!(log, "Configuring 'API' service.");

    info!(log, "Load environment variables.");
    dotenv().ok();

    let database_url = &get_env_var("DATABASE_URL").take();
    info!(log, "Establish connection with database.");
    let _connection = PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let (creds, queue_config) = get_queue_config();
    let (_connection, channel) = setup_queue_connection(&log, &creds, &queue_config).await;

    info!(log, "Start rocket.");
    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    rocket::custom(config)
        .attach(common::rocket::LogFairing(log))
        .mount("/", routes![get_all_jobs])
        .launch()
        .await?;

    return Ok(());
}

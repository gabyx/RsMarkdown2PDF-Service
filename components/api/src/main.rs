#[macro_use]
extern crate rocket;
use rocket::config::{Config, LogLevel};
use rocket::serde::{json::Json, Serialize};

use common::log::{create_logger, info};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct Job {
    id: String,
    document_title: String,
}

#[get("/api/jobs")]
fn get_all_jobs() -> Json<Vec<Job>> {
    let result = vec![];
    Json(result)
}

#[launch]
fn rocket() -> _ {
    let log = Arc::new(create_logger());
    info!(log, "Configuring 'API' service.");

    info!(log, "Load environment variables.");
    dotenv().ok();

    info!(log, "Establish connection with database.");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    info!(log, "Start rocket.");
    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    rocket::custom(config)
        .attach(common::rocket::LogFairing(log))
        .mount("/", routes![get_all_jobs])
}

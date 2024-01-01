use api::{handlers::install_handlers, state::AppState};
use common::{
    config::get_env_var,
    log::{create_logger, info},
    queue::{get_job_queue_config, setup_job_queue},
};

use diesel::{pg::PgConnection, prelude::Connection};
use dotenvy::dotenv;
use rocket::config::{Config, LogLevel};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let log = create_logger();
    info!(log, "Configuring 'API' service.");

    info!(log, "Load environment variables.");
    dotenv().ok();

    let database_url = &get_env_var("DATABASE_URL").take();

    info!(log, "Establish connection with database.");
    let db_conn = PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let (creds, config) = get_job_queue_config();
    let job_queue = setup_job_queue(&log, creds, config).await;
    let app_state = AppState::new(log.clone(), db_conn, job_queue);

    info!(log, "Start rocket.");
    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    let r = rocket::custom(config)
        .attach(common::rocket::LogFairing(log))
        .manage(app_state);

    install_handlers(r).launch().await?;

    return Ok(());
}

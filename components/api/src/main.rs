use std::sync::Arc;

use common::{
    config::get_env_var,
    job::Job,
    log::{create_logger, info},
    queue::{get_job_queue_config, setup_job_queue, JobQueue},
    response::json,
};

use diesel::{pg::PgConnection, prelude::Connection};
use dotenvy::dotenv;
use rocket::{
    config::{Config, LogLevel},
    http::Status,
    routes,
    tokio::sync::Mutex,
    State,
};

struct AppState {
    log: Arc<slog::Logger>,

    // TODO: Abstract away db connection, if possible: Make an interface in `common`
    // such that only converter/api use the same interface and dont need to know if its postgres or
    // something else.
    db: Mutex<PgConnection>,
    job_queue: JobQueue,
}

#[rocket::get("/api/jobs")]
async fn get_all_jobs(state: &State<AppState>) -> json::JsonResponse<Vec<Job>> {
    info!(state.log, "Handling 'get_all_jobs'...");
    let result = vec![Job::new("new job")];

    return json::success!(result);
}

#[rocket::post("/api/debug/publish-job")]
async fn send_job(s: &State<AppState>) -> json::JsonResponse<Job> {
    info!(s.log, "Publishing debug job into queue.");
    let job = Job::new("new job");

    return match s.job_queue.publish(&job).await {
        Ok(_) => json::success!(job),
        Err(e) => json::failure!(
            Status::InternalServerError,
            "Could not publish job id '{}', error: \n'{}'.",
            job.id,
            e
        ),
    };
}

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

    info!(log, "Start rocket.");
    let app_state = AppState {
        log: log.clone(),
        db: Mutex::new(db_conn),
        job_queue,
    };

    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    rocket::custom(config)
        .attach(common::rocket::LogFairing(log))
        .mount("/", routes![get_all_jobs, send_job])
        .manage(app_state)
        .launch()
        .await?;

    return Ok(());
}

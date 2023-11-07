#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket::serde::{json::Json, Serialize};
use std::io::Error;
use uuid::Uuid;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::status::NotFound;

mod database;
mod files;
mod messages;

#[derive(Debug, Serialize)]
struct SubmittedJob {
    id: Uuid,
}

#[get("/api/jobs")]
async fn get_all_jobs() -> Json<Vec<messages::Job>> {
    let result = vec![];
    Json(result)
}

#[derive(FromForm, Debug)]
struct JobUpload<'r> {
    name: String,
    stylesheet: String,
    file: TempFile<'r>,
}

#[put("/api/job", data = "<job>")]
fn submit_job(mut job: Form<JobUpload<'_>>) -> Result<Json<SubmittedJob>, NotFound<String>> {
    println!(
        "received job name: {:?}, file size in bytes: {}",
        &job.name,
        &job.file.len()
    );
    let name = job.name.clone();
    let stylesheet = job.stylesheet.clone();
    let job_bundle = files::create_job_bundle(&mut job.file, name, stylesheet);
    match job_bundle {
        Ok(bundle) => Ok(Json(SubmittedJob { id: bundle.id })),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    database::migrate_if_needed();

    rocket::build().mount("/", routes![get_all_jobs, submit_job])
}

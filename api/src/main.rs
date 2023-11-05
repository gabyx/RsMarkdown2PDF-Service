#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use dotenvy::dotenv;
use uuid::Uuid;

use rocket::form::Form;
use rocket::fs::TempFile;

mod files;
mod database;
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

#[derive(FromForm, Deserialize, Debug)]
struct JobMetadata {
    name: String,
}

#[derive(FromForm, Debug)]
struct JobUpload<'r> {
    metadata: Json<JobMetadata>,
    file: TempFile<'r>,
}

#[put("/api/job", data = "<job>")]
async fn submit_job(job: Form<JobUpload<'_>>) -> Json<SubmittedJob> {
    println!("received job metadata: {:?}, file size in bytes: {}", job.metadata, job.file.len());
    let job_id = Uuid::new_v4();
    Json(SubmittedJob { id: job_id })
}



#[launch]
fn rocket() -> _ {
    dotenv().ok();

    database::migrate_if_needed();

    rocket::build().mount("/", routes![get_all_jobs, submit_job])
}

#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize, Deserialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

use rocket::form::Form;
use rocket::fs::TempFile;


#[derive(Debug, Serialize)]
struct Job {
    id: String,
    document_title: String,
    document_size_in_bytes: i32,
    status: Status,
}

#[derive(Debug, Serialize)]
enum Status {
    Ready,
    Processing,
    Done,
    Failed,
}

#[derive(Debug, Serialize)]
struct Message {
    name: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct SubmittedJob {
    id: Uuid
}

#[get("/hello/<name>")]
async fn hello(name: String) -> Json<Message> {
    let result = Message {
        name,
        message: String::from("Hoi!"),
    };
    Json(result)
}

#[get("/api/jobs")]
async fn get_all_jobs() -> Json<Vec<Job>> {
    let result = vec![];

    Json(result)
}

#[derive(FromForm, Deserialize, Debug)]
struct JobMetadata {
    name: String
}


#[derive(FromForm, Debug)]
struct JobUpload<'r> {
    metadata: Json<JobMetadata>,
    file: TempFile<'r>,
}

#[put("/api/job", data = "<job>")]
async fn submit_job(job: Form<JobUpload<'_>>) -> Json<SubmittedJob> {
    println!("received job: {:?}", job);

    Json(SubmittedJob{
        id: Uuid::new_v4()
    })
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // TODO: figure out how to properly configure the logger.
    println!("Connected to the database!");

    rocket::build().mount("/", routes![get_all_jobs, hello, submit_job])
}

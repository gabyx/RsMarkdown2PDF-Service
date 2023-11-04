#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};


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
    rocket::build().mount("/", routes![get_all_jobs])
}

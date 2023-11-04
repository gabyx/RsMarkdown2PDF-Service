#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


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

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    
    // TODO: figure out how to properly configure the logger. 
    println!("Connected to the database!");

    rocket::build().mount("/", routes![get_all_jobs])
}

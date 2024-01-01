use crate::state::AppState;
use common::{job::Job, log::info, response::json};
use rocket::{
    form::Form,
    fs::TempFile,
    http::Status,
    routes,
    serde::{json::Json, Deserialize, Serialize},
    Build, FromForm, Rocket, State,
};
use uuid::Uuid;

#[rocket::get("/api/jobs")]
async fn get_all_jobs(s: &State<AppState>) -> json::JsonResponse<Vec<Job>> {
    info!(s.log, "Getting all jobs.");

    let result = vec![Job::new("my-doc")];
    return json::success!(result);
}

#[rocket::get("/api/job/<uuid>")]
async fn get_job(s: &State<AppState>, uuid: &str) -> json::JsonResponse<Job> {
    info!(s.log, "Getting job id: '{}'.", uuid);

    let job = Job::new("new job");
    return json::success!(job);
}

#[derive(Debug, Serialize)]
struct SubmittedJob {
    id: Uuid,
}

#[derive(FromForm, Deserialize, Debug)]
struct JobMetaData {
    name: String,
}
#[derive(FromForm, Debug)]
struct JobUpload<'r> {
    metadata: Json<JobMetaData>,
    file: TempFile<'r>,
}

#[rocket::put("/api/job", data = "<job>")]
async fn submit_job(
    s: &State<AppState>,
    job: Form<JobUpload<'_>>,
) -> json::JsonResponse<SubmittedJob> {
    info!(s.log, "Submit job {:?}", job);
    return json::success!(SubmittedJob { id: Uuid::new_v4() });
}

/// Install all handlers for this application.
pub fn install_handlers(r: Rocket<Build>) -> Rocket<Build> {
    let r = r.mount("/", routes![get_job, get_all_jobs, submit_job]);
    return install_debug_handlers(r);
}

#[cfg(not(feature = "debug-handlers"))]
fn install_debug_handlers(r: Rocket<Build>) -> Rocket<Build> {
    return r;
}

#[cfg(feature = "debug-handlers")]
fn install_debug_handlers(r: Rocket<Build>) -> Rocket<Build> {
    return r.mount("/", routes![submit_job_debug]);
}

#[rocket::post("/api/debug/job")]
async fn submit_job_debug(s: &State<AppState>) -> json::JsonResponse<Job> {
    info!(s.log, "Publishing debug job into queue.");

    let job = Job::new("my-doc");

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

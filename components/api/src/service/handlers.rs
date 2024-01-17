#![allow(unused_imports)] // Rocket generates pub functions which cause these warnings.

use common::{job::JobBundle, log::info, response::json};
use rocket::{form::Form, http::Status, routes, Build, Rocket, State};

use crate::{
    messages::{JobUpload, SubmittedJob},
    persist,
    state::AppState,
};

#[rocket::get("/api/jobs")]
async fn get_all_jobs(s: &State<AppState>) -> json::JsonResponse<Vec<JobBundle>> {
    info!(s.log, "Getting all jobs.");

    let result = vec![JobBundle::new("my-doc", "no-digest", "text/markdown")];
    return json::success!(result);
}

#[rocket::get("/api/job/<uuid>")]
async fn get_job(s: &State<AppState>, uuid: &str) -> json::JsonResponse<JobBundle> {
    info!(s.log, "Getting job id: '{}'.", uuid);

    let job = JobBundle::new("new job", "no-digest", "text/markdown");
    return json::success!(job);
}

#[rocket::put("/api/job", data = "<job>")]
async fn submit_job(
    s: &State<AppState>,
    mut job: Form<JobUpload<'_>>,
) -> json::JsonResponse<SubmittedJob> {
    info!(s.log, "Submit job {:?}", job);

    let name = job.metadata.name.clone();

    let job_bundle =
        persist::create_job_bundle(&s.log, &mut job.file, &name, s.storage.clone()).await?;

    // TODO: store that shit into the db and send it to the queue.

    json::success!(SubmittedJob {
        id: job_bundle.id,
        digest: job_bundle.blob_digest
    })
}

#[rocket::post("/api/debug/job")]
async fn submit_job_debug(s: &State<AppState>) -> json::JsonResponse<JobBundle> {
    info!(s.log, "Publishing debug job into queue.");

    let job = JobBundle::new("my-doc", "no-digest", "text/markdown");

    return match s.job_queue.publish(&job).await {
        Ok(_) => json::success!(job),
        Err(e) => json::failure!(
            &s.log,
            Status::InternalServerError,
            "Could not publish job id '{}', error: \n'{}'.",
            job.id,
            e
        ),
    };
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

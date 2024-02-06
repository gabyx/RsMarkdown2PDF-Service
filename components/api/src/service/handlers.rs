#![allow(unused_imports)] // Rocket generates pub functions which cause these warnings.

use std::ops::DerefMut;

use common::{
    db,
    job::JobBundle,
    log::info,
    response,
    response::{json, Status},
    result::ResultExt,
};
use rocket::{form::Form, routes, Build, Rocket, Shutdown, State};
use snafu::prelude::*;

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

    {
        info!(s.log, "Insert job meta '{}' into database.", job_bundle.id);
        let mut d = s.db.lock().await;
        db::transactions::insert_job(d.deref_mut(), &job_bundle).log(&s.log)?
    }

    info!(s.log, "Submit job '{}' to queue.", job_bundle.id);
    s.job_queue.publish(&job_bundle).await.log(&s.log)?;
    json::success!(SubmittedJob {
        id: job_bundle.id,
        digest: job_bundle.blob_digest
    })
}

#[rocket::get("/api/shutdown")]
fn shutdown(shutdown: Shutdown) {
    shutdown.notify();
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
    return r.mount("/", routes![shutdown]);
}

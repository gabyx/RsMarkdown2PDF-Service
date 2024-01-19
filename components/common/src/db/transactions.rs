use super::{
    models::{ConverterResult, DbJob, JobStatus},
    schema::jobs::dsl::*,
};
use crate::{job::JobBundle, result, result::Res};
use diesel::{insert_into, pg::Pg, Connection, RunQueryDsl};
use snafu::prelude::*;

pub fn insert_job(conn: &mut impl Connection<Backend = Pg>, job: &JobBundle) -> Res<()> {
    // Convert into internal DB type.
    let j = DbJob {
        id: job.id,
        status: JobStatus::Pending,
        blob_digest: job.blob_digest.clone(),
        name: job.name.clone(),
        converter_result: ConverterResult::None,
        converter_log: "".to_owned(),
    };

    insert_into(jobs)
        .values(j)
        .execute(conn)
        .context(result::DBErrorCtx {
            message: "Transaction 'insert_job' failed.",
        })?;

    return Ok(());
}

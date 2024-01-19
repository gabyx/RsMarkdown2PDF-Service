use diesel::prelude::*;
use diesel_derive_enum;
use uuid::Uuid;

// CREATE TYPE JobStatus as ENUM(...);
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::db::schema::sql_types::StatusT"]
pub enum JobStatus {
    Pending,
    Queued,
    Running,
    Done,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::db::schema::sql_types::ResultT"]
pub enum ConverterResult {
    None,
    Failure,
    Success,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbJob {
    pub id: Uuid,
    pub status: JobStatus,

    pub blob_digest: String,

    pub name: String,

    pub converter_result: ConverterResult,
    pub converter_log: String,
}

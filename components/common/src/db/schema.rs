// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "level_t"))]
    pub struct LevelT;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "result_t"))]
    pub struct ResultT;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_t"))]
    pub struct StatusT;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LevelT;

    job_logs (id) {
        id -> Uuid,
        job_id -> Uuid,
        timestamp -> Timestamp,
        message -> Text,
        level -> LevelT,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusT;
    use super::sql_types::ResultT;

    jobs (id) {
        id -> Uuid,
        status -> StatusT,
        submit_time -> Timestamp,
        end_time -> Timestamp,
        blob_digest -> Varchar,
        name -> Varchar,
        converter_result -> ResultT,
        converter_log -> Text,
    }
}

diesel::joinable!(job_logs -> jobs (job_id));

diesel::allow_tables_to_appear_in_same_query!(job_logs, jobs,);

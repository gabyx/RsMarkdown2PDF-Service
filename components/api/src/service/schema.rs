// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "result_t"))]
    pub struct ResultT;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_t"))]
    pub struct StatusT;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusT;
    use super::sql_types::ResultT;

    jobs (id) {
        id -> Varchar,
        name -> Varchar,
        blob_digest -> Varchar,
        status -> StatusT,
        converter_result -> Nullable<ResultT>,
        converter_log -> Nullable<Text>,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    jobs (id) {
        id -> Varchar,
        document_title -> Varchar,
        document_size_in_bytes -> Int4,
        status -> Varchar,
    }
}

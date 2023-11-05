use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // TODO: figure out how to properly configure the logger.
    println!("Connected to the database!");
    connection
}

pub fn migrate_if_needed() {
	let mut connection = establish_connection();
    println!("Starting to run pending migrations.");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
    println!("Finished running pending migrations.");
}

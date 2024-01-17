use common::{
    config::get_env_var,
    log::{create_logger, info, log_panic},
};
use diesel::{backend::Backend, pg::PgConnection, prelude::Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db-migration/migrations");

fn run_migrations<DB: Backend>(
    connection: &mut impl MigrationHarness<DB>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

fn main() {
    let log = create_logger();
    info!(log, "Configuring 'API' service.");

    info!(log, "Load environment variables.");
    dotenv().ok();

    let database_url = &get_env_var("DATABASE_URL").take();

    info!(log, "Establish connection with database.");
    let mut db_conn = PgConnection::establish(database_url).unwrap_or_else(|e| {
        log_panic!(log, "Error connecting to '{}': error:\n{}", database_url, e);
    });

    info!(log, "Run pending migrations ...");
    run_migrations(&mut db_conn).unwrap_or_else(|e| {
        log_panic!(log, "Could not run migration: error:\n{}", e);
    });

    return;
}

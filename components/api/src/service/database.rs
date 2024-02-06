use common::{
    log::{info, Logger},
    log_panic,
};
use diesel::{Connection, PgConnection};

pub fn connect(log: &Logger, url: &str) -> PgConnection {
    info!(log, "Establish connection with database.");
    return PgConnection::establish(url).unwrap_or_else(|_| {
        log_panic!(log, "Error connecting to {}", url);
    });
}

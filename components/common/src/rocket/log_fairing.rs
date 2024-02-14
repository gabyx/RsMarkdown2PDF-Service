use std::sync::Arc;

use crate::log::{self, Logger};
use rocket::{
    fairing::{Fairing, Info, Kind},
    Build, Data, Orbit, Request, Response, Rocket,
};

/// Newtype struct wrapper around the passed-in `Logger`.
#[derive(Debug, Clone)]
pub struct LogFairing(pub Arc<Logger>);

impl LogFairing {
    pub fn new(logger: Arc<Logger>) -> LogFairing {
        return LogFairing(logger);
    }

    pub fn get(&self) -> &log::Logger {
        &self.0
    }
}

impl std::ops::Deref for LogFairing {
    type Target = log::Logger;

    fn deref(&self) -> &log::Logger {
        &self.0
    }
}

#[rocket::async_trait]
impl Fairing for LogFairing {
    fn info(&self) -> Info {
        Info {
            name: "Slog Fairing",
            kind: Kind::Liftoff | Kind::Request | Kind::Response | Kind::Shutdown,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        log::info!(&self.0, "Starting up rocket...");
        Ok(rocket.manage(self.clone()))
    }

    async fn on_request(&self, r: &mut Request<'_>, _: &mut Data<'_>) {
        log::info!(&self.0, "Handling Request: '{}'", r)
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, r: &mut Response<'r>) {
        if r.status().class().is_server_error() {
            let s = r
                .body_mut()
                .to_string()
                .await
                .expect("Could not read body to log internal error.");

            log::critical!(&self.0, "Internal server error response occured:\n{}", s);
        }
    }

    async fn on_shutdown(&self, _r: &Rocket<Orbit>) {
        log::info!(&self.0, "Shutting down rocket.");
    }
}

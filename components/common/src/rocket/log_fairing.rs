use std::sync::Arc;

use crate::log;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Build, Data, Orbit, Request, Response, Rocket,
};

/// Newtype struct wrapper around the passed-in slog::Logger
#[derive(Debug, Clone)]
pub struct LogFairing(pub Arc<log::Logger>);

impl LogFairing {
    pub fn new(logger: Arc<log::Logger>) -> LogFairing {
        return LogFairing(logger);
    }

    pub fn get(&self) -> &log::Logger {
        &*self.0
    }
}

impl std::ops::Deref for LogFairing {
    type Target = log::Logger;

    fn deref(&self) -> &log::Logger {
        &*self.0
    }
}

#[rocket::async_trait]
impl Fairing for LogFairing {
    fn info(&self) -> Info {
        Info {
            name: "Slog Fairing",
            kind: Kind::Ignite | Kind::Liftoff | Kind::Request | Kind::Response,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        log::info!(&self.0, "Starting up rocket...");
        Ok(rocket.manage(self.clone()))
    }

    async fn on_liftoff(&self, _: &Rocket<Orbit>) {}

    async fn on_request(&self, _: &mut Request<'_>, _: &mut Data<'_>) {}

    async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {}
}

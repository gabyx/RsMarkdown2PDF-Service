use std::io;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response,
};

/// Newtype struct wrapper around the passed-in `Logger`.
pub struct GuardInternalErrors();

#[rocket::async_trait]
impl Fairing for GuardInternalErrors {
    fn info(&self) -> Info {
        Info {
            name: "Guard Internal Errors",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, r: &mut Response<'r>) {
        if r.status().class().is_server_error() {
            let s = "Internal error occured, see the logs.";
            r.set_sized_body(s.len(), io::Cursor::new(s));
        }
    }
}

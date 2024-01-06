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
            let size = r.body_mut().size().await.unwrap_or(0);
            assert!(
                size == 0,
                "Rocket should never send any body for server errors, as we experienced (?)."
            );

            let s = "Internal error occured, see the logs, maybe plant a tree\n\
                     or go shopping for the greater good of humanity or maybe\n\
                     just maybe discuss the quantity of gender.";

            r.set_sized_body(s.len(), io::Cursor::new(s));
        }
    }
}

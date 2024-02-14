use crate::result;
use rocket::response::status::Custom;

pub type Status = rocket::http::Status;

// The response error used. Currently we are just returning an normal
// string on errors.
pub struct Error(pub Custom<String>);

impl Error {
    pub fn new(status: Status, msg: String) -> Error {
        return Error(Custom(status, msg));
    }
}

#[macro_export]
macro_rules! _error {
    ($status:expr, $($args:tt)+) => {
        $crate::response::error::Error::new($status, format!($($args)+))

    };
}

pub use _error as error;

/// Converstion to a `response::Error` from external ones.
impl From<result::Error> for Error {
    fn from(value: result::Error) -> Self {
        return match value {
            result::Error::IOError { .. } => {
                error!(Status::InternalServerError, "IO Error.")
            }
            result::Error::DBError { .. } => {
                error!(Status::InternalServerError, "Database Error.")
            }
            result::Error::QueueError { .. } => {
                error!(Status::InternalServerError, "Queue Error.")
            }
            result::Error::GenericError { .. } => {
                error!(Status::InternalServerError, "GenericError")
            }
        };
    }
}

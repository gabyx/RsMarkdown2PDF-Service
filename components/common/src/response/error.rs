use core::fmt;

use rocket::{http::Status, response::status::Custom};

#[derive(Debug)]
pub struct Error {
    inner: Custom<String>,
}

impl Error {
    pub fn new(status: Status, msg: String) -> Error {
        return Error {
            inner: Custom(status, msg),
        };
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        return write!(f, "{}", self.inner.1);
    }
}

#[macro_export]
macro_rules! _error {
    ($status:expr, $($args:tt)+) => {
        Error::new($status, format!($($args)+))
    };
}

pub use _error as error;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        return error!(Status::InternalServerError, "IO Error: {}", value);
    }
}

impl From<Error> for Custom<String> {
    fn from(value: Error) -> Self {
        return value.inner;
    }
}

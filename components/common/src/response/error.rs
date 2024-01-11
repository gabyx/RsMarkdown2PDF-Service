use core::fmt;
use rocket::{http::Status, response::status::Custom};
use std::backtrace;

#[derive(Debug)]
pub struct Error {
    inner: Custom<String>,
}

/// We wrap the Custom error with our own Error to better denote what it is.
/// Type alias behave shitty with Ctors...
impl Error {
    pub fn new(
        status: Status,
        msg: String,
    ) -> Error {
        return Error {
            inner: Custom(status, msg),
        };
    }
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        return write!(f, "{}", self.inner.1);
    }
}

#[macro_export]
macro_rules! _error {
    ($status:expr, $($args:tt)+) => {
        $crate::response::error::Error::new($status, format!($($args)+))
    };
}

pub use _error as error;

/// TODO: This is better handled probably with something similar to
/// https://github.com/rust-lang-deprecated/error-chain.
/// for now its a solution.
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        let b = backtrace::Backtrace::capture();
        return error!(
            Status::InternalServerError,
            "IO Error: {}\nBacktrace:\n{}", value, b
        );
    }
}

impl From<Error> for Custom<String> {
    fn from(value: Error) -> Self {
        return value.inner;
    }
}

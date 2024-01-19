pub mod error;
pub use error::*;

use crate::log::{error, Logger};
use snafu::Report;
use std::fmt::Display;

/// The comman error shared across the components.
pub type Res<T> = Result<T, Error>;

/// Extention trait to log result if [`Result`] is [`Err`] as a report.
pub trait ResultExt<T, E>: Sized {
    fn log(self, log: &Logger) -> Result<T, E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Display + std::error::Error,
{
    fn log(self, log: &Logger) -> Result<T, E> {
        if let Err(err) = self {
            error!(log, "{}", Report::from_error(&err));
            return Err(err);
        }

        return self;
    }
}

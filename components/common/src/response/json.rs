use crate::result;
use rocket::{
    http::{Status, StatusClass},
    response::status::Custom,
    serde::json::Json,
};

/// A common JSON response which is a result containing either the `Ok` value
/// `Json<R>`
/// or the `Err` value
/// as a failed response with `Custom<String>`.
pub type JsonResponse<R> = std::result::Result<Json<R>, Custom<String>>;

/// Easily create a succesfull JSON response.
pub fn new_success<R>(r: R) -> JsonResponse<R> {
    return Ok(Json(r));
}

/// Easily create a JSON failure response.
pub fn new_failure<R>(status: Status, msg: String) -> JsonResponse<R> {
    assert!(
        status.class() == StatusClass::ClientError || status.class() == StatusClass::ServerError
    );
    return Err(Custom(status, msg));
}

/// Create a succeeded JSON response.
#[macro_export]
macro_rules! _success {
    ($data:expr) => {
        $crate::response::json::new_success($data)
    };
}
pub use _success as success;

/// Create a failed JSON response and before doing so log it as error.
#[macro_export]
macro_rules! _failure {
    ($log:expr, $status:expr, $($args:tt)+) => {
        {
            let msg = format!($($args)+);
            $crate::log::error!($log, "Request failure occured: {}", &msg);

            // Note: This call seems to miss type information but the compiler
            // does magically deduce the type of `R` in `new_failure<R>`.
            $crate::response::json::new_failure($status, msg)
        }
    };
}
pub use _failure as failure;

impl From<result::Error> for Custom<String> {
    fn from(value: result::Error) -> Self {
        return super::Error(value.into()).into();
    }
}

impl From<super::Error> for Custom<String> {
    fn from(value: super::Error) -> Self {
        return value.0;
    }
}

impl From<result::Error> for super::Error {
    fn from(value: result::Error) -> Self {
        return match value {
            result::Error::IOError {
                source: _,
                backtrace: _,
            } => super::error!(Status::InternalServerError, "IO Error."),
            result::Error::QueueError {
                message: _,
                source: _,
                backtrace: _,
            } => super::error!(Status::InternalServerError, "Queue Error."),
            result::Error::DBError {
                message: _,
                source: _,
                backtrace: _,
            } => super::error!(Status::InternalServerError, "Database Error."),
            result::Error::GenericError {
                message: _,
                source: _,
            } => super::error!(Status::InternalServerError, "GenericError"),
        };
    }
}

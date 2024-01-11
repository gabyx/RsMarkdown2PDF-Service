use rocket::{
    http::{Status, StatusClass},
    serde::json::Json,
};

use rocket::response::status::Custom;

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
pub fn new_failure<R>(
    status: Status,
    msg: String,
) -> JsonResponse<R> {
    assert!(
        status.class() == StatusClass::ClientError || status.class() == StatusClass::ServerError
    );
    return Err(Custom(status, msg));
}

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

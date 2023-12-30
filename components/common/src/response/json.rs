use rocket::{http::Status, response::status::Custom, serde::json::Json};

/// A common JSON response which is a result containing either the `Ok` value
/// `Json<R>`
/// or the `Err` value
/// as a failed response with `Custom<String>`.
pub type JsonResponse<R> = std::result::Result<Json<R>, Custom<String>>;

pub fn new_success<R>(r: R) -> JsonResponse<R> {
    return Ok(Json(r));
}

pub fn new_failure<R>(status: Status, msg: String) -> JsonResponse<R> {
    return Err(Custom(status, msg));
}

#[macro_export]
macro_rules! _success {
    ($data:expr) => {
        $crate::response::json::new_success($data);
    };
}

pub use _success as success;

#[macro_export]
macro_rules! _failure {
    ($status:expr, $($args:tt)+) => {
        // Note: This call seems to miss type information but the compiler
        // does magically deduce the type of `R` in `new_failure<R>`.
        $crate::response::json::new_failure($status, format!($($args)+));
    };
}

pub use _failure as failure;

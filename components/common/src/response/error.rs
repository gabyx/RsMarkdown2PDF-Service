use rocket::response::status::Custom;

pub type Status = rocket::http::Status;
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

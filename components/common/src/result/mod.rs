use simple_error::SimpleError;

pub type Error = SimpleError;

pub type ResBox<T> = Result<T, Box<dyn std::error::Error>>;
pub type Res<T> = Result<T, Error>;

use snafu::{prelude::*, Backtrace};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))] // Sets the default visibility for these context selectors
pub enum Error {
    #[snafu(context(suffix(ErrorCtx)))]
    IOError {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(context(suffix(ErrorCtx)), display("Database Error: {message}"))]
    DBError {
        message: String,
        source: diesel::result::Error,
        backtrace: Backtrace,
    },

    #[snafu(context(suffix(ErrorCtx)), display("Queue Error: {message}"))]
    QueueError {
        message: String,
        source: amqprs::error::Error,
        backtrace: Backtrace,
    },

    #[snafu(whatever, display("Generic Error: {message}"))]
    GenericError {
        message: String,
        // Having a `source` is optional, but if it is present, it must
        // have this specific attribute and type:
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}

use slog::{o, Drain};
use slog_async;
use std::io;

#[allow(dead_code)]
fn no_out(_io: &mut dyn io::Write) -> io::Result<()> {
    return Ok(());
}

pub fn create_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        //.use_custom_timestamp(no_out)
        .build()
        .fuse();

    let drain = slog_async::Async::new(drain)
        .chan_size(5_000_000)
        .build()
        .fuse();

    return slog::Logger::root(drain, o!());
}

pub type Logger = slog::Logger;

/// Log trace level record
#[macro_export]
macro_rules! log_trace(
    ($log:expr, #$tag:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Trace, $tag, $($args)+)
    };
    ($log:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Trace, "", $($args)+)
    };
);

pub use log_trace as trace;

/// Log debug level record
#[macro_export]
macro_rules! log_debug(
    ($log:expr, #$tag:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Debug, $tag, $($args)+)
    };
    ($log:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Debug, "", $($args)+)
    };
);

pub use log_debug as debug;

/// Log info level record
#[macro_export]
macro_rules! log_info(
    ($log:expr, #$tag:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Info, $tag, $($args)+)
    };
    ($log:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Info, "", $($args)+)
    };
);

pub use log_info as info;

/// Log warn level record
#[macro_export]
macro_rules! log_warn(
    ($log:expr, #$tag:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Warning, $tag, $($args)+)
    };
    ($log:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Warning, "", $($args)+)
    };
);

pub use log_warn as warn;

/// Log warn level record
#[macro_export]
macro_rules! log_error(
    ($log:expr, #$tag:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Error, $tag, $($args)+)
    };
    ($log:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Error, "", $($args)+)
    };
);

pub use log_error as error;

/// Log panic level record
#[macro_export]
macro_rules! log_panic(
    ($log:expr, #$tag:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Error, $tag, $($args)+);
        panic!();
    };
    ($log:expr, $($args:tt)+) => {
        slog::log!($log, slog::Level::Error, "", $($args)+);
        panic!();
    };
);

pub use log_panic;

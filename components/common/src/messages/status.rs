/// Messages sent on the status queue.
///
use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub enum JobStatus {
    Sucess,
    Failure,
}

#[derive(Debug, Serialize)]
pub struct JobCompletionMessage<T> {
    /// Id of the message.
    pub id: Uuid,

    /// Job id.
    pub job_id: Uuid,

    /// The timestamp of the completion.
    pub timestamp: DateTime<Utc>,

    /// The job status.
    pub status: JobStatus,

    /// The log of the conversion in UTF-8.
    pub log: T,
}

#[derive(Debug, Serialize)]
pub enum JobLogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Serialize)]
pub struct JobLogMessage<T> {
    /// Id of the message.
    pub id: Uuid,

    /// Job id.
    pub job_id: Uuid,

    /// The timestamp of the log message.
    pub timestamp: DateTime<Utc>,

    /// The message.
    pub message: T,

    /// The log level.
    pub level: JobLogLevel,
}

#[derive(Debug, Serialize)]
pub enum StatusMessage<T> {
    Log(JobLogMessage<T>),
    Completion(JobCompletionMessage<T>),
}

impl<T> StatusMessage<T> {
    pub fn job_id(&self) -> &Uuid {
        return match self {
            StatusMessage::Log(l) => &l.job_id,
            StatusMessage::Completion(c) => &c.job_id,
        };
    }
}

pub fn create_log(job_id: Uuid, message: &str, level: JobLogLevel) -> StatusMessage<&str> {
    return StatusMessage::Log(JobLogMessage {
        id: Uuid::new_v4(),
        job_id,
        timestamp: Utc::now(),
        message,
        level,
    });
}

pub fn create_completion(job_id: Uuid, status: JobStatus, log: &str) -> StatusMessage<&str> {
    return StatusMessage::Completion(JobCompletionMessage {
        id: Uuid::new_v4(),
        job_id,
        timestamp: Utc::now(),
        status,
        log,
    });
}

#[macro_export]
macro_rules! create_job_log {
    ($id:expr, $level:expr, $($args:tt)+) => {
        $crate::job::StatusMessage::create_log($id, format!($($args)+), $level)
    };
}

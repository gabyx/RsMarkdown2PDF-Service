use uuid::Uuid;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct JobInputMessage {
    id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct JobOutputMessage {
    id: Uuid,

    status: Status,
    message: String
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Ready,
    Processing,
    Finished,
    Failed
}

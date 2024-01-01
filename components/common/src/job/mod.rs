use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Job {
    pub id: Uuid,

    pub document_digest: String,
    pub document_title: String,
}

impl Job {
    pub fn new(title: &str) -> Self {
        let digest = {
            // TODO: compute here the digest of the document input.
            let context = Context::new(&SHA256);
            context.finish()
        };

        return Job {
            id: uuid::Uuid::new_v4(),
            document_title: title.to_owned(),
            document_digest: HEXLOWER.encode(digest.as_ref()),
        };
    }
}

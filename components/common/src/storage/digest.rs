use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA256};
use rocket::tokio::io::AsyncReadExt;
use std::io::Error;

pub async fn get_digest<R: AsyncReadExt + Unpin>(mut reader: R) -> Result<String, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    let digest = context.finish();

    return Ok(HEXLOWER.encode(digest.as_ref()));
}

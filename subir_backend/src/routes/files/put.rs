use std::convert::Infallible;

use futures::stream::once;
use multer::{bytes::Bytes, Multipart};
use serde::Deserialize;
use tide::{Request, Result};

use crate::app::{AppState, FileChannelMessage};

#[derive(Deserialize)]
#[serde(default)]
struct UploadFileQuery {
    name: String,
    ctype: String,
}
impl Default for UploadFileQuery {
    fn default() -> Self {
        Self {
            name: "yakir".to_owned(),
            ctype: "text/plain".to_owned(),
        }
    }
}

pub async fn upload_file_route(mut req: Request<AppState>) -> Result {
    let query: UploadFileQuery = req.query()?;

    let bytes = req.body_bytes().await?;
    let byte_stream =
        once(
            async move { std::result::Result::<Bytes, Infallible>::Ok(Bytes::from(bytes)) },
        );

    let content_type = req.content_type().unwrap().to_string();

    let parsed_boundary = multer::parse_boundary(content_type).unwrap();

    let mut multipart = Multipart::new(byte_stream, parsed_boundary);

    while let Some(mut field) = multipart.next_field().await? {
        let file_name = format!("{}", field.file_name().unwrap());

        if let Some(chunk) = field.chunk().await? {
            req.state()
                .files_sender
                .send_async(FileChannelMessage {
                    name: file_name,
                    r#type: query.ctype.clone(),
                    bytes: chunk.to_vec()
                })
                .await
                .unwrap();
        }
    }

    Ok("File uploaded successfully".into())
}

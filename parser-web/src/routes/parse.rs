//! Routes for parsing documents.

use crate::{errors::ApiError, responses::ParseResponse};
use actix_multipart::Multipart;
use actix_web::post;
use futures_util::StreamExt;
use parser_core::InputFiles;

/// Parses various document formats into plain text.
#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<ParseResponse, ApiError> {
    let mut files = Vec::new();

    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;

        // Get filename or use default
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .unwrap_or("file")
            .to_string();

        // Collect data chunks directly into Vec<u8>
        let mut buffer = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk_data = chunk?;
            buffer.extend_from_slice(&chunk_data);
        }

        // Add to files collection (no extra copy needed)
        files.push((buffer, filename));
    }

    if files.is_empty() {
        return Err(ApiError::BadRequest("No files provided".to_string()));
    }

    let parsed_text = InputFiles::with_filenames(files).parse()?;

    Ok(ParseResponse { texts: parsed_text })
}

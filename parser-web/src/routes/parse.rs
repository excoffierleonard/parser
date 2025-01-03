use crate::{errors::ApiError, responses::ParseResponse};
use actix_multipart::Multipart;
use actix_web::post;
use futures_util::StreamExt;
use parser_core::parsers::parse_any;
use std::io::Write;
use tempfile::NamedTempFile;

/// Parses various document formats into plain text.
#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<ParseResponse, ApiError> {
    let temp_file = create_temp_file(&mut payload).await?;
    let parsed_text = parse_any(temp_file.path())?;

    Ok(ParseResponse { text: parsed_text })
}

async fn create_temp_file(payload: &mut Multipart) -> Result<NamedTempFile, ApiError> {
    let mut temp_file = NamedTempFile::new()?;

    // Take the first field from the multipart payload
    let mut field = payload
        .next()
        .await
        .ok_or_else(|| ApiError::BadRequest("No file provided".to_string()))??;

    // Stream chunks directly to the temp file
    while let Some(chunk) = field.next().await {
        temp_file.write_all(&chunk?)?;
    }

    Ok(temp_file)
}

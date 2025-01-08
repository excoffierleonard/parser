//! Routes for parsing documents.

use crate::{errors::ApiError, responses::ParseResponse};
use actix_multipart::Multipart;
use actix_web::post;
use futures_util::StreamExt;
use parser_core::InputFiles;
use std::{io::Write, path::PathBuf};
use tempfile::NamedTempFile;

/// Parses various document formats into plain text.
#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<ParseResponse, ApiError> {
    let temp_files = create_temp_files(&mut payload).await?;

    // Extract paths from all temp files
    let file_paths: Vec<PathBuf> = temp_files
        .iter()
        .map(|temp_file| temp_file.path().to_path_buf())
        .collect();

    let parsed_text = InputFiles::new(file_paths).parse()?;

    Ok(ParseResponse { texts: parsed_text })
}

async fn create_temp_files(payload: &mut Multipart) -> Result<Vec<NamedTempFile>, ApiError> {
    let mut temp_files = Vec::new();

    // Process each field in the payload
    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;
        let mut temp_file = NamedTempFile::new()?;

        // Stream chunks directly to the temp file
        while let Some(chunk) = field.next().await {
            temp_file.write_all(&chunk?)?;
        }

        temp_files.push(temp_file);
    }

    // Check if any files were processed
    if temp_files.is_empty() {
        return Err(ApiError::BadRequest("No files provided".to_string()));
    }

    Ok(temp_files)
}

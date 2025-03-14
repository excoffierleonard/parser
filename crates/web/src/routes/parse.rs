//! Routes for parsing documents.

use crate::{errors::ApiError, responses::ParseResponse};
use actix_multipart::Multipart;
use actix_web::post;
use futures_util::StreamExt;
use parser_core::parse;
use rayon::prelude::*;

/// Parses various document formats into plain text.
#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<ParseResponse, ApiError> {
    let mut files = Vec::new();

    while let Some(field_result) = payload.next().await {
        let mut field = field_result?;

        // Collect data chunks directly into Vec<u8>
        let mut buffer = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk_data = chunk?;
            buffer.extend_from_slice(&chunk_data);
        }

        // Add to files collection
        files.push(buffer);
    }

    if files.is_empty() {
        return Err(ApiError::BadRequest("No files provided".to_string()));
    }

    // Create a vector of slices for processing
    let file_slices: Vec<&[u8]> = files.iter().map(|d| d.as_slice()).collect();

    let parsed_text = file_slices
        .par_iter()
        .map(|d| parse(d))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ParseResponse { texts: parsed_text })
}

//! Routes for parsing documents.

use crate::errors::ApiError;
use actix_multipart::Multipart;
use actix_web::{body::BoxBody, post, HttpRequest, HttpResponse, Responder};
use futures_util::TryStreamExt;
use parser_core::parse;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// Response type for parsed texts
#[derive(Serialize, Deserialize)]
struct ParseResponse {
    /// Parsed text from the documents
    texts: Vec<String>,
}

impl Responder for ParseResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

/// Parses various document formats into plain text.
#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<ParseResponse, ApiError> {
    let mut files = Vec::new();

    // Process each field in the multipart payload
    while let Some(mut field) = payload.try_next().await? {
        // Preallocate buffer with reasonable capacity to reduce allocations
        let mut buffer = Vec::with_capacity(256 * 1024); // 256KB initial capacity
        
        // Stream chunks directly into buffer
        while let Some(chunk) = field.try_next().await? {
            buffer.extend_from_slice(&chunk);
        }

        // Only add non-empty files
        if !buffer.is_empty() {
            files.push(buffer);
        }
    }

    if files.is_empty() {
        return Err(ApiError::BadRequest("No files provided".to_string()));
    }

    // Process files in parallel
    let parsed_text = files
        .par_iter()
        .map(|data| parse(data))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ParseResponse { texts: parsed_text })
}

//! Static files route.

use crate::errors::ApiError;
use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, get, web};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
struct Assets;

/// Response type for serving static assets
#[derive(Serialize)]
pub struct AssetResponse {
    /// Raw binary content of the asset
    pub content: Vec<u8>,
    /// MIME type of the asset (e.g. "text/html", "image/png")
    pub mime_type: String,
}

impl Responder for AssetResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(self.mime_type)
            .body(self.content)
    }
}

/// Serves static files from the `static` folder. Embeds the files into the binary.
#[get("/{filename:.*}")]
async fn serve_files(filename: web::Path<String>) -> Result<AssetResponse, ApiError> {
    let path = if filename.as_str().trim_start_matches('/').is_empty() {
        "index.html"
    } else {
        filename.as_str().trim_start_matches('/')
    };

    Assets::get(path)
        .map(|content| AssetResponse {
            content: content.data.to_vec(),
            mime_type: from_path(path).first_or_octet_stream().to_string(),
        })
        .ok_or_else(|| ApiError::BadRequest("File not found".to_string()))
}

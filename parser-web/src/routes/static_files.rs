use crate::{errors::ApiError, response::AssetResponse};
use actix_web::{get, web};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct Assets;

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

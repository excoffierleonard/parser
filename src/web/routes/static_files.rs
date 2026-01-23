//! Static files route.

use actix_web::{HttpResponse, get, http::StatusCode, web};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets/web"]
struct Assets;

#[get("/{filename:.*}")]
async fn serve_files(filename: web::Path<String>) -> HttpResponse {
    let path = if filename.as_str().trim_start_matches('/').is_empty() {
        "index.html"
    } else {
        filename.as_str().trim_start_matches('/')
    };

    match Assets::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::build(StatusCode::NOT_FOUND).body("Not found"),
    }
}

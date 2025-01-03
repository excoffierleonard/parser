use actix_web::{get, web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct Assets;

#[get("/{filename:.*}")]
async fn serve_files(filename: web::Path<String>) -> impl Responder {
    let path = if filename.as_str().trim_start_matches('/').is_empty() {
        "index.html"
    } else {
        filename.as_str().trim_start_matches('/')
    };

    Assets::get(path)
        .map(|content| {
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream())
                .body(content.data.to_vec())
        })
        .unwrap_or_else(|| HttpResponse::NotFound().finish())
}

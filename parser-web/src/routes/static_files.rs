use actix_web::{get, web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

#[get("/{filename:.*}")]
async fn serve_files(filename: web::Path<String>) -> impl Responder {
    let path = if filename.is_empty() {
        "index.html"
    } else {
        filename.as_str()
    };

    match Assets::get(path) {
        Some(content) => {
            let mime = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime)
                .body(content.data.to_vec())
        }
        None => HttpResponse::NotFound().finish(),
    }
}

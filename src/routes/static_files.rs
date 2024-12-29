use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn serve_index() -> impl Responder {
    let index_html = include_str!("../../static/index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index_html)
}

//! Response types for the parser web server

use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

/// Response type for greeting
#[derive(Serialize)]
pub struct GreetingResponse {
    /// Greeting message
    pub message: String,
}

impl Responder for GreetingResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

/// Response type for parsed text
#[derive(Serialize)]
pub struct ParseResponse {
    /// Parsed text from the document
    pub text: String,
}

impl Responder for ParseResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

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

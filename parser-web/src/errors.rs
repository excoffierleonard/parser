use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    InternalError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };

        match self {
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
            ApiError::InternalError(_) => HttpResponse::InternalServerError().json(error_response),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(msg) | ApiError::InternalError(msg) => write!(f, "{}", msg),
        }
    }
}

// For std::io::Error
impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::InternalError(err.to_string())
    }
}

// For actix_multipart::MultipartError
impl From<actix_multipart::MultipartError> for ApiError {
    fn from(err: actix_multipart::MultipartError) -> Self {
        ApiError::InternalError(err.to_string())
    }
}

// For parser_core::errors::ParserError
impl From<parser_core::errors::ParserError> for ApiError {
    fn from(err: parser_core::errors::ParserError) -> Self {
        ApiError::InternalError(err.to_string())
    }
}

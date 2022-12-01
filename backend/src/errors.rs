use std::fmt::{Display, Formatter};

use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Debug, Serialize)]
pub struct ApiErrorResponseBody {
    pub success: bool,
    pub error_code: u16,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub enum ApiErrorResponse {
    NotFound,
    ServiceUnavailable,
}

impl Display for ApiErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl actix_web::error::ResponseError for ApiErrorResponse {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiErrorResponse::NotFound => StatusCode::NOT_FOUND,
            ApiErrorResponse::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiErrorResponse::NotFound => {
                HttpResponse::build(self.status_code()).json(ApiErrorResponseBody {
                    success: false,
                    error_code: 100,
                    message: String::from("url not found"),
                })
            }
            ApiErrorResponse::ServiceUnavailable => {
                HttpResponse::build(self.status_code()).json(ApiErrorResponseBody {
                    success: false,
                    error_code: 100,
                    message: String::from("Connection failed. Try later."),
                })
            }
        }
    }
}

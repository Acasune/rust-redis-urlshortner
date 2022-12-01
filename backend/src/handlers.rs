use actix_web::{http::StatusCode, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::errors::ApiErrorResponse;

use crate::responses::{
    ResponseBodyDeleteUrl, ResponseBodyGetUrl, ResponseBodyInit, ResponseBodyPostUrl,
};
use crate::services::UrlShortenerService;

pub async fn index(
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse, ApiErrorResponse> {
    let result = url_shortener_service.get_all_urls().await;
    match result {
        Ok(all_urls) => Ok(HttpResponse::Ok().json(ResponseBodyInit { urls: all_urls })),
        _ => Err(ApiErrorResponse::NotFound),
    }
}

pub async fn get_url(
    hashed: web::Path<String>,
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse, ApiErrorResponse> {
    let result = url_shortener_service.get_url(hashed.into_inner()).await;
    match result {
        Ok(url) => {
            Ok(HttpResponse::build(StatusCode::SEE_OTHER).json(ResponseBodyGetUrl { url: url }))
        }
        _ => Err(ApiErrorResponse::NotFound),
    }
}

pub async fn post_url(
    data: web::Json<RequestBody>,
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse, ApiErrorResponse> {
    let result = url_shortener_service.post_url(data.0.url).await;
    match result {
        Ok(hashed) => Ok(
            HttpResponse::build(StatusCode::OK).json(ResponseBodyPostUrl { hashed: hashed })
        ),

        _ => Err(ApiErrorResponse::ServiceUnavailable),
    }
}

pub async fn delete_url(
    hashed: web::Path<String>,
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse, ApiErrorResponse> {
    let result = url_shortener_service
        .delete_url(hashed.to_string())
        .await;
    match result {
        Ok(()) => Ok(
            HttpResponse::build(StatusCode::OK).json(ResponseBodyDeleteUrl {
                hashed: hashed.to_string(),
            }),
        ),
        _ => Err(ApiErrorResponse::NotFound),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    user_name: String,
    url: String,
}

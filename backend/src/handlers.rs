use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::services::UrlShortenerService;

pub async fn index(
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse> {
    let all_urls = url_shortener_service
        .get_all_urls()
        .await
        .expect("connection failed");
    Ok(HttpResponse::Ok().json(ResponseBodyInit { urls: all_urls }))
}

pub async fn get_url(
    hashed_url: web::Path<String>,
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse> {
    let raw_url = url_shortener_service
        .get_url(hashed_url.into_inner())
        .await
        .expect("connection failed");
    Ok(HttpResponse::Ok().json(ResponseBody { url: raw_url }))
}

pub async fn post_url(
    data: web::Json<PostData>,
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse> {
    let hashed = url_shortener_service
        .post_url(data.0.url)
        .await
        .expect("connection failed");
    Ok(HttpResponse::Ok().json(ResponseBody { url: hashed }))
}

pub async fn delete_url(
    hashed_url: web::Path<String>,
    url_shortener_service: web::Data<UrlShortenerService>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let _result = url_shortener_service
        .delete_url(hashed_url.to_string())
        .await;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    user_name: String,
    url: String,
}

#[derive(Serialize)]
pub struct ResponseBodyInit {
    urls: Vec<String>,
}
#[derive(Serialize)]
pub struct ResponseBody {
    url: String,
}

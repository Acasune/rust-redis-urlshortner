use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::services::UrlShortenerService;

pub async fn index() -> &'static str {
    // Result<HttpResponse, actix_web::Error> {
    // print!("hello");
    // let response_body = "Hello world!!";
    // Ok(HttpResponse::Ok().body(response_body))
    "Hello world!!"
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
    let result = url_shortener_service
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
pub struct ResponseBody {
    url: String,
}

﻿use actix_web::{web, HttpRequest, HttpResponse, Responder};
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};

pub async fn index() -> &'static str {
    // Result<HttpResponse, actix_web::Error> {
    // print!("hello");
    // let response_body = "Hello world!!";
    // Ok(HttpResponse::Ok().body(response_body))
    "Hello world!!"
}

pub async fn get_url(hashed_url: web::Path<String>) -> impl Responder {
    let res = format!("{}", hashed_url.as_str());
    HttpResponse::Ok().body(res)
}

pub async fn post_url(data: web::Json<PostData>) -> impl Responder {
    let hashed = Base64::encode_string(data.0.url.as_bytes());
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    HttpResponse::Ok().body(hashed)
}

// #[delete("/{hashed_url}")]
// async fn delete_url(hashed_url: web::Path<String>) -> impl Responder {
//     let res = format!("{}", hashed_url.as_str());
//     HttpResponse::Ok().body(res)
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    user_name: String,
    url: String,
}
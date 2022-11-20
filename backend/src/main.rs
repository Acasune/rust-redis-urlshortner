use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(get_url).service(post_url))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

#[get("/{hashed_url}")]
async fn get_url(hashed_url: web::Path<String>) -> impl Responder {
    let res = format!("{}", hashed_url.as_str());
    HttpResponse::Ok().body(res)
}

#[post("/")]
async fn post_url(data: web::Json<PostData>) -> impl Responder {
    let hashed = Base64::encode_string(data.0.url.as_bytes());
    HttpResponse::Ok().body(hashed)
}

// #[delete("/{hashed_url}")]
// async fn delete_url(hashed_url: web::Path<String>) -> impl Responder {
//     let res = format!("{}", hashed_url.as_str());
//     HttpResponse::Ok().body(res)
// }

#[derive(Debug, Serialize, Deserialize)]
struct PostData {
    user_name: String,
    url: String,
}

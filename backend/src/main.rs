use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index).service(post_url))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

#[post("/")]
async fn post_url(data: web::Json<PostData>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(data.0.user_name)
}

#[derive(Debug, Serialize, Deserialize)]
struct PostData {
    user_name: String,
    url: String,
}

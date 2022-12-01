use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use redis::Client;

const REDIS_URI: &str = "redis://redis:6379";

mod errors;
mod handlers;
mod responses;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let redis_client = Client::open(REDIS_URI).expect("Can't create Redis client");
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Can't create Redis connection manager");
    let url_shortener_services = Data::new(services::UrlShortenerService::new(
        redis_client,
        redis_connection_manager.clone(),
    ));
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/{hashed_url}", web::get().to(handlers::get_url))
            .route("/", web::post().to(handlers::post_url))
            .route("/{hashed_url}", web::delete().to(handlers::delete_url))
            .app_data(url_shortener_services.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use redis::aio::ConnectionManager;
use redis::Client;
const REDIS_CON_STRING: &str = "redis://127.0.0.1/";

mod handlers;

#[derive(Clone)]
pub struct Clients {
    redis_client: Client,
    redis_connection_manager: ConnectionManager,
}

impl Clients {
    pub fn new(redis_client: Client, redis_connection_manager: ConnectionManager) -> Self {
        Clients {
            redis_client,
            redis_connection_manager,
        }
    }
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // let redis_uri = "http://127.0.0.1:6379";
    // let redis_client = Client::open(redis_uri).expect("Can't create Redis client");
    // let redis_connection_manager = redis_client
    //     .get_tokio_connection_manager()
    //     .await
    //     .expect("Can't create Redis connection manager");
    // let clients = Data::new(Clients::new(redis_client, redis_connection_manager.clone()));
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/{hashed_url}", web::get().to(handlers::get_url))
            .route("/", web::post().to(handlers::post_url))
        // .app_data(clients.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}

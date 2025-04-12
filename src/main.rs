mod routes;
mod proxy;

use actix_web::{get, App, HttpServer, Responder};
use kite::{create_classify_url_response, get_config, kite::classify_url::{self, classify_url_response::DisplayClassification}};

use routes::classification::classify_url_service;

#[get("/")]
async fn index() -> impl Responder {
    "Kite Server"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config();

    if config.proxy.enabled {
        // Spawn a new thread to handle the HTTP proxy
        tokio::spawn(async {
            proxy::proxy::start_proxy(get_config()).await;
        });
    }

    println!("🪁 Kite is running on http://{}:{}", config.server.ip, config.server.port);

    HttpServer::new(|| App::new().service(index).service(classify_url_service))
        .bind((config.server.ip, config.server.port))?
        .run()
        .await
}
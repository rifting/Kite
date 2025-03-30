use std::string::ParseError;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_protobuf::*;
use kite::{create_classify_url_response, kite::classify_url::{self, classify_url_response::{self, DisplayClassification}}};
use url::{Url};

#[get("/")]
async fn index() -> impl Responder {
    "Kite Server"
}

#[post("/kidsmanagement/v1/people/me:classifyUrl")]
async fn classify_url_service(msg: ProtoBuf<classify_url::ClassifyUrlRequest>) -> impl Responder {
    match Url::parse(msg.url()) {
        Ok(url) => {
            HttpResponse::Ok().protobuf(create_classify_url_response(
                classify_host(url).await
            ))
        }
        Err(_) => {
            HttpResponse::Ok().protobuf(create_classify_url_response(
                DisplayClassification::UnknownDisplayClassification,
            ))
        }
    }
}

async fn classify_host(url: Url) -> DisplayClassification {
    if url.host_str() == Some("example.com") {
        return DisplayClassification::Restricted;
    } else {
        return DisplayClassification::Allowed;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(classify_url_service))
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
}
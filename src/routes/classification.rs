use actix_web::{post, HttpResponse, Responder};
use actix_protobuf::*;
use url::Url;

use crate::create_classify_url_response;
use crate::classify_url;
use crate::get_config;
use crate::DisplayClassification;

#[post("/kidsmanagement/v1/people/me:classifyUrl")]
pub async fn classify_url_service(msg: ProtoBuf<classify_url::ClassifyUrlRequest>) -> impl Responder {
    // Debug line
    // println!("{:?}", msg);
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
    let mode = get_config().blocking.mode.to_lowercase();

    match mode.as_str() {
        "example" => {
            match url.host_str() {
                Some("example.com") => DisplayClassification::Restricted,
                _ => DisplayClassification::Allowed,
            }
        }
        "all" => DisplayClassification::Restricted,
        _ => DisplayClassification::Allowed,
    }
}

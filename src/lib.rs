use kite::classify_url::{self, classify_url_response::DisplayClassification, TimestampWrapper};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod kite {
    pub mod classify_url {
        include!(concat!(env!("OUT_DIR"), "/kite.classify_url.rs"));
    }
}

pub fn create_classify_url_response(classification: DisplayClassification) -> classify_url::ClassifyUrlResponse {
    let mut res = classify_url::ClassifyUrlResponse::default();
    let mut timestamp_wrapper = classify_url::TimestampWrapper::default();


    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    timestamp_wrapper.timestamp = since_the_epoch.as_millis() as u64;

    res.timestamp = Some(timestamp_wrapper);
    res.display_classification = Some(classification.into());
    res
}
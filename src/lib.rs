use kite::classify_url::{self, classify_url_response::DisplayClassification};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_derive::Deserialize;
use std::process::exit;
use toml;
use clap::Parser;
use std::{fs, path::PathBuf};

pub mod routes;

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

/// A server reimplementation of a particular parental control service.
#[derive(Parser, Debug)]
struct Cli {
    /// Path to the configuration file
    #[arg(long)]
    config: PathBuf,
}

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
pub struct Data {
    pub server: Server,
    pub proxy: Proxy,
    pub blocking: Blocking
}

#[derive(Deserialize)]
pub struct Server {
    pub port: u16,
    pub ip: String
}

#[derive(Deserialize)]
pub struct Proxy {
    pub enabled: bool,
    pub port: u16,
    pub ip: String,
    pub cert: String,
    pub private_key: String
}

#[derive(Deserialize)]
pub struct Blocking {
    pub mode: String
}

pub fn get_config() -> Data {
    let args = Cli::parse();
    let config_path = args.config;
    let contents = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", config_path.display());
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", config_path.display());
            exit(1);
        }
    };

    data
}
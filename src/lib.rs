extern crate pest;
#[macro_use]
extern crate pest_derive;

use data_types::map::MapItem;
use data_types::vector::VectorItem;
use tracing::{error, info, warn};

use crate::config::Config;
use crate::data_types::item::Item;
use crate::data_types::primitives::link::Link;
use crate::data_types::primitives::path::PathToValue;
use crate::data_types::primitives::string::StringPrimitive;
use crate::data_types::primitives::Primitive;
use crate::errors::DBError;
use crate::storage::main::Storage;
use crate::tyson::de::Desereilize;
use crate::tyson::map::TySONMap;
use crate::tyson::primitive::TySONPrimitive;
use crate::tyson::vector::TySONVector;
use storage::transaction::Transaction;

mod config;
mod constants;
pub mod data_types;
mod errors;
pub mod query;
pub mod response;
pub mod storage;
pub mod tyson;

pub fn get_storage(path: &str) -> Result<Storage, DBError> {
    Storage::new(path)
}

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("AnnaDB starting");
    let config = Config::new();

    let mut storage = match Storage::new(&config.wh_path) {
        Ok(s) => {
            info!("storage initialized");
            s
        }
        Err(e) => {
            error!(error = %e, "failed to initialize storage");
            std::process::exit(1);
        }
    };

    let context = zmq::Context::new();
    let responder = match context.socket(zmq::REP) {
        Ok(s) => s,
        Err(e) => {
            error!(error = %e, "failed to create ZMQ socket");
            std::process::exit(1);
        }
    };

    let bind_addr = format!("tcp://0.0.0.0:{}", config.port);
    if responder.bind(bind_addr.as_str()).is_err() {
        error!(port = %config.port, "failed to bind to port");
        std::process::exit(1);
    }

    info!(port = %config.port, "AnnaDB listening");

    let mut msg = zmq::Message::new();
    loop {
        match responder.recv(&mut msg, 0) {
            Ok(_) => match msg.as_str() {
                Some(msg_value) => {
                    let start = std::time::Instant::now();
                    let res = storage.run(msg_value);
                    let duration = start.elapsed();
                    tracing::debug!(
                        duration_ms = duration.as_millis() as u64,
                        "transaction processed"
                    );
                    if let Err(e) = responder.send(res.as_str(), 0) {
                        error!(error = %e, "failed to send response");
                    }
                }
                None => {
                    warn!("received non-utf8 message");
                }
            },
            Err(e) => {
                error!(error = %e, "failed to receive message");
                if let Err(e) = responder.send("Receiving problem", 0) {
                    error!(error = %e, "failed to send error response");
                }
            }
        }
    }
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

use tracing::{error, info};

use crate::config::Config;
use crate::errors::DBError;

mod config;
mod constants;
pub mod data_types;
pub mod embedding;
mod errors;
pub mod query;
pub mod response;
mod server;
pub mod storage;
pub mod tyson;

// Re-exports for convenience
pub use crate::data_types::item::Item;
pub use crate::data_types::map::MapItem;
pub use crate::data_types::primitives::link::Link;
pub use crate::data_types::primitives::path::PathToValue;
pub use crate::data_types::primitives::string::StringPrimitive;
pub use crate::data_types::primitives::Primitive;
pub use crate::data_types::vector::VectorItem;
pub use crate::errors::DBError as DbError;
pub use crate::storage::main::Storage;
pub use crate::storage::transaction::Transaction;
pub use crate::tyson::de::Desereilize;
pub use crate::tyson::map::TySONMap;
pub use crate::tyson::primitive::TySONPrimitive;
pub use crate::tyson::vector::TySONVector;

/// Open an AnnaDB instance at the given path. Use ":memory:" for a
/// temporary in-memory database.
///
/// # Examples
/// ```ignore
/// let mut db = AnnaDB::open("~/.annadb/memory")?;
/// db.remember("facts", "hello", None)?;
/// ```
pub fn open(path: &str, embedding_provider: Option<Box<dyn embedding::EmbeddingProvider>>) -> Result<Storage, DBError> {
    if path == ":memory:" {
        let dir = std::env::temp_dir().join(format!("annadb_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir)?;
        Storage::new(dir.to_str().unwrap(), embedding_provider)
    } else {
        Storage::new(path, embedding_provider)
    }
}

/// Start the HTTP server. Listens on the configured port and serves
/// TySON transactions via POST /tx.
pub fn serve(storage: &mut Storage, port: u16) {
    server::serve(storage, port)
}

/// Run AnnaDB as a standalone server. Reads config from environment
/// variables and starts the HTTP listener.
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("AnnaDB starting");
    let config = Config::new();

    let embedding_provider = match config.build_embedding_provider() {
        Ok(Some(p)) => {
            info!(dims = p.dimensions(), "embedding provider configured");
            Some(p)
        }
        Ok(None) => None,
        Err(e) => {
            error!(error = %e, "failed to configure embedding provider");
            std::process::exit(1);
        }
    };

    let mut storage = match Storage::new(&config.wh_path, embedding_provider) {
        Ok(s) => {
            info!(path = %config.wh_path, "storage initialized");
            s
        }
        Err(e) => {
            error!(error = %e, "failed to initialize storage");
            std::process::exit(1);
        }
    };

    let port: u16 = config.port.parse().unwrap_or(10001);
    server::serve(&mut storage, port);
}

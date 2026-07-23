use std::env;

use crate::DBError;

pub struct Config {
    pub(crate) port: String,
    pub(crate) wh_path: String,
    /// Optional embedding provider: ("openai", model_name)
    pub(crate) embedding_provider: Option<(String, String)>,
}

impl Config {
    pub fn new() -> Self {
        let port = match env::var("PORT") {
            Ok(v) => v.to_string(),
            Err(_) => "10001".to_string(),
        };
        let wh_path = match env::var("WH_PATH") {
            Ok(v) => v.to_string(),
            Err(_) => "warehouse".to_string(),
        };
        let embedding_provider = match env::var("EMBEDDING_PROVIDER") {
            Ok(v) => match env::var("EMBEDDING_MODEL") {
                Ok(model) => Some((v, model)),
                Err(_) => None,
            },
            Err(_) => None,
        };
        Self {
            port,
            wh_path,
            embedding_provider,
        }
    }

    /// Build the configured embedding provider.
    /// API key is read from environment variable at construction time.
    pub fn build_embedding_provider(
        &self,
    ) -> Result<Option<Box<dyn crate::embedding::EmbeddingProvider>>, DBError> {
        match &self.embedding_provider {
            Some((provider, model)) => match provider.as_str() {
                "openai" => {
                    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
                        DBError::UnsupportedOperation(
                            "OPENAI_API_KEY environment variable not set".to_string(),
                        )
                    })?;
                    Ok(Some(Box::new(
                        crate::embedding::openai::OpenAiProvider::new(api_key, model.clone()),
                    )))
                }
                #[cfg(feature = "embedding-local")]
                "local" => Ok(Some(Box::new(
                    crate::embedding::local::LocalEmbeddingProvider::new()?,
                ))),
                _ => Err(DBError::UnsupportedOperation(format!(
                    "unknown embedding provider: {}",
                    provider
                ))),
            },
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn config_defaults() {
        let _lock = env_lock();
        std::env::remove_var("PORT");
        std::env::remove_var("WH_PATH");
        std::env::remove_var("EMBEDDING_PROVIDER");
        std::env::remove_var("EMBEDDING_MODEL");

        let c = Config::new();
        assert_eq!(c.port, "10001");
        assert_eq!(c.wh_path, "warehouse");
        assert!(c.embedding_provider.is_none());
    }

    #[test]
    fn config_custom_port() {
        let _lock = env_lock();
        std::env::set_var("PORT", "9999");
        let c = Config::new();
        assert_eq!(c.port, "9999");
        std::env::remove_var("PORT");
    }
}

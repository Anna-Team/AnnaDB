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
                _ => Err(DBError::UnsupportedOperation(format!(
                    "unknown embedding provider: {}",
                    provider
                ))),
            },
            None => Ok(None),
        }
    }
}

use crate::DBError;

pub mod openai;
#[cfg(feature = "embedding-local")]
pub mod local;

/// A third-party service that converts text into vector embeddings.
pub trait EmbeddingProvider: Send + Sync {
    /// Generate an embedding vector for the given text.
    fn embed(&self, text: &str) -> Result<Vec<f32>, DBError>;

    /// Number of dimensions in the generated embedding.
    fn dimensions(&self) -> u16;
}

/// No-op provider for when embeddings are not configured.
pub struct NoopProvider;

impl EmbeddingProvider for NoopProvider {
    fn embed(&self, _text: &str) -> Result<Vec<f32>, DBError> {
        Err(DBError::UnsupportedOperation(
            "no embedding provider configured".to_string(),
        ))
    }

    fn dimensions(&self) -> u16 {
        0
    }
}

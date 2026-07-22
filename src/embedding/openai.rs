use serde::{Deserialize, Serialize};

use crate::embedding::EmbeddingProvider;
use crate::DBError;

pub struct OpenAiProvider {
    api_key: String,
    model: String,
    dimensions: u16,
}

#[derive(Serialize)]
struct EmbeddingRequest {
    input: String,
    model: String,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        let dimensions = match model.as_str() {
            "text-embedding-3-small" => 1536,
            "text-embedding-3-large" => 3072,
            "text-embedding-ada-002" => 1536,
            _ => 1536,
        };
        Self {
            api_key,
            model,
            dimensions,
        }
    }
}

impl EmbeddingProvider for OpenAiProvider {
    fn embed(&self, text: &str) -> Result<Vec<f32>, DBError> {
        let request = EmbeddingRequest {
            input: text.to_string(),
            model: self.model.clone(),
        };

        let response: EmbeddingResponse = ureq::post("https://api.openai.com/v1/embeddings")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_json(&request)
            .map_err(|e| {
                DBError::UnsupportedOperation(format!("OpenAI API error: {}", e))
            })?
            .into_json()
            .map_err(|e| {
                DBError::UnsupportedOperation(format!("OpenAI parse error: {}", e))
            })?;

        response
            .data
            .into_iter()
            .next()
            .map(|d| d.embedding)
            .ok_or_else(|| {
                DBError::UnsupportedOperation("OpenAI returned empty embedding".to_string())
            })
    }

    fn dimensions(&self) -> u16 {
        self.dimensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openai_provider_dimensions() {
        let provider = OpenAiProvider::new("test-key".to_string(), "text-embedding-3-small".to_string());
        assert_eq!(provider.dimensions(), 1536);
    }

    #[test]
    fn openai_provider_large_dims() {
        let provider = OpenAiProvider::new("test-key".to_string(), "text-embedding-3-large".to_string());
        assert_eq!(provider.dimensions(), 3072);
    }
}

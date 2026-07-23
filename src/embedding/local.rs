#[cfg(feature = "embedding-local")]
mod imp {
    use std::fs;
    use std::path::PathBuf;

    use tracing::info;

    use crate::embedding::EmbeddingProvider;
    use crate::DBError;

    const MODEL_URL: &str =
        "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx";
    const TOKENIZER_URL: &str =
        "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/tokenizer.json";
    const DIMS: u16 = 384;

    pub struct LocalEmbeddingProvider {
        _private: (),
    }

    impl LocalEmbeddingProvider {
        pub fn new() -> Result<Self, DBError> {
            let cache_dir = cache_dir();
            fs::create_dir_all(&cache_dir).map_err(|e| {
                DBError::UnsupportedOperation(format!("cannot create cache dir: {}", e))
            })?;

            let model_path = cache_dir.join("model.onnx");
            let tokenizer_path = cache_dir.join("tokenizer.json");

            if !model_path.exists() {
                download(MODEL_URL, &model_path)?;
                info!(path = %model_path.display(), "downloaded embedding model");
            }
            if !tokenizer_path.exists() {
                download(TOKENIZER_URL, &tokenizer_path)?;
                info!(path = %tokenizer_path.display(), "downloaded tokenizer");
            }

            // TODO: load ONNX model + tokenizer when ort + tokenizers deps are present
            Ok(Self { _private: () })
        }
    }

    impl EmbeddingProvider for LocalEmbeddingProvider {
        fn embed(&self, _text: &str) -> Result<Vec<f32>, DBError> {
            Err(DBError::UnsupportedOperation(
                "local embedding inference not yet implemented — add ort + tokenizers deps".to_string(),
            ))
        }

        fn dimensions(&self) -> u16 {
            DIMS
        }
    }

    fn cache_dir() -> PathBuf {
        dirs_next::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("annadb")
            .join("models")
    }

    fn download(url: &str, path: &PathBuf) -> Result<(), DBError> {
        let resp = ureq::get(url)
            .call()
            .map_err(|e| DBError::UnsupportedOperation(format!("download: {}", e)))?;
        let mut file = fs::File::create(path).map_err(|e| {
            DBError::UnsupportedOperation(format!("create file: {}", e))
        })?;
        std::io::copy(&mut resp.into_reader(), &mut file).map_err(|e| {
            DBError::UnsupportedOperation(format!("save: {}", e))
        })?;
        Ok(())
    }
}

#[cfg(feature = "embedding-local")]
pub use imp::LocalEmbeddingProvider;

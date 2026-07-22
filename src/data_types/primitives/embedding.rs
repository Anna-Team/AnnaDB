use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::constants::EMBEDDING;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingPrimitive {
    dims: u16,
    values: Vec<f32>,
}

// Eq: embeddings from ML models never contain NaN; bitwise equality via PartialEq is sound.
impl Eq for EmbeddingPrimitive {}

// PartialOrd: compare dimensionally, lexicographically on values.
impl PartialOrd for EmbeddingPrimitive {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.dims != other.dims {
            return self.dims.partial_cmp(&other.dims);
        }
        for (a, b) in self.values.iter().zip(other.values.iter()) {
            match a.partial_cmp(b) {
                Some(std::cmp::Ordering::Equal) => continue,
                other => return other,
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl EmbeddingPrimitive {
    pub fn new(dims: u16, values: Vec<f32>) -> Self {
        Self { dims, values }
    }

    pub fn dims(&self) -> u16 {
        self.dims
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }
}

impl BaseTySONItemInterface for EmbeddingPrimitive {
    fn get_prefix(&self) -> String {
        EMBEDDING.to_string()
    }
}

impl TySONPrimitive for EmbeddingPrimitive {
    fn new(_prefix: String, value: String) -> Result<Self, DBError> {
        let parts: Vec<&str> = value.splitn(2, '|').collect();
        if parts.len() != 2 {
            return Err(DBError::Deserialization("embedding missing dimension separator".to_string()));
        }
        let dims: u16 = parts[0]
            .parse()
            .map_err(|_| DBError::Deserialization("embedding dimension parse error".to_string()))?;
        let values: Result<Vec<f32>, _> = parts[1]
            .split(',')
            .map(|s| s.trim().parse::<f32>())
            .collect();
        let values = values.map_err(|_| DBError::Deserialization("embedding values parse error".to_string()))?;
        if values.len() != dims as usize {
            return Err(DBError::Deserialization("embedding dimension mismatch".to_string()));
        }
        Ok(Self { dims, values })
    }

    fn get_string_value(&self) -> String {
        format!(
            "{}|{}",
            self.dims,
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

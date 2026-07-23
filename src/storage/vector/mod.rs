
use crate::data_types::primitives::link::Link;
use crate::storage::vector::hnsw::{HnswIndex, HnswMetric};

pub mod distance;
pub mod hnsw;

#[derive(Debug)]
pub struct VectorIndex {
    pub field_path: String,
    pub dims: u16,
    pub hnsw: HnswIndex,
    pub links: Vec<Link>,
    pub metric: HnswMetric,
}

impl VectorIndex {
    pub fn new(field_path: String, dims: u16, m: usize, ef_construction: usize, metric: HnswMetric) -> Self {
        Self {
            field_path,
            dims,
            hnsw: HnswIndex::new(dims, m, ef_construction, metric),
            links: Vec::new(),
            metric,
        }
    }

    pub fn insert(&mut self, embedding: &[f32], link: Link) -> usize {
        let id = self.hnsw.insert(embedding);
        while self.links.len() <= id {
            self.links.push(Link::create("_placeholder".to_string()));
        }
        self.links[id] = link;
        id
    }

    pub fn remove(&mut self, link: &Link) {
        for (i, l) in self.links.iter().enumerate() {
            if l == link {
                self.hnsw.remove(i);
                return;
            }
        }
    }

    pub fn search(&self, query: &[f32], k: usize) -> Vec<Link> {
        let ids = self.hnsw.search(query, k);
        ids.into_iter().map(|id| self.links[id].clone()).collect()
    }

    pub fn lookup_by_op(&self, _op: &super::index::CompareOp, _key: &super::index::IndexKey) -> Vec<Link> {
        vec![]
    }

    pub fn all_links(&self) -> Vec<Link> {
        self.links.clone()
    }

    pub fn len(&self) -> usize {
        self.hnsw.len()
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, crate::DBError> {
        bincode::serialize(&(&self.hnsw, &self.links, &self.metric)).map_err(|e| {
            crate::DBError::UnsupportedOperation(format!("vector index serialize error: {}", e))
        })
    }

    pub fn from_bytes(field_path: String, dims: u16, data: &[u8]) -> Result<Self, crate::DBError> {
        let (hnsw, links, metric): (HnswIndex, Vec<Link>, HnswMetric) =
            bincode::deserialize(data).map_err(|e| {
                crate::DBError::UnsupportedOperation(format!("vector index deserialize error: {}", e))
            })?;
        Ok(Self {
            field_path,
            dims,
            hnsw,
            links,
            metric,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::vector::hnsw::HnswMetric;

    #[test]
    fn vector_index_new() {
        let idx = VectorIndex::new("emb".to_string(), 3, 16, 200, HnswMetric::Cosine);
        assert_eq!(idx.field_path, "emb");
        assert_eq!(idx.dims, 3);
        assert_eq!(idx.len(), 0);
    }

    #[test]
    fn vector_index_insert_and_search() {
        let mut idx = VectorIndex::new("emb".to_string(), 3, 32, 200, HnswMetric::Cosine);
        let link = Link::create("test".to_string());
        let emb = vec![1.0f32, 0.0, 0.0];
        idx.insert(&emb, link.clone());
        let results = idx.search(&emb, 5);
        assert!(!results.is_empty());
    }

    #[test]
    fn vector_index_remove() {
        let mut idx = VectorIndex::new("emb".to_string(), 3, 32, 200, HnswMetric::Cosine);
        let link = Link::create("test".to_string());
        let emb = vec![1.0f32, 0.0, 0.0];
        idx.insert(&emb, link.clone());
        idx.remove(&link);
        let results = idx.search(&emb, 5);
        assert!(results.is_empty());
    }

    #[test]
    fn vector_index_all_links() {
        let mut idx = VectorIndex::new("emb".to_string(), 3, 32, 200, HnswMetric::Cosine);
        let link1 = Link::create("test".to_string());
        let link2 = Link::create("test".to_string());
        idx.insert(&[1.0, 0.0, 0.0], link1.clone());
        idx.insert(&[0.0, 1.0, 0.0], link2.clone());
        assert_eq!(idx.all_links().len(), 2);
    }

    #[test]
    fn vector_index_to_from_bytes_roundtrip() {
        let mut idx = VectorIndex::new("emb".to_string(), 3, 32, 200, HnswMetric::Cosine);
        let link = Link::create("test".to_string());
        idx.insert(&[1.0, 0.0, 0.0], link);
        let bytes = idx.to_bytes().unwrap();
        let restored = VectorIndex::from_bytes("emb".to_string(), 3, &bytes).unwrap();
        assert_eq!(restored.dims, 3);
    }

    #[test]
    fn vector_index_lookup_by_op() {
        let idx = VectorIndex::new("emb".to_string(), 3, 32, 200, HnswMetric::Cosine);
        assert!(idx.lookup_by_op(&crate::storage::index::CompareOp::Eq, &crate::storage::index::IndexKey::Null).is_empty());
    }
}

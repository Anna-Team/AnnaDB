
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

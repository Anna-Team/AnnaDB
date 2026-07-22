use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::distance::DistanceMetric;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HnswNode {
    id: usize,
    vector: Vec<f32>,
    layers: Vec<Vec<usize>>,
    deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnswIndex {
    pub dims: u16,
    pub M: usize,
    pub M_max: usize,
    pub ef_construction: usize,
    pub ef_search: usize,
    pub ml: f64,
    pub entry_point: Option<usize>,
    pub max_layer: usize,
    nodes: Vec<HnswNode>,
    deleted_count: usize,
    metric_tag: u8,
}

struct Candidate {
    id: usize,
    distance: f32,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HnswMetric {
    Cosine = 0,
    Euclidean = 1,
    DotProduct = 2,
}

impl HnswIndex {
    pub fn new(dims: u16, M: usize, ef_construction: usize, metric: HnswMetric) -> Self {
        let ml = 1.0 / (M as f64).ln();
        Self {
            dims,
            M,
            M_max: M * 2,
            ef_construction: ef_construction.max(M),
            ef_search: ef_construction,
            ml,
            entry_point: None,
            max_layer: 0,
            nodes: Vec::new(),
            deleted_count: 0,
            metric_tag: metric as u8,
        }
    }

    pub fn set_ef_search(&mut self, ef: usize) {
        self.ef_search = ef.max(1);
    }

    pub fn len(&self) -> usize {
        self.nodes.len() - self.deleted_count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, vector: &[f32]) -> usize {
        let id = self.nodes.len();
        let level = self.random_level();
        let mut node = HnswNode {
            id,
            vector: vector.to_vec(),
            layers: vec![Vec::new(); level + 1],
            deleted: false,
        };

        if self.entry_point.is_none() {
            self.nodes.push(node);
            self.entry_point = Some(0);
            self.max_layer = level;
            return id;
        }

        let mut ep = self.entry_point.unwrap();
        let mut current_layer = self.max_layer;

        // Greedy descent from top layer to level+1
        while current_layer > level {
            let nearest = self.search_layer(vector, ep, 1, current_layer);
            if let Some(first) = nearest.first() {
                ep = first.id;
            }
            current_layer -= 1;
        }

        let old_max_layer = self.max_layer;

        // Add node to storage first so neighbors can reference it
        self.nodes.push(node);

        if level > self.max_layer {
            for n in self.nodes.iter_mut() {
                while n.layers.len() <= level {
                    n.layers.push(Vec::new());
                }
            }
            self.max_layer = level;
            self.entry_point = Some(id);
        }

        // Connect at existing layers only
        let connect_up_to = level.min(old_max_layer);
        for lc in (0..=connect_up_to).rev() {
            let candidates = self.search_layer(vector, ep, self.ef_construction, lc);
            let m_max = if lc == 0 { self.M_max } else { self.M };
            let neighbors = self.select_neighbors_simple(&candidates, m_max);

            for &nb in &neighbors {
                self.nodes[nb].layers[lc].push(id);
                if self.nodes[nb].layers[lc].len() > m_max {
                    self.prune_neighbors(nb, lc, m_max);
                }
            }
            self.nodes[id].layers[lc] = neighbors;
            if let Some(first) = candidates.first() {
                ep = first.id;
            }
        }

        id
    }

    pub fn search(&self, query: &[f32], k: usize) -> Vec<usize> {
        if self.entry_point.is_none() || k == 0 {
            return Vec::new();
        }

        let mut ep = self.entry_point.unwrap();
        let mut current_layer = self.max_layer;

        while current_layer > 0 {
            let nearest = self.search_layer(query, ep, 1, current_layer);
            if let Some(first) = nearest.first() {
                ep = first.id;
            }
            current_layer -= 1;
        }

        let candidates = self.search_layer(query, ep, self.ef_search.max(k), 0);
        candidates
            .into_iter()
            .filter(|c| !self.nodes[c.id].deleted)
            .take(k)
            .map(|c| c.id)
            .collect()
    }

    pub fn remove(&mut self, id: usize) {
        if id < self.nodes.len() {
            self.nodes[id].deleted = true;
            self.deleted_count += 1;
        }
    }

    fn random_level(&self) -> usize {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen();
        (-(r.ln()) * self.ml) as usize
    }

    fn search_layer(&self, query: &[f32], ep: usize, ef: usize, layer: usize) -> Vec<Candidate> {
        let visited: HashSet<usize> = HashSet::new();
        let mut visited = visited;
        let mut candidates = BinaryHeap::new();
        let mut results = BinaryHeap::new();

        let d = self.distance(query, &self.nodes[ep].vector);
        candidates.push(Candidate { id: ep, distance: d });
        results.push(Candidate { id: ep, distance: d });
        visited.insert(ep);

        while let Some(current) = candidates.pop() {
            let worst_result = results.peek().map(|c| c.distance).unwrap_or(f32::INFINITY);
            if current.distance > worst_result {
                break;
            }

            if let Some(layer_neighbors) = self.nodes[current.id].layers.get(layer) {
                for &nb in layer_neighbors {
                    if visited.contains(&nb) {
                        continue;
                    }
                    visited.insert(nb);
                    let dist = self.distance(query, &self.nodes[nb].vector);
                    if results.len() < ef || dist < worst_result {
                        candidates.push(Candidate { id: nb, distance: dist });
                        results.push(Candidate { id: nb, distance: dist });
                        if results.len() > ef {
                            results.pop();
                        }
                    }
                }
            }
        }

        results.into_sorted_vec()
    }

    fn select_neighbors_simple(&self, candidates: &[Candidate], m: usize) -> Vec<usize> {
        candidates.iter().take(m).map(|c| c.id).collect()
    }

    fn prune_neighbors(&mut self, node_id: usize, layer: usize, m_max: usize) {
        let node_ep = self.nodes[node_id].vector.clone();
        let node_vec = &node_ep;

        // Keep m_max nearest neighbors
        let mut scored: Vec<(f32, usize)> = self.nodes[node_id].layers[layer]
            .iter()
            .map(|&nb| (self.distance(node_vec, &self.nodes[nb].vector), nb))
            .collect();
        scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
        scored.truncate(m_max);
        self.nodes[node_id].layers[layer] = scored.into_iter().map(|(_, id)| id).collect();
    }

    fn distance(&self, a: &[f32], b: &[f32]) -> f32 {
        match self.metric_tag {
            0 => <super::distance::CosineDistance as DistanceMetric>::distance(a, b),
            1 => <super::distance::EuclideanDistance as DistanceMetric>::distance(a, b),
            2 => <super::distance::DotProduct as DistanceMetric>::distance(a, b),
            _ => <super::distance::EuclideanDistance as DistanceMetric>::distance(a, b),
        }
    }
}
mod tests {
    use super::*;

    #[test]
    fn hnsw_insert_and_search() {
        let mut index = HnswIndex::new(3, 6, 20, HnswMetric::Euclidean);

        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        let v3 = vec![0.0, 0.0, 1.0];
        let v4 = vec![1.0, 1.0, 0.0];

        index.insert(&v1);
        index.insert(&v2);
        index.insert(&v3);
        index.insert(&v4);

        let results = index.search(&vec![0.9, 0.0, 0.1], 2);
        assert!(!results.is_empty());
        assert!(results.len() <= 2);
    }

    #[test]
    fn hnsw_remove_excludes_from_search() {
        let mut index = HnswIndex::new(3, 6, 20, HnswMetric::Euclidean);
        let id = index.insert(&[1.0, 0.0, 0.0]);
        index.insert(&[0.0, 1.0, 0.0]);

        index.remove(id);
        let results = index.search(&[1.0, 0.0, 0.0], 2);
        assert!(!results.contains(&id));
    }

    #[test]
    fn hnsw_empty_search() {
        let index = HnswIndex::new(3, 6, 20, HnswMetric::Cosine);
        assert!(index.search(&[1.0, 0.0, 0.0], 5).is_empty());
    }

    #[test]
    fn hnsw_exact_nearest() {
        let mut index = HnswIndex::new(2, 16, 50, HnswMetric::Euclidean);
        index.set_ef_search(100);

        let ids: Vec<usize> = (0..50)
            .map(|i| {
                let angle = (i as f32) * 0.125;
                index.insert(&[angle.cos(), angle.sin()])
            })
            .collect();

        let query = vec![1.0, 0.0];
        let results = index.search(&query, 3);
        assert_eq!(results.len(), 3);
    }
}

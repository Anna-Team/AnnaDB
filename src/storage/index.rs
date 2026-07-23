use std::collections::{BTreeMap, HashMap, HashSet};

use tracing::{debug, info};

use crate::data_types::primitives::link::Link;
use crate::data_types::primitives::Primitive;
use crate::storage::vector::hnsw::HnswMetric;
use crate::storage::vector::VectorIndex;
use crate::tyson::map::TySONMap;
use crate::tyson::primitive::TySONPrimitive;
use crate::Item;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
    Neq,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum IndexKey {
    Str(String),
    Num(u64),
    Bool(bool),
    Null,
}

impl IndexKey {
    pub fn from_primitive(p: &Primitive) -> Option<Self> {
        match p {
            Primitive::StringPrimitive(s) => Some(IndexKey::Str(s.get_string_value())),
            Primitive::NumberPrimitive(n) => Some(IndexKey::Num(f64_to_sortable(n.get_value()))),
            Primitive::BoolPrimitive(b) => Some(IndexKey::Bool(b.val())),
            Primitive::NullPrimitive(_) => Some(IndexKey::Null),
            _ => None,
        }
    }
}

impl Ord for IndexKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (IndexKey::Null, IndexKey::Null) => std::cmp::Ordering::Equal,
            (IndexKey::Null, _) => std::cmp::Ordering::Less,
            (_, IndexKey::Null) => std::cmp::Ordering::Greater,
            (IndexKey::Bool(a), IndexKey::Bool(b)) => a.cmp(b),
            (IndexKey::Bool(_), _) => std::cmp::Ordering::Less,
            (_, IndexKey::Bool(_)) => std::cmp::Ordering::Greater,
            (IndexKey::Num(a), IndexKey::Num(b)) => a.cmp(b),
            (IndexKey::Num(_), _) => std::cmp::Ordering::Less,
            (_, IndexKey::Num(_)) => std::cmp::Ordering::Greater,
            (IndexKey::Str(a), IndexKey::Str(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for IndexKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn f64_to_sortable(f: f64) -> u64 {
    let bits = f.to_bits();
    if bits & (1u64 << 63) != 0 { !bits } else { bits | (1u64 << 63) }
}

#[derive(Debug)]
pub struct BTreeIndex {
    pub field_path: String,
    tree: BTreeMap<IndexKey, HashSet<Link>>,
}

impl BTreeIndex {
    pub fn new(field_path: String) -> Self {
        Self { field_path, tree: BTreeMap::new() }
    }

    pub fn insert(&mut self, key: IndexKey, link: Link) {
        self.tree.entry(key).or_default().insert(link);
    }

    pub fn remove(&mut self, key: &IndexKey, link: &Link) {
        if let Some(set) = self.tree.get_mut(key) {
            set.remove(link);
            if set.is_empty() { self.tree.remove(key); }
        }
    }

    pub fn lookup_by_op(&self, op: CompareOp, key: &IndexKey) -> Vec<Link> {
        match op {
            CompareOp::Eq => self.lookup_eq(key),
            CompareOp::Gt => self.lookup_gt(key),
            CompareOp::Gte => self.lookup_gte(key),
            CompareOp::Lt => self.lookup_lt(key),
            CompareOp::Lte => self.lookup_lte(key),
            CompareOp::Neq => {
                let all = self.all_links();
                let excluded: HashSet<Link> = self.lookup_eq(key).into_iter().collect();
                all.into_iter().filter(|l| !excluded.contains(l)).collect()
            }
        }
    }

    pub fn lookup_eq(&self, key: &IndexKey) -> Vec<Link> {
        match self.tree.get(key) {
            Some(set) => set.iter().cloned().collect(),
            None => vec![],
        }
    }

    pub fn lookup_gt(&self, key: &IndexKey) -> Vec<Link> {
        let mut result = Vec::new();
        for (_, set) in self.tree.range((std::ops::Bound::Excluded(key.clone()), std::ops::Bound::Unbounded)) {
            result.extend(set.iter().cloned());
        }
        result
    }

    pub fn lookup_gte(&self, key: &IndexKey) -> Vec<Link> {
        let mut result = Vec::new();
        for (_, set) in self.tree.range(key.clone()..) {
            result.extend(set.iter().cloned());
        }
        result
    }

    pub fn lookup_lt(&self, key: &IndexKey) -> Vec<Link> {
        let mut result = Vec::new();
        for (_, set) in self.tree.range(..key.clone()) {
            result.extend(set.iter().cloned());
        }
        result
    }

    pub fn lookup_lte(&self, key: &IndexKey) -> Vec<Link> {
        let mut result = Vec::new();
        for (_, set) in self.tree.range(..=key.clone()) {
            result.extend(set.iter().cloned());
        }
        result
    }

    pub fn all_links(&self) -> Vec<Link> {
        self.tree.values().flat_map(|s| s.iter().cloned()).collect()
    }

    pub fn len(&self) -> usize { self.tree.len() }
}

#[derive(Debug)]
pub struct IndexManager {
    indexes: HashMap<String, HashMap<String, BTreeIndex>>,
    pub vector_indexes: HashMap<String, HashMap<String, VectorIndex>>,
}

impl IndexManager {
    pub fn new() -> Self {
        Self { indexes: HashMap::new(), vector_indexes: HashMap::new() }
    }

    pub fn create_index(&mut self, collection_name: &str, field_path: &str) {
        let coll = self.indexes.entry(collection_name.to_string()).or_default();
        if !coll.contains_key(field_path) {
            coll.insert(field_path.to_string(), BTreeIndex::new(field_path.to_string()));
            info!(collection = collection_name, field = field_path, "index created");
        }
    }

    pub fn create_vector_index(
        &mut self, collection_name: &str, field_path: &str,
        dims: u16, m: usize, ef_construction: usize, metric: HnswMetric,
    ) {
        let coll = self.vector_indexes.entry(collection_name.to_string()).or_default();
        if !coll.contains_key(field_path) {
            coll.insert(field_path.to_string(), VectorIndex::new(field_path.to_string(), dims, m, ef_construction, metric));
            info!(collection = collection_name, field = field_path, "vector index created");
        }
    }

    pub fn drop_index(&mut self, collection_name: &str, field_path: &str) -> bool {
        self.indexes
            .get_mut(collection_name)
            .and_then(|m| m.remove(field_path))
            .is_some()
    }

    pub fn drop_collection_indexes(&mut self, collection_name: &str) {
        self.indexes.remove(collection_name);
        self.vector_indexes.remove(collection_name);
        debug!(collection = collection_name, "all indexes dropped for collection");
    }

    pub fn get_index(&self, collection_name: &str, field_path: &str) -> Option<&BTreeIndex> {
        self.indexes.get(collection_name).and_then(|m| m.get(field_path))
    }

    pub fn get_index_mut(&mut self, collection_name: &str, field_path: &str) -> Option<&mut BTreeIndex> {
        self.indexes.get_mut(collection_name).and_then(|m| m.get_mut(field_path))
    }

    pub fn get_vector_index(&self, collection_name: &str, field_path: &str) -> Option<&VectorIndex> {
        self.vector_indexes.get(collection_name).and_then(|m| m.get(field_path))
    }

    pub fn get_vector_index_mut(&mut self, collection_name: &str, field_path: &str) -> Option<&mut VectorIndex> {
        self.vector_indexes.get_mut(collection_name).and_then(|m| m.get_mut(field_path))
    }

    pub fn get_indexed_fields(&self, collection_name: &str) -> Vec<String> {
        self.indexes.get(collection_name).map(|m| m.keys().cloned().collect()).unwrap_or_default()
    }

    pub fn on_insert(&mut self, collection_name: &str, link: &Link, item: &Item, old_item: Option<&Item>) {
        // B-tree indexes
        if let Some(btree_fields) = self.indexes.get(collection_name) {
            let fields: Vec<String> = btree_fields.keys().cloned().collect();
            for field_path in &fields {
                if let Some(old) = old_item {
                    if let Some(old_key) = extract_field_value(old, field_path) {
                        if let Some(idx) = self.get_index_mut(collection_name, field_path) {
                            idx.remove(&old_key, link);
                        }
                    }
                }
                if let Some(new_key) = extract_field_value(item, field_path) {
                    if let Some(idx) = self.get_index_mut(collection_name, field_path) {
                        idx.insert(new_key, link.clone());
                    }
                }
            }
        }
        // Vector indexes
        if let Some(vec_fields) = self.vector_indexes.get(collection_name) {
            let fields: Vec<String> = vec_fields.keys().cloned().collect();
            for field_path in &fields {
                if let Some(old) = old_item {
                    if extract_embedding(old, field_path).is_some() {
                        if let Some(idx) = self.get_vector_index_mut(collection_name, field_path) {
                            idx.remove(link);
                        }
                    }
                }
                if let Some(emb) = extract_embedding(item, field_path) {
                    if let Some(idx) = self.get_vector_index_mut(collection_name, field_path) {
                        idx.insert(&emb, link.clone());
                    }
                }
            }
        }
    }

    pub fn on_delete(&mut self, collection_name: &str, link: &Link, old_item: &Item) {
        if let Some(btree_fields) = self.indexes.get(collection_name) {
            let fields: Vec<String> = btree_fields.keys().cloned().collect();
            for field_path in &fields {
                if let Some(old_key) = extract_field_value(old_item, field_path) {
                    if let Some(idx) = self.get_index_mut(collection_name, field_path) {
                        idx.remove(&old_key, link);
                    }
                }
            }
        }
        if let Some(vec_fields) = self.vector_indexes.get(collection_name) {
            let fields: Vec<String> = vec_fields.keys().cloned().collect();
            for field_path in &fields {
                if extract_embedding(old_item, field_path).is_some() {
                    if let Some(idx) = self.get_vector_index_mut(collection_name, field_path) {
                        idx.remove(link);
                    }
                }
            }
        }
    }

    pub fn rebuild_collection(&mut self, collection_name: &str, data: &HashMap<Link, Item>) {
        if let Some(btree_fields) = self.indexes.get(collection_name) {
            let fields: Vec<String> = btree_fields.keys().cloned().collect();
            for field_path in &fields {
                if let Some(idx) = self.get_index_mut(collection_name, field_path) {
                    *idx = BTreeIndex::new(field_path.clone());
                }
                for (link, item) in data {
                    if let Some(key) = extract_field_value(item, field_path) {
                        if let Some(idx) = self.get_index_mut(collection_name, field_path) {
                            idx.insert(key, link.clone());
                        }
                    }
                }
            }
        }

        if let Some(vec_fields) = self.vector_indexes.get(collection_name) {
            let fields: Vec<String> = vec_fields.keys().cloned().collect();
            for field_path in &fields {
                let (dims, m, ef, metric) = {
                    let idx = self
                        .vector_indexes
                        .get(collection_name)
                        .and_then(|m| m.get(field_path));
                    match idx {
                        Some(i) => (i.dims, i.hnsw.M, i.hnsw.ef_construction, i.metric),
                        None => continue,
                    }
                };
                let mut new_idx = VectorIndex::new(field_path.clone(), dims, m, ef, metric);
                for (link, item) in data {
                    if let Some(emb) = extract_embedding(item, field_path) {
                        new_idx.insert(&emb, link.clone());
                    }
                }
                self.vector_indexes
                    .get_mut(collection_name)
                    .and_then(|m| m.insert(field_path.clone(), new_idx));
            }
        }

        info!(collection = collection_name, documents = data.len(), "indexes rebuilt");
    }
}

fn extract_embedding(item: &Item, field_path: &str) -> Option<Vec<f32>> {
    if field_path == "_root" {
        match item {
            Item::Primitive(Primitive::EmbeddingPrimitive(e)) => Some(e.values().to_vec()),
            _ => None,
        }
    } else {
        let mut parts = field_path.split('.');
        let first = parts.next()?;
        traverse_embedding(item, first, parts)
    }
}

fn traverse_embedding<'a, I: Iterator<Item = &'a str>>(
    current: &Item,
    segment: &str,
    mut remaining: I,
) -> Option<Vec<f32>> {
    match current {
        Item::Map(map_item) => {
            let items = map_item.get_items();
            for (k, v) in &items {
                if let Primitive::StringPrimitive(s) = k {
                    if s.get_string_value() == segment {
                        return match remaining.next() {
                            Some(next) => traverse_embedding(v, next, remaining),
                            None => match v {
                                Item::Primitive(Primitive::EmbeddingPrimitive(e)) => Some(e.values().to_vec()),
                                _ => None,
                            },
                        };
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn extract_field_value(item: &Item, field_path: &str) -> Option<IndexKey> {
    if field_path == "_root" {
        match item {
            Item::Primitive(p) => IndexKey::from_primitive(p),
            _ => None,
        }
    } else {
        let mut parts = field_path.split('.');
        let first = parts.next()?;
        traverse_path(item, first, parts)
    }
}

fn traverse_path<'a, I: Iterator<Item = &'a str>>(current: &Item, segment: &str, mut remaining: I) -> Option<IndexKey> {
    match current {
        Item::Map(map_item) => {
            let items = map_item.get_items();
            for (k, v) in &items {
                if let Primitive::StringPrimitive(s) = k {
                    if s.get_string_value() == segment {
                        return match remaining.next() {
                            Some(next) => traverse_path(v, next, remaining),
                            None => match v {
                                Item::Primitive(p) => IndexKey::from_primitive(p),
                                _ => None,
                            },
                        };
                    }
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_key_ordering_nulls_first() {
        assert!(IndexKey::Null < IndexKey::Bool(false));
        assert!(IndexKey::Null < IndexKey::Num(0));
        assert!(IndexKey::Null < IndexKey::Str("".to_string()));
    }

    #[test]
    fn index_key_ordering_bools_before_numbers() {
        assert!(IndexKey::Bool(false) < IndexKey::Num(0));
    }

    #[test]
    fn index_key_ordering_numbers_before_strings() {
        assert!(IndexKey::Num(0) < IndexKey::Str("a".to_string()));
    }

    #[test]
    fn index_key_from_primitive_string() {
        let p = Primitive::new("s".to_string(), "hello".to_string()).unwrap();
        assert_eq!(IndexKey::from_primitive(&p), Some(IndexKey::Str("hello".to_string())));
    }

    #[test]
    fn index_key_from_embedding_is_none() {
        let p = Primitive::new("e".to_string(), "3|0.1,0.2,0.3".to_string()).unwrap();
        assert_eq!(IndexKey::from_primitive(&p), None);
    }

    #[test]
    fn btree_index_insert_and_lookup() {
        let mut idx = BTreeIndex::new("test".to_string());
        let key = IndexKey::Str("hello".to_string());
        let link = Link::create("test".to_string());
        idx.insert(key.clone(), link.clone());
        assert_eq!(idx.lookup_eq(&key), vec![link]);
    }

    #[test]
    fn btree_index_remove_clears() {
        let mut idx = BTreeIndex::new("test".to_string());
        let key = IndexKey::Str("hello".to_string());
        let link = Link::create("test".to_string());
        idx.insert(key.clone(), link.clone());
        idx.remove(&key, &link);
        assert!(idx.lookup_eq(&key).is_empty());
    }
}

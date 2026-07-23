use serde::{Deserialize, Serialize};

use crate::{Item, Link, Primitive};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InsertBuffer {
    pub(crate) items: HashMap<Link, Item>,
    pub(crate) changed: bool,
    pub dropped_collections: Vec<String>,
}

impl InsertBuffer {
    pub(crate) fn new() -> Self {
        Self {
            items: HashMap::new(),
            changed: false,
            dropped_collections: vec![],
        }
    }

    pub(crate) fn insert(&mut self, link: Link, item: Item) {
        self.items.insert(link, item);
        self.changed = true;
    }

    pub(crate) fn add_collection_to_drop(&mut self, collection_name: String) {
        self.items
            .retain(|k, _| *k.collection_name != collection_name);
        self.dropped_collections.push(collection_name);
    }
}

#[derive(Clone, Debug)]
pub struct FilterBuffer {
    pub(crate) ids: Vec<Link>,
}

impl FilterBuffer {
    pub(crate) fn new() -> Self {
        Self { ids: vec![] }
    }

    pub(crate) fn update(&mut self, ids: Vec<Link>) {
        self.ids = ids;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_buffer_new_is_empty() {
        let buf = InsertBuffer::new();
        assert!(!buf.changed);
        assert!(buf.items.is_empty());
        assert!(buf.dropped_collections.is_empty());
    }

    #[test]
    fn insert_buffer_insert_changes() {
        let mut buf = InsertBuffer::new();
        let link = Link::create("test".to_string());
        let item = Item::Primitive(Primitive::new("s".to_string(), "hello".to_string()).unwrap());
        buf.insert(link.clone(), item);
        assert!(buf.changed);
        assert!(buf.items.contains_key(&link));
    }

    #[test]
    fn filter_buffer_new_is_empty() {
        let buf = FilterBuffer::new();
        assert!(buf.ids.is_empty());
    }

    #[test]
    fn filter_buffer_update() {
        let mut buf = FilterBuffer::new();
        let link = Link::create("test".to_string());
        buf.update(vec![link.clone()]);
        assert_eq!(buf.ids, vec![link]);
    }
}

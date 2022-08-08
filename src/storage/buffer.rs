use crate::{Item, Link};
use std::collections::HashMap;

#[derive(Clone, Debug)]
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

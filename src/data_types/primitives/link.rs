use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use uuid::Uuid;

use crate::data_types::primitives::path::Path;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::{DBError, Item, TySONMap};

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub struct Link {
    pub(crate) collection_name: String,
    id: Uuid,
    links_to: Vec<Link>,
}

impl Link {
    pub(crate) fn create(collection_name: String) -> Self {
        Self {
            collection_name,
            id: Uuid::new_v4(),
            links_to: vec![],
        }
    }

    pub fn unlink(&mut self, link: &Link) {
        self.links_to.retain(|x| x != link)
    }
}

impl BaseTySONItemInterface for Link {
    fn get_prefix(&self) -> String {
        self.collection_name.to_string()
    }
}

impl TySONPrimitive for Link {
    fn new(prefix: String, value: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self {
            collection_name: prefix,
            id: Uuid::from_str(value.as_str())?,
            links_to: vec![],
        })
    }

    fn get_string_value(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Debug)]
pub struct LinkData {
    pub(crate) data: Item,
    pub(crate) back_refs: HashMap<Link, Path>,
}

impl LinkData {
    pub(crate) fn new(data: Item) -> Self {
        Self {
            data,
            back_refs: HashMap::new(),
        }
    }

    pub(crate) fn add_back_ref(&mut self, link: Link, path: Path) {
        self.back_refs.insert(link, path);
    }

    pub(crate) fn remove_back_ref(&mut self, link: &Link) {
        self.back_refs.remove(link);
    }

    pub(crate) fn generate_back_refs_for_data(&mut self) {
        match &self.data {
            Item::Map(map) => for (key, value) in map.get_items() {},
            _ => {}
        }
    }
}

use std::fmt::Debug;
use std::str::FromStr;

use uuid::Uuid;

use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

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

use std::fmt::Debug;
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::DBError;

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Link {
    pub collection_name: String,
    pub id: Uuid,
    pub links_to: Vec<Link>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_unlink_removes_reference() {
        let mut link = Link::create("test".to_string());
        let other = Link::create("other".to_string());
        link.links_to.push(other.clone());
        assert_eq!(link.links_to.len(), 1);
        link.unlink(&other);
        assert!(link.links_to.is_empty());
    }

    #[test]
    fn link_get_string_value() {
        let link = Link::new("test".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap();
        assert_eq!(link.get_string_value(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn link_get_prefix() {
        use crate::tyson::item::BaseTySONItemInterface;
        let link = Link::create("test".to_string());
        assert_eq!(link.get_prefix(), "test");
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

use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::constants::{ASC_OPERATOR, DESC_OPERATOR, SORT_QUERY};

use crate::data_types::modifier::ModifierItem;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::modifier::TySONModifier;
use crate::{DBError, Item, Primitive, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AscOperator {
    expr: Box<Item>, // TODO looks ugly
}

impl BaseTySONItemInterface for AscOperator {
    fn get_prefix(&self) -> String {
        ASC_OPERATOR.to_string()
    }
}

impl TySONModifier for AscOperator {
    fn new(_: String, value: Item) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        match value {
            Item::Primitive(Primitive::PathToValue(_)) => Ok(Self {
                expr: Box::new(value),
            }),
            Item::Primitive(Primitive::RootPrimitive(_)) => Ok(Self {
                expr: Box::new(value),
            }),
            _ => Err(DBError::TypeMismatch(format!("cannot sort by {}", value.to_tyson()))),
        }
    }

    fn get_serialized_value(&self) -> String {
        self.expr.to_tyson()
    }
}

impl AscOperator {
    pub fn get_value(&self) -> &Item {
        self.expr.as_ref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DescOperator {
    expr: Box<Item>, // TODO looks ugly
}

impl BaseTySONItemInterface for DescOperator {
    fn get_prefix(&self) -> String {
        DESC_OPERATOR.to_string()
    }
}

impl TySONModifier for DescOperator {
    fn new(_: String, value: Item) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        Ok(Self {
            expr: Box::new(value),
        })
    }

    fn get_serialized_value(&self) -> String {
        self.expr.to_tyson()
    }
}

impl DescOperator {
    pub fn get_value(&self) -> &Item {
        self.expr.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SortQuery {
    pub items: Vec<Item>,
}

impl BaseTySONItemInterface for SortQuery {
    fn get_prefix(&self) -> String {
        SORT_QUERY.to_string()
    }
}

impl TySONVector for SortQuery {
    fn new(_: String) -> Result<Self, DBError> {
        Ok(Self { items: vec![] })
    }

    fn push(&mut self, item: Item) -> Result<bool, DBError> {
        match item {
            Item::Modifier(ModifierItem::AscOperator(_)) => {
                self.items.push(item);
                Ok(true)
            }
            Item::Modifier(ModifierItem::DescOperator(_)) => {
                self.items.push(item);
                Ok(true)
            }
            _ => Err(DBError::UnsupportedOperation(format!("sort operator: {}", item.to_tyson()))),
        }
    }

    fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    fn to_item(self) -> Item {
        Item::Vector(VectorItem::SortQuery(self))
    }
}

impl SortQuery {
    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![
            QueryOperation::FindOperation,
            QueryOperation::UpdateOperation,
            QueryOperation::DeleteOperation,
            QueryOperation::LimitOperation,
            QueryOperation::ProjectOperation,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_types::primitives::number::NumberPrimitive;
    use crate::data_types::primitives::path::PathToValue;
    use crate::data_types::primitives::root::RootPrimitive;
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn asc_operator_creation_with_path() {
        let p = PathToValue::new("".to_string(), "name".to_string()).unwrap();
        let item = Item::Primitive(Primitive::PathToValue(p));
        let asc = AscOperator::new("".to_string(), item).unwrap();
        assert_eq!(asc.get_prefix(), "asc");
    }

    #[test]
    fn asc_operator_creation_with_root() {
        let root = RootPrimitive::new("".to_string(), "".to_string()).unwrap();
        let item = Item::Primitive(Primitive::RootPrimitive(root));
        let asc = AscOperator::new("".to_string(), item).unwrap();
        assert_eq!(asc.get_prefix(), "asc");
    }

    #[test]
    fn asc_operator_rejects_non_path() {
        let num = NumberPrimitive::new("".to_string(), "42".to_string()).unwrap();
        let item = Item::Primitive(Primitive::NumberPrimitive(num));
        assert!(AscOperator::new("".to_string(), item).is_err());
    }

    #[test]
    fn desc_operator_creation() {
        let p = PathToValue::new("".to_string(), "name".to_string()).unwrap();
        let item = Item::Primitive(Primitive::PathToValue(p));
        let desc = DescOperator::new("".to_string(), item).unwrap();
        assert_eq!(desc.get_prefix(), "desc");
    }

    #[test]
    fn sort_query_new() {
        let sq = SortQuery::new("".to_string()).unwrap();
        assert!(sq.items.is_empty());
        assert_eq!(sq.get_prefix(), "sort");
    }

    #[test]
    fn sort_query_push_asc() {
        let mut sq = SortQuery::new("".to_string()).unwrap();
        let p = PathToValue::new("".to_string(), "name".to_string()).unwrap();
        let asc = AscOperator::new("".to_string(), Item::Primitive(Primitive::PathToValue(p))).unwrap();
        sq.push(Item::Modifier(ModifierItem::AscOperator(asc))).unwrap();
        assert_eq!(sq.items.len(), 1);
    }

    #[test]
    fn sort_query_push_invalid() {
        let mut sq = SortQuery::new("".to_string()).unwrap();
        let invalid = Item::Primitive(Primitive::new("s".to_string(), "hello".to_string()).unwrap());
        assert!(sq.push(invalid).is_err());
    }
}

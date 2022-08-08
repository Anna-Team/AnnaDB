use std::fmt::Debug;

use crate::constants::{ASC_OPERATOR, DESC_OPERATOR, SORT_QUERY};

use crate::data_types::modifier::ModifierItem;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::modifier::TySONModifier;
use crate::{DBError, Item, Primitive, TySONVector, VectorItem};

#[derive(Debug, Clone, Eq, PartialEq)]
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
            _ => Err(DBError::new(
                format!("Can not sort by {}", value.serialize()).as_str(),
            )),
        }
    }

    fn get_serialized_value(&self) -> String {
        self.expr.serialize()
    }
}

impl AscOperator {
    pub fn get_value(&self) -> &Item {
        self.expr.as_ref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
        self.expr.serialize()
    }
}

impl DescOperator {
    pub fn get_value(&self) -> &Item {
        self.expr.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
            _ => Err(DBError::new(
                format!("Sort does not support '{}' as operator", item.serialize()).as_str(),
            )),
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
        ]
    }
}

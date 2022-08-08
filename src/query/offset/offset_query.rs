use crate::constants::OFFSET_QUERY;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::modifier::TySONModifier;
use crate::{DBError, Item, Primitive};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OffsetQuery {
    expr: Box<Item>, // TODO looks ugly
}

impl BaseTySONItemInterface for OffsetQuery {
    fn get_prefix(&self) -> String {
        OFFSET_QUERY.to_string()
    }
}

impl TySONModifier for OffsetQuery {
    fn new(_: String, value: Item) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        match &value {
            Item::Primitive(Primitive::NumberPrimitive(_pr)) => Ok(Self {
                expr: Box::new(value),
            }),
            _ => Err(DBError::new("Limit supports only numbers as a parameter")),
        }
    }

    fn get_serialized_value(&self) -> String {
        self.expr.serialize()
    }
}

impl OffsetQuery {
    pub fn get_value(&self) -> &Item {
        self.expr.as_ref()
    }

    pub fn next_available(&self) -> Vec<QueryOperation> {
        vec![
            QueryOperation::FindOperation,
            QueryOperation::UpdateOperation,
            QueryOperation::DeleteOperation,
            QueryOperation::LimitOperation,
            QueryOperation::OffsetOperation,
        ]
    }
}

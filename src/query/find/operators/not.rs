use std::fmt::Debug;

use crate::constants::NOT_OPERATOR;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::modifier::TySONModifier;
use crate::{DBError, Item};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotOperator {
    expr: Box<Item>, // TODO looks ugly
}

impl BaseTySONItemInterface for NotOperator {
    fn get_prefix(&self) -> String {
        NOT_OPERATOR.to_string()
    }
}

impl TySONModifier for NotOperator {
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

impl NotOperator {
    pub fn get_value(&self) -> &Item {
        self.expr.as_ref()
    }
}

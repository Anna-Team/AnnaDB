use serde::{Serialize, Deserialize};

use crate::constants::LIMIT_QUERY;
use crate::query::operations::QueryOperation;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::modifier::TySONModifier;
use crate::{DBError, Item, Primitive};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LimitQuery {
    expr: Box<Item>, // TODO looks ugly
}

impl BaseTySONItemInterface for LimitQuery {
    fn get_prefix(&self) -> String {
        LIMIT_QUERY.to_string()
    }
}

impl TySONModifier for LimitQuery {
    fn new(_: String, value: Item) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        match &value {
            Item::Primitive(Primitive::NumberPrimitive(_pr)) => Ok(Self {
                expr: Box::new(value),
            }),
            _ => Err(DBError::TypeMismatch("limit supports only numbers".to_string())),
        }
    }

    fn get_serialized_value(&self) -> String {
        self.expr.to_tyson()
    }
}

impl LimitQuery {
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
            QueryOperation::ProjectOperation,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_types::primitives::number::NumberPrimitive;
    use crate::tyson::modifier::TySONModifier;
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn limit_query_new_with_number() {
        let num = NumberPrimitive::new("".to_string(), "5".to_string()).unwrap();
        let item = Item::Primitive(Primitive::NumberPrimitive(num));
        let q = LimitQuery::new("".to_string(), item).unwrap();
        assert_eq!(q.get_prefix(), "limit");
        assert!(q.next_available().len() > 0);
    }

    #[test]
    fn limit_query_rejects_non_number() {
        let item = Item::Primitive(Primitive::new("s".to_string(), "bad".to_string()).unwrap());
        assert!(LimitQuery::new("".to_string(), item).is_err());
    }

    #[test]
    fn limit_query_get_value() {
        let num = NumberPrimitive::new("".to_string(), "10".to_string()).unwrap();
        let item = Item::Primitive(Primitive::NumberPrimitive(num));
        let q = LimitQuery::new("".to_string(), item).unwrap();
        assert!(matches!(q.get_value(), Item::Primitive(Primitive::NumberPrimitive(_))));
    }

    #[test]
    fn limit_query_serialized_value() {
        let num = NumberPrimitive::new("".to_string(), "3".to_string()).unwrap();
        let item = Item::Primitive(Primitive::NumberPrimitive(num));
        let q = LimitQuery::new("".to_string(), item).unwrap();
        assert!(q.get_serialized_value().contains("3"));
    }
}

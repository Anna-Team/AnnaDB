use serde::{Serialize, Deserialize};

use crate::constants::{ASC_OPERATOR, DESC_OPERATOR, LIMIT_QUERY, NOT_OPERATOR, OFFSET_QUERY};
use crate::query::find::operators::not::NotOperator;
use crate::query::limit::query::LimitQuery;
use crate::query::offset::query::OffsetQuery;
use crate::query::sort::query::{AscOperator, DescOperator};
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::modifier::TySONModifier;
use crate::{DBError, Item};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ModifierItem {
    NotOperator(NotOperator),
    AscOperator(AscOperator),
    DescOperator(DescOperator),
    LimitQuery(LimitQuery),
    OffsetQuery(OffsetQuery),
}

impl BaseTySONItemInterface for ModifierItem {
    fn get_prefix(&self) -> String {
        match self {
            ModifierItem::NotOperator(o) => o.get_prefix(),
            ModifierItem::AscOperator(o) => o.get_prefix(),
            ModifierItem::DescOperator(o) => o.get_prefix(),
            ModifierItem::LimitQuery(o) => o.get_prefix(),
            ModifierItem::OffsetQuery(o) => o.get_prefix(),
        }
    }
}

impl TySONModifier for ModifierItem {
    fn new(prefix: String, value: Item) -> Result<Self, DBError> {
        match prefix.as_str() {
            NOT_OPERATOR => Ok(Self::NotOperator(NotOperator::new(prefix, value)?)),
            ASC_OPERATOR => Ok(Self::AscOperator(AscOperator::new(prefix, value)?)),
            DESC_OPERATOR => Ok(Self::DescOperator(DescOperator::new(prefix, value)?)),
            LIMIT_QUERY => Ok(Self::LimitQuery(LimitQuery::new(prefix, value)?)),
            OFFSET_QUERY => Ok(Self::OffsetQuery(OffsetQuery::new(prefix, value)?)),
            _ => Err(DBError::UnexpectedType(format!("modifier: {}", prefix))),
        }
    }

    fn get_serialized_value(&self) -> String {
        match self {
            ModifierItem::NotOperator(o) => o.get_serialized_value(),
            ModifierItem::AscOperator(o) => o.get_serialized_value(),
            ModifierItem::DescOperator(o) => o.get_serialized_value(),
            ModifierItem::LimitQuery(o) => o.get_serialized_value(),
            ModifierItem::OffsetQuery(o) => o.get_serialized_value(),
        }
    }
}

impl ModifierItem {
    pub fn get_value(&self) -> &Item {
        match self {
            ModifierItem::NotOperator(o) => o.get_value(),
            ModifierItem::AscOperator(o) => o.get_value(),
            ModifierItem::DescOperator(o) => o.get_value(),
            ModifierItem::LimitQuery(o) => o.get_value(),
            ModifierItem::OffsetQuery(o) => o.get_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tyson::modifier::TySONModifier;
    use crate::tyson::primitive::TySONPrimitive;

    #[test]
    fn modifier_unknown_prefix() {
        let val = crate::Item::Primitive(crate::Primitive::new("null".to_string(), "".to_string()).unwrap());
        assert!(ModifierItem::new("unknown".to_string(), val).is_err());
    }

    #[test]
    fn modifier_limit_query() {
        use crate::query::limit::query::LimitQuery;
        use crate::data_types::primitives::number::NumberPrimitive;
        let num = NumberPrimitive::new("".to_string(), "5".to_string()).unwrap();
        let val = crate::Item::Primitive(crate::Primitive::NumberPrimitive(num));
        let m = ModifierItem::new("limit".to_string(), val).unwrap();
        assert_eq!(m.get_prefix(), "limit");
    }

    #[test]
    fn modifier_offset_query() {
        use crate::data_types::primitives::number::NumberPrimitive;
        let num = NumberPrimitive::new("".to_string(), "10".to_string()).unwrap();
        let val = crate::Item::Primitive(crate::Primitive::NumberPrimitive(num));
        let m = ModifierItem::new("offset".to_string(), val).unwrap();
        assert_eq!(m.get_prefix(), "offset");
    }

    #[test]
    fn modifier_get_serialized_value() {
        use crate::data_types::primitives::number::NumberPrimitive;
        let num = NumberPrimitive::new("".to_string(), "3".to_string()).unwrap();
        let val = crate::Item::Primitive(crate::Primitive::NumberPrimitive(num));
        let m = ModifierItem::new("limit".to_string(), val).unwrap();
        assert!(m.get_serialized_value().contains("3"));
    }

    #[test]
    fn modifier_get_value() {
        use crate::data_types::primitives::number::NumberPrimitive;
        let num = NumberPrimitive::new("".to_string(), "7".to_string()).unwrap();
        let val = crate::Item::Primitive(crate::Primitive::NumberPrimitive(num));
        let m = ModifierItem::new("limit".to_string(), val).unwrap();
        assert!(matches!(m.get_value(), crate::Item::Primitive(crate::Primitive::NumberPrimitive(_))));
    }
}

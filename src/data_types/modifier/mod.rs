use crate::constants::{ASC_OPERATOR, DESC_OPERATOR, LIMIT_QUERY, NOT_OPERATOR, OFFSET_QUERY};
use crate::query::find::operators::not::NotOperator;
use crate::query::limit::query::LimitQuery;
use crate::query::offset::query::OffsetQuery;
use crate::query::sort::query::{AscOperator, DescOperator};
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::modifier::TySONModifier;
use crate::{DBError, Item};

#[derive(Debug, Clone, Eq, PartialEq)]
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
            _ => Err(DBError::new("Unexpected modifier type")),
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

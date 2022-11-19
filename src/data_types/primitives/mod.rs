use crate::constants::{
    BOOL, COLLECTION_NAME, DELETED, DELETE_QUERY, KEEP, NULL, NUMBER, PATH_TO_VALUE, ROOT, STRING,
    UTS,
};
use crate::data_types::primitives::bool::BoolPrimitive;
use crate::data_types::primitives::deleted::DeletedPrimitive;
use crate::data_types::primitives::link::Link;
use crate::data_types::primitives::null::NullPrimitive;
use crate::data_types::primitives::number::NumberPrimitive;
use crate::data_types::primitives::root::RootPrimitive;
use crate::data_types::primitives::string::StringPrimitive;
use crate::data_types::primitives::unix_timestamp::UTSPrimitive;
use crate::query::delete::query::DeleteQuery;
use crate::query::project::operators::keep::KeepPrimitive;
use crate::storage::common::collection_name::CollectionName;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::primitive::TySONPrimitive;
use crate::{DBError, PathToValue};

pub mod bool;
pub mod deleted;
pub mod link;
mod null;
pub mod number;
pub mod path;
pub mod root;
pub mod string;
pub mod unix_timestamp;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum Primitive {
    Link(Link),
    StringPrimitive(StringPrimitive),
    NumberPrimitive(NumberPrimitive),
    UTSPrimitive(UTSPrimitive),
    BoolPrimitive(BoolPrimitive),
    NullPrimitive(NullPrimitive),
    DeletedPrimitive(DeletedPrimitive),

    CollectionName(CollectionName),
    PathToValue(PathToValue),
    RootPrimitive(RootPrimitive),

    DeleteQuery(DeleteQuery),

    KeepPrimitive(KeepPrimitive),
}

impl Primitive {
    pub(crate) fn new(prefix: String, value: String) -> Result<Self, DBError> {
        match prefix.as_str() {
            STRING => Ok(Self::StringPrimitive(StringPrimitive::new(prefix, value)?)),
            NUMBER => Ok(Self::NumberPrimitive(NumberPrimitive::new(prefix, value)?)),
            UTS => Ok(Self::UTSPrimitive(UTSPrimitive::new(prefix, value)?)),
            BOOL => Ok(Self::BoolPrimitive(BoolPrimitive::new(prefix, value)?)),
            NULL => Ok(Self::NullPrimitive(NullPrimitive::new(prefix, value)?)),
            DELETED => Ok(Self::DeletedPrimitive(DeletedPrimitive::new(
                prefix, value,
            )?)),

            COLLECTION_NAME => Ok(Self::CollectionName(CollectionName::new(prefix, value)?)),

            PATH_TO_VALUE => Ok(Self::PathToValue(PathToValue::new(prefix, value)?)),
            ROOT => Ok(Self::RootPrimitive(RootPrimitive::new(prefix, value)?)),

            DELETE_QUERY => Ok(Self::DeleteQuery(DeleteQuery::new(prefix, value)?)),

            KEEP => Ok(Self::KeepPrimitive(KeepPrimitive::new(prefix, value)?)),

            _ => Ok(Self::Link(Link::new(prefix, value)?)),
        }
    }

    pub(crate) fn serialize(&self) -> String {
        match self {
            Self::Link(o) => o.serialize(),

            Self::StringPrimitive(o) => o.serialize(),
            Self::NumberPrimitive(o) => o.serialize(),
            Self::UTSPrimitive(o) => o.serialize(),
            Self::BoolPrimitive(o) => o.serialize(),
            Self::NullPrimitive(o) => o.serialize(),
            Self::DeletedPrimitive(o) => o.serialize(),

            Self::CollectionName(o) => o.serialize(),
            Self::PathToValue(o) => o.serialize(),
            Self::RootPrimitive(o) => o.serialize(),

            Self::DeleteQuery(o) => o.serialize(),

            Self::KeepPrimitive(o) => o.serialize(),
        }
    }

    pub fn get_prefix(&self) -> String {
        match self {
            Self::Link(o) => o.get_prefix(),

            Self::StringPrimitive(o) => o.get_prefix(),
            Self::NumberPrimitive(o) => o.get_prefix(),
            Self::UTSPrimitive(o) => o.get_prefix(),
            Self::BoolPrimitive(o) => o.get_prefix(),
            Self::NullPrimitive(o) => o.get_prefix(),
            Self::DeletedPrimitive(o) => o.get_prefix(),

            Self::CollectionName(o) => o.get_prefix(),
            Self::PathToValue(o) => o.get_prefix(),
            Self::RootPrimitive(o) => o.get_prefix(),

            Self::DeleteQuery(o) => o.get_prefix(),

            Self::KeepPrimitive(o) => o.get_prefix(),
        }
    }
}

impl From<Link> for Primitive {
    fn from(link: Link) -> Self {
        Primitive::Link(link)
    }
}

impl From<StringPrimitive> for Primitive {
    fn from(data: StringPrimitive) -> Self {
        Primitive::StringPrimitive(data)
    }
}

impl From<CollectionName> for Primitive {
    fn from(data: CollectionName) -> Self {
        Primitive::CollectionName(data)
    }
}

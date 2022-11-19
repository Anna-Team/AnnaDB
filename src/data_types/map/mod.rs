use crate::constants::{
    EQ_OPERATOR, GTE_OPERATOR, GT_OPERATOR, INC_OPERATOR, LTE_OPERATOR, LT_OPERATOR, NEQ_OPERATOR,
    PROJECT_QUERY, RESPONSE_OBJECTS, SET_OPERATOR, STORAGE_MAP,
};
use crate::data_types::item::Item;
use crate::data_types::map::storage::StorageMap;
use crate::data_types::primitives::Primitive;
use crate::query::find::operators::eq::EqOperator;
use crate::query::find::operators::gt::GtOperator;
use crate::query::find::operators::gte::GteOperator;
use crate::query::find::operators::lt::LtOperator;
use crate::query::find::operators::lte::LteOperator;
use crate::query::find::operators::neq::NeqOperator;
use crate::query::project::query::ProjectQuery;
use crate::query::update::operators::inc::IncOperator;
use crate::query::update::operators::set::SetOperator;
use crate::response::objects::ResponseObjects;
use crate::tyson::item::BaseTySONItemInterface;
use crate::tyson::map::TySONMap;
use crate::DBError;

pub mod storage;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MapItem {
    StorageMap(StorageMap),

    // QUERIES
    ProjectQuery(ProjectQuery),

    // FIND OPERATORS
    EqOperator(EqOperator),
    NeqOperator(NeqOperator),
    GtOperator(GtOperator),
    GteOperator(GteOperator),
    LtOperator(LtOperator),
    LteOperator(LteOperator),

    // UPDATE OPERATORS
    SetOperator(SetOperator),
    IncOperator(IncOperator),

    // RESPONSE
    ResponseObjects(ResponseObjects),
}

impl BaseTySONItemInterface for MapItem {
    fn get_prefix(&self) -> String {
        match self {
            MapItem::StorageMap(o) => o.get_prefix(),
            MapItem::ProjectQuery(o) => o.get_prefix(),
            MapItem::SetOperator(o) => o.get_prefix(),
            MapItem::EqOperator(o) => o.get_prefix(),
            MapItem::NeqOperator(o) => o.get_prefix(),
            MapItem::GtOperator(o) => o.get_prefix(),
            MapItem::GteOperator(o) => o.get_prefix(),
            MapItem::LtOperator(o) => o.get_prefix(),
            MapItem::LteOperator(o) => o.get_prefix(),
            MapItem::IncOperator(o) => o.get_prefix(),
            MapItem::ResponseObjects(o) => o.get_prefix(),
        }
    }
}

impl TySONMap for MapItem {
    fn new(prefix: String) -> Result<Self, DBError>
    where
        Self: Sized,
    {
        match prefix.as_str() {
            STORAGE_MAP => Ok(MapItem::StorageMap(StorageMap::new("".to_string())?)),
            PROJECT_QUERY => Ok(MapItem::ProjectQuery(ProjectQuery::new("".to_string())?)),
            SET_OPERATOR => Ok(MapItem::SetOperator(SetOperator::new("".to_string())?)),
            EQ_OPERATOR => Ok(MapItem::EqOperator(EqOperator::new("".to_string())?)),
            NEQ_OPERATOR => Ok(MapItem::NeqOperator(NeqOperator::new("".to_string())?)),
            GT_OPERATOR => Ok(MapItem::GtOperator(GtOperator::new("".to_string())?)),
            GTE_OPERATOR => Ok(MapItem::GteOperator(GteOperator::new("".to_string())?)),
            LT_OPERATOR => Ok(MapItem::LtOperator(LtOperator::new("".to_string())?)),
            LTE_OPERATOR => Ok(MapItem::LteOperator(LteOperator::new("".to_string())?)),
            INC_OPERATOR => Ok(MapItem::IncOperator(IncOperator::new("".to_string())?)),
            RESPONSE_OBJECTS => Ok(MapItem::ResponseObjects(ResponseObjects::new(
                "".to_string(),
            )?)),
            _ => Err(DBError::new("Unexpected map type")),
        }
    }

    fn insert(&mut self, k: Primitive, v: Item) -> Result<bool, DBError> {
        match self {
            MapItem::StorageMap(o) => o.insert(k, v),
            MapItem::ProjectQuery(o) => o.insert(k, v),
            MapItem::SetOperator(o) => o.insert(k, v),
            MapItem::EqOperator(o) => o.insert(k, v),
            MapItem::NeqOperator(o) => o.insert(k, v),
            MapItem::GtOperator(o) => o.insert(k, v),
            MapItem::GteOperator(o) => o.insert(k, v),
            MapItem::LtOperator(o) => o.insert(k, v),
            MapItem::LteOperator(o) => o.insert(k, v),
            MapItem::IncOperator(o) => o.insert(k, v),
            MapItem::ResponseObjects(o) => o.insert(k, v),
        }
    }

    fn get_items(&self) -> Vec<(Primitive, Item)> {
        match self {
            MapItem::StorageMap(o) => o.get_items(),
            MapItem::ProjectQuery(o) => o.get_items(),
            MapItem::SetOperator(o) => o.get_items(),
            MapItem::EqOperator(o) => o.get_items(),
            MapItem::NeqOperator(o) => o.get_items(),
            MapItem::GtOperator(o) => o.get_items(),
            MapItem::GteOperator(o) => o.get_items(),
            MapItem::LtOperator(o) => o.get_items(),
            MapItem::LteOperator(o) => o.get_items(),
            MapItem::IncOperator(o) => o.get_items(),
            MapItem::ResponseObjects(o) => o.get_items(),
        }
    }

    fn to_item(self) -> Item {
        Item::Map(self)
    }
}

impl From<StorageMap> for MapItem {
    fn from(data: StorageMap) -> Self {
        MapItem::StorageMap(data)
    }
}

impl From<SetOperator> for MapItem {
    fn from(data: SetOperator) -> Self {
        MapItem::SetOperator(data)
    }
}

// impl From<Response> for MapItem {
//     fn from(data: Response) -> Self {
//         MapItem::Response(data)
//     }
// }

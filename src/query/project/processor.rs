use crate::constants::NULL;
use crate::data_types::map::storage::StorageMap;
use crate::{DBError, Item, MapItem, Primitive, StringPrimitive, TySONMap, TySONPrimitive};

#[derive(PartialEq)]
pub enum ProjectionTarget {
    Map,
    Vector,
    Primitive,
    Replace,
    NotSet,
}

impl ProjectionTarget {
    pub fn fits(&self, item: &Item) -> bool {
        match (self, item) {
            (ProjectionTarget::Map, Item::Map(_)) => true,
            (ProjectionTarget::Vector, Item::Vector(_)) => true,
            (ProjectionTarget::Primitive, Item::Primitive(_)) => true,
            _ => false,
        }
    }
}

pub struct PlainSet {
    field: StringPrimitive,
    value: Item,
}

impl PlainSet {
    pub fn new(field: StringPrimitive, value: Item) -> Self {
        Self { field, value }
    }

    pub fn get_target(&self) -> ProjectionTarget {
        return ProjectionTarget::Map;
    }

    pub fn resolve(&self, item: &StorageMap) -> Result<Item, DBError> {
        match self.value {
            Item::Primitive(Primitive::KeepPrimitive(_)) => {
                let default = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
                let res = item
                    .get_by_str(self.field.get_string_value().as_str())?
                    .unwrap_or_else(|| &default);
                Ok(res.clone())
            }
            _ => Err(DBError::new("Projection rule is not supported")),
        }
    }
}

pub struct ProjectionRules {
    items: Vec<PlainSet>,
    pub target: ProjectionTarget,
}

impl ProjectionRules {
    pub fn new() -> Self {
        Self {
            items: vec![],
            target: ProjectionTarget::NotSet,
        }
    }

    pub fn push_rule(&mut self, rule: PlainSet) -> Result<bool, DBError> {
        match self.target {
            ProjectionTarget::NotSet => {
                self.target = rule.get_target();
            }
            _ => {
                if self.target != rule.get_target() {
                    return Err(DBError::new(
                        "Incompatible projection rules. Result data structure is in conflict.",
                    ));
                }
            }
        }
        self.items.push(rule);
        return Ok(true);
    }

    pub fn is_empty(&self) -> bool {
        return self.items.len() == 0;
    }

    pub fn resolve(&self, item: &Item) -> Result<Item, DBError> {
        return match item {
            Item::Map(MapItem::StorageMap(m)) => {
                let mut new_item = StorageMap::new("".to_string())?;
                for rule in &self.items {
                    new_item.insert(
                        Primitive::StringPrimitive(rule.field.clone()),
                        rule.resolve(m)?,
                    )?;
                }
                Ok(Item::Map(MapItem::StorageMap(new_item)))
            }
            _ => Ok(Item::Primitive(Primitive::new(
                NULL.to_string(),
                "".to_string(),
            )?)),
        };
    }
}

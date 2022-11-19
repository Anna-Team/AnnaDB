use crate::query::find::compare::Res::True;
use crate::{DBError, Item, StringPrimitive};

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
}

pub enum Rule {
    PlainSet(PlainSet),
}

impl Rule {
    pub fn get_target(&self) -> ProjectionTarget {
        match self {
            Rule::PlainSet(r) => r.get_target(),
        }
    }
}

pub struct ProjectionRules {
    items: Vec<Rule>,
    pub target: ProjectionTarget,
}

impl ProjectionRules {
    pub fn new() -> Self {
        Self {
            items: vec![],
            target: ProjectionTarget::NotSet,
        }
    }

    pub fn push_rule(&mut self, rule: Rule) -> Result<bool, DBError> {
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

    pub fn resolve(&self, item: &Item) {}
}

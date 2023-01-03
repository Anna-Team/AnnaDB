use crate::data_types::primitives::path::Path;
use crate::StringPrimitive;

pub enum PathRule {
    Exists(bool),
}

pub struct ProjectionRule {
    path: StringPrimitive,
    rule: PathRule,
}

pub struct Projection {
    rules: Vec<ProjectionRule>,
}

impl Projection {
    pub(crate) fn new() -> Self {
        Self { rules: vec![] }
    }
}

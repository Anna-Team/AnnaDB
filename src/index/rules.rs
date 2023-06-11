use crate::data_types::primitives::path::Path;

pub struct PathRule {
    path: Path,
}

pub enum Rule {
    PathRule(PathRule),
}

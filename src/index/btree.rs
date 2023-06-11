use crate::index::rules::Rule;

pub struct BTreeIndex {
    pub(crate) rules: Vec<Rule>,
}

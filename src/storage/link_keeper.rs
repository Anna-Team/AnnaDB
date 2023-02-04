use std::collections::HashMap;

use crate::data_types::primitives::path::Path;
use crate::{Item, Link};

#[derive(Debug)]
pub struct LinkData {
    pub item: Item,
    pub back_track: HashMap<Link, Path>,
}

impl LinkData {
    pub fn new(item: Item) -> Self {
        Self {
            item,
            back_track: HashMap::new(),
        }
    }
}

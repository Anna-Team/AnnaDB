use crate::query::find::compare::CompareSign;
use crate::response::QueryResponse;
use crate::storage::buffer::InsertBuffer;
use crate::{DBError, Item, Link, PathToValue, Primitive, Storage};
use std::collections::{BTreeMap, HashSet};

pub enum IndexTypes {
    BTree,
}

#[derive(Clone, Debug)]
pub struct BTreeIndex {
    // path: PathToValue,
    values: BTreeMap<Primitive, Vec<Link>>,
}

impl BTreeIndex {
    pub fn new() -> Self {
        Self {
            // path,
            values: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        link: &Link,
        item: &Item,
        path: PathToValue,
        storage: &Storage,
        insert_buf: Option<&InsertBuffer>,
    ) -> Result<(), DBError> {
        let value = storage.get_item_value_by_path(path, item, insert_buf)?;
        if value.is_some() {
            match value.unwrap() {
                Item::Primitive(p) => match self.values.get_mut(&p) {
                    Some(data) => {
                        data.push(link.clone());
                    }
                    None => {
                        self.values.insert(p, vec![link.clone()]);
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }

    // pub fn find(&self,
    //             links: &HashSet<Link>,
    //             other: &Primitive,
    //             sigh: CompareSign,
    //             storage: &Storage,
    //             insert_buf: &InsertBuffer,
    // ) -> Result<HashSet<Link>, DBError> {
    //     match sigh {
    //         CompareSign::Eq => {
    //             let mut res: HashSet<Link> = HashSet::new();
    //             match self.values.get(other) {
    //                 Some(links) => {
    //                     let res: HashSet<Link> = links.iter().cloned().collect();
    //                     Ok(res)
    //                 }
    //                 None => {
    //                     Ok(HashSet::new())
    //                 }
    //             }
    //         }
    //         CompareSign::NEq => {
    //             let mut res: HashSet<Link> = HashSet::new();
    //             match self.values.get(other) {
    //                 Some(links) => {
    //                     let res: HashSet<Link> = links.iter().cloned().collect();
    //                     Ok(res)
    //                 }
    //                 None => {
    //                     Ok(HashSet::new())
    //                 }
    //             }
    //         }
    //         CompareSign::GT => {}
    //         CompareSign::GTE => {}
    //         CompareSign::LT => {}
    //         CompareSign::LTE => {}
    //     }
    //     Ok()
    // }
}

#[derive(Clone, Debug)]
pub enum Index {
    BTree(BTreeIndex),
}

impl Index {
    pub fn new(index_type: IndexTypes) -> Self {
        match index_type {
            IndexTypes::BTree => Index::BTree(BTreeIndex::new()),
        }
    }

    pub fn insert(
        &mut self,
        link: &Link,
        item: &Item,
        path: PathToValue,
        storage: &Storage,
        insert_buf: Option<&InsertBuffer>,
    ) -> Result<(), DBError> {
        match self {
            Index::BTree(i) => Ok(i.insert(link, item, path, storage, insert_buf)?),
        }
    }
}

// pub fn index(
//     storage: &Storage,
//     collection_name: String,
//     path: PathToValue,
// ) -> Result<QueryResponse, DBError> {
//     match storage.get_collection(collection_name.to_string()) {
//         Some(collection) => {
//             fs::remove_file(collection.get_path(self.wh_path.clone()))?; // TODO clean internal collection too
//             self.warehouse.remove(collection_name);
//         }
//         _ => {}
//     };
//     Ok(QueryResponse::new(
//         Item::Vector(VectorItem::ResponseIds(links)),
//         Meta::InsertMeta(meta),
//         QueryStatus::Ready,
//     ))
// }

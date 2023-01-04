use crate::constants::NULL;
use crate::data_types::modifier::ModifierItem;
use crate::data_types::primitives::path::Path;
use crate::query::sort::query::SortQuery;
use crate::response::meta::{FindMeta, Meta};
use crate::response::{QueryResponse, QueryStatus};
use crate::storage::buffer::{FilterBuffer, InsertBuffer};
use crate::{DBError, Item, Link, Primitive, Storage};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Ascending,
    Descending,
}

struct SortProcessor<'a> {
    storage: &'a Storage,
    insert_buf: &'a InsertBuffer,
    paths: Vec<Path>,
    directions: Vec<Direction>,
}

impl<'a> SortProcessor<'a> {
    fn new(storage: &'a Storage, query: &SortQuery, insert_buf: &'a InsertBuffer) -> Self {
        let (paths, directions) = SortProcessor::build_paths_and_directions(query);
        Self {
            storage,
            insert_buf,
            paths,
            directions,
        }
    }

    fn path_from_item(item: &Item) -> Option<Path> {
        match item {
            Item::Primitive(Primitive::RootPrimitive(o)) => Some(Path::from(o.clone())),
            Item::Primitive(Primitive::PathToValue(o)) => Some(Path::from(o.clone())),
            _ => None,
        }
    }

    fn build_paths_and_directions(query: &SortQuery) -> (Vec<Path>, Vec<Direction>) {
        let mut directions: Vec<Direction> = vec![];
        let mut paths: Vec<Path> = vec![];
        for item in &query.items {
            match item {
                Item::Modifier(ModifierItem::AscOperator(v)) => {
                    if let Some(path) = SortProcessor::path_from_item(v.get_value()) {
                        directions.push(Direction::Ascending);
                        paths.push(path);
                    }
                }
                Item::Modifier(ModifierItem::DescOperator(v)) => {
                    if let Some(path) = SortProcessor::path_from_item(v.get_value()) {
                        directions.push(Direction::Descending);
                        paths.push(path);
                    }
                }
                _ => {}
            }
        }
        (paths, directions)
    }

    fn build_vector(&self, link: Link) -> Vec<Option<Primitive>> {
        // TODO refactor this. It is too ugly
        let mut result_vec: Vec<Option<Primitive>> = vec![];
        for path in &self.paths {
            match path {
                Path::PathToValue(p) => {
                    let val = self.storage.find_sub_item_by_path(
                        p.clone(),
                        link.clone(),
                        self.insert_buf,
                    );
                    match val {
                        Ok(Some(found_val)) => result_vec.push(found_val.get_primitive_or_none()),
                        _ => result_vec.push(None),
                    }
                }
                Path::Root(_p) => match self.insert_buf.items.get(&link) {
                    Some(o) => match o {
                        Item::Primitive(pr) => result_vec.push(Some(pr.clone())),
                        _ => result_vec.push(None),
                    },
                    None => {
                        match self.storage.get_value_by_link(&link, None) {
                            Ok(o) => match o {
                                Item::Primitive(pr) => result_vec.push(Some(pr)),
                                _ => result_vec.push(None),
                            },
                            _ => result_vec.push(None),
                        };
                    }
                },
            }
        }
        result_vec
    }

    fn cmp_vectors(&self, a: Vec<Option<Primitive>>, b: Vec<Option<Primitive>>) -> Ordering {
        let mut index = 0 as usize;
        for left in a {
            let right = b[index].clone();
            let direction = &self.directions[index];
            index += 1;
            if left == None && right != None {
                return Ordering::Greater;
            }
            if left != None && right == None {
                return Ordering::Less;
            }
            if left > right {
                if direction == &Direction::Ascending {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            } else if left < right {
                if direction == &Direction::Ascending {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
        }
        return Ordering::Equal;
    }

    fn cmp(&self, link_a: &Link, link_b: &Link) -> Ordering {
        let vector_a = self.build_vector(link_a.clone());
        let vector_b = self.build_vector(link_b.clone());
        let res = self.cmp_vectors(vector_a, vector_b);
        res
    }
}

pub fn sort(
    query: &SortQuery,
    storage: &Storage,
    buf: &mut FilterBuffer,
    insert_buf: &InsertBuffer,
) -> Result<QueryResponse, DBError> {
    let processor = SortProcessor::new(storage, query, insert_buf);
    buf.ids.sort_by(|a, b| processor.cmp(a, b));
    let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
    let meta = Meta::FindMeta(FindMeta::new(buf.ids.len()));
    Ok(QueryResponse::new(data, meta, QueryStatus::NotFetched))
}

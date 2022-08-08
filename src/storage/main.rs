use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

use crate::constants::{
    DELETED, FETCH_DEPTH_LIMIT, INTERNAL_COLLECTION_NAME, NULL, ROOT, STORAGE_MAP, STORAGE_VECTOR,
};
use crate::data_types::modifier::ModifierItem;
use crate::data_types::primitives::path::PathToValue;
use crate::errors::DBError;
use crate::query::find::processor::find;
use crate::query::get::processor::get;
use crate::query::insert::processor::insert;
use crate::query::limit::processor::limit;
use crate::query::offset::offset_processor::offset;
use crate::query::operations::QueryOperation;
use crate::query::sort::processor::sort;
use crate::query::update::operators::set::SetOperator;
use crate::query::update::processor::update;
use crate::query::update::query::UpdateQuery;
use crate::response::meta::{DeleteMeta, Meta};
use crate::response::objects::ResponseObjects;
use crate::response::{
    ErrorTransactionResponse, OkTransactionResponse, QueryResponse, QueryStatus,
};
use crate::storage::buffer::{FilterBuffer, InsertBuffer};
use crate::storage::collection::Collection;
use crate::tyson::item::BaseTySONItemInterface;
use crate::{
    Desereilize, Item, Link, MapItem, Primitive, Transaction, TySONMap, TySONPrimitive,
    TySONVector, VectorItem,
};

#[derive(Debug)]
pub enum FoundItem {
    FoundSubItem(FoundSubItem),
    FoundRootItem(FoundRootItem),
}

impl FoundItem {
    pub fn get_value(&self) -> Option<Item> {
        match self {
            FoundItem::FoundRootItem(o) => Some(o.value.clone()),
            FoundItem::FoundSubItem(o) => o.value.clone(),
        }
    }
}

#[derive(Debug)]
pub struct FoundSubItem {
    pub container_id: Link,
    pub container_value: Item,
    pub key: String,
    pub value: Option<Item>,
}

impl FoundSubItem {
    pub fn get_primitive_or_none(&self) -> Option<Primitive> {
        match &self.value {
            Some(Item::Primitive(pr)) => Some(pr.clone()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct FoundRootItem {
    pub id: Link,
    pub value: Item,
}

#[derive(Debug)]
pub struct Storage {
    pub(crate) warehouse: HashMap<String, Collection>,
    wh_path: String,
}

impl Storage {
    pub fn new(wh_path: String) -> Result<Self, DBError> {
        fs::create_dir_all(wh_path.clone())?;
        let paths = fs::read_dir(format!("{}/", wh_path.clone()))?;
        let mut warehouse: HashMap<String, Collection> = HashMap::new();
        Collection::new(INTERNAL_COLLECTION_NAME.to_string(), wh_path.clone())?;
        for path in paths {
            let collection_name = path?.file_name().into_string()?.replace(".tyson", "");
            let collection = Collection::new(collection_name.clone(), wh_path.clone())?;
            warehouse.insert(collection_name.clone(), collection);
        }
        Ok(Self { warehouse, wh_path })
    }

    pub fn run(&mut self, data: String) -> String {
        return match self.run_transaction(data) {
            Ok(response) => response.serialize(),
            Err(e) => ErrorTransactionResponse::from(e).serialize(),
        };
    }

    fn run_transaction(&mut self, data: String) -> Result<OkTransactionResponse, DBError> {
        let transaction = Transaction::deserialize("".to_string(), data)?;

        let mut transaction_response: OkTransactionResponse = OkTransactionResponse::new();
        // let mut bufs: Vec<InsertBuffer> = vec![];
        let mut insert_buf: InsertBuffer = InsertBuffer::new();

        for query_set in transaction.steps {
            let mut filter_buf: FilterBuffer = FilterBuffer::new();
            let mut next_available: Vec<QueryOperation> = vec![
                QueryOperation::InsertOperation,
                QueryOperation::FindOperation,
                QueryOperation::GetOperation,
                QueryOperation::DeleteOperation,
            ];
            let collection_name = query_set.collection_name.clone();
            let mut iteration = 0;
            let query_set_size = query_set.query_set.items.len() as i32;
            for query in query_set.query_set.items {
                iteration += 1;
                let mut query_response = match query {
                    Item::Vector(VectorItem::InsertQuery(o)) => {
                        if next_available.contains(&QueryOperation::InsertOperation) {
                            next_available = o.next_available();
                            insert(&self, collection_name.clone(), &o.items, &mut insert_buf)?
                        } else {
                            return Err(DBError::new("Insert query is unavailable"));
                        }
                    }
                    Item::Vector(VectorItem::FindQuery(o)) => {
                        if next_available.contains(&QueryOperation::FindOperation) {
                            next_available = o.next_available();
                            let is_first: bool = if iteration == 1 { true } else { false };
                            find(
                                &self,
                                collection_name.clone(),
                                &o,
                                &mut filter_buf,
                                &insert_buf,
                                is_first,
                            )?
                        } else {
                            return Err(DBError::new("Find query is unavailable"));
                        }
                    }
                    Item::Vector(VectorItem::GetQuery(o)) => {
                        if next_available.contains(&QueryOperation::GetOperation) {
                            next_available = o.next_available();
                            get(&self, collection_name.clone(), &o, &mut filter_buf)?
                        } else {
                            return Err(DBError::new("Get query is unavailable"));
                        }
                    }
                    Item::Vector(VectorItem::UpdateQuery(o)) => {
                        if next_available.contains(&QueryOperation::UpdateOperation) {
                            next_available = o.next_available();
                            update(&self, &o, &mut insert_buf, &filter_buf)?
                        } else {
                            return Err(DBError::new("Update query is unavailable"));
                        }
                    }
                    Item::Vector(VectorItem::SortQuery(o)) => {
                        if next_available.contains(&QueryOperation::SortOperation) {
                            next_available = o.next_available();
                            sort(&o, &self, &mut filter_buf, &insert_buf)?
                        } else {
                            return Err(DBError::new("Sort query is unavailable"));
                        }
                    }
                    Item::Primitive(Primitive::DeleteQuery(o)) => {
                        // TODO rework this
                        if next_available.contains(&QueryOperation::DeleteOperation) {
                            next_available = o.next_available();
                            let delete_res: QueryResponse;
                            if iteration == 1 {
                                insert_buf.add_collection_to_drop(collection_name.clone());
                                let data = Item::Primitive(Primitive::new(
                                    NULL.to_string(),
                                    "".to_string(),
                                )?);
                                let meta = Meta::DeleteMeta(DeleteMeta::new(0 as usize));
                                delete_res = QueryResponse::new(data, meta, QueryStatus::Ready);
                            } else {
                                let set_operator = Item::Map(MapItem::SetOperator(SetOperator {
                                    values: vec![(
                                        Primitive::new(ROOT.to_string(), "".to_string())?,
                                        Item::Primitive(Primitive::new(
                                            DELETED.to_string(),
                                            "".to_string(),
                                        )?),
                                    )],
                                }));
                                let query = UpdateQuery {
                                    items: vec![set_operator],
                                };
                                delete_res = update(self, &query, &mut insert_buf, &filter_buf)?;
                            }
                            delete_res
                        } else {
                            return Err(DBError::new("Delete query is unavailable"));
                        }
                    }
                    Item::Modifier(ModifierItem::LimitQuery(o)) => {
                        if next_available.contains(&QueryOperation::LimitOperation) {
                            next_available = o.next_available();
                            limit(&o, &mut filter_buf)?
                        } else {
                            return Err(DBError::new("Limit query is unavailable"));
                        }
                    }
                    Item::Modifier(ModifierItem::OffsetQuery(o)) => {
                        if next_available.contains(&QueryOperation::LimitOperation) {
                            next_available = o.next_available();
                            offset(&o, &mut filter_buf)?
                        } else {
                            return Err(DBError::new("Offset query is unavailable"));
                        }
                    }
                    _ => {
                        return Err(DBError::new("Unexpected query type"));
                    }
                };
                if iteration == query_set_size {
                    if query_response.status == QueryStatus::NotFetched {
                        query_response.data = self.fetch_found_ids(&filter_buf, &insert_buf)?;
                        query_response.status = QueryStatus::Ready;
                    }
                    transaction_response.add_response(query_response);
                }
            }
        }
        self.sync_buf(&insert_buf)?;
        Ok(transaction_response)
    }

    fn sync_buf(&mut self, buf: &InsertBuffer) -> Result<(), DBError> {
        if buf.dropped_collections.len() > 0 {
            for collection_name in &buf.dropped_collections {
                match self.get_collection(collection_name.to_string()) {
                    Some(collection) => {
                        fs::remove_file(collection.get_path(self.wh_path.clone()))?; // TODO clean internal collection too
                        self.warehouse.remove(collection_name);
                    }
                    _ => {}
                };
            }
        }
        if buf.changed {
            for (link, item) in &buf.items {
                let collection = match self.warehouse.entry(link.get_prefix()) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => {
                        let inserting_collection =
                            Collection::new(link.get_prefix(), self.wh_path.clone())?;
                        v.insert(inserting_collection)
                    }
                };
                let mut file = collection.get_file(self.wh_path.clone())?;
                write!(file, "{}:{};", link.serialize(), item.serialize())?;
                match item {
                    Item::Primitive(Primitive::DeletedPrimitive(_)) => {
                        collection.values.remove(link);
                    }
                    _ => {
                        collection.values.insert(link.clone(), item.clone());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn insert_item(
        &self,
        collection_name: String,
        mut buf: &mut InsertBuffer,
        item: Item,
    ) -> Result<Item, DBError> {
        let link = Link::create(collection_name);
        match item {
            Item::Primitive(Primitive::Link(o)) => {
                buf.insert(link.clone(), Item::Primitive(Primitive::Link(o)));
            }
            Item::Primitive(Primitive::StringPrimitive(o)) => {
                buf.insert(link.clone(), Item::Primitive(Primitive::StringPrimitive(o)));
            }
            Item::Primitive(Primitive::NumberPrimitive(o)) => {
                buf.insert(link.clone(), Item::Primitive(Primitive::NumberPrimitive(o)));
            }
            Item::Primitive(Primitive::UTSPrimitive(o)) => {
                buf.insert(link.clone(), Item::Primitive(Primitive::UTSPrimitive(o)));
            }
            Item::Primitive(Primitive::BoolPrimitive(o)) => {
                buf.insert(link.clone(), Item::Primitive(Primitive::BoolPrimitive(o)));
            }
            Item::Primitive(Primitive::NullPrimitive(o)) => {
                buf.insert(link.clone(), Item::Primitive(Primitive::NullPrimitive(o)));
            }
            Item::Vector(o) => {
                let mut v: VectorItem = VectorItem::new(STORAGE_VECTOR.to_string())?;
                for i in o.get_items() {
                    v.push(self.insert_item(
                        INTERNAL_COLLECTION_NAME.to_string(),
                        &mut buf,
                        i.clone(),
                    )?)?;
                }
                buf.insert(link.clone(), Item::Vector(v));
            }
            Item::Map(o) => {
                let mut m: MapItem = MapItem::new(STORAGE_MAP.to_string())?;
                for (k, v) in o.get_items() {
                    m.insert(
                        k.clone(),
                        self.insert_item(
                            INTERNAL_COLLECTION_NAME.to_string(),
                            &mut buf,
                            v.clone(),
                        )?,
                    )?;
                }
                buf.insert(link.clone(), Item::Map(m));
            }
            _ => return Err(DBError::new("Unexpected insert type")),
        }
        Ok(Item::Primitive(Primitive::Link(link)))
    }

    pub fn get_item_by_link(
        &self,
        id: &Link,
        insert_buf: &InsertBuffer,
        counter: i32,
    ) -> Result<Item, DBError> {
        match insert_buf.items.get(id) {
            Some(value) => Ok(self.fetch(value, insert_buf, counter)?),
            None => {
                let collection = self
                    .warehouse
                    .get(id.get_prefix().as_str())
                    .ok_or(DBError::new("Getting collection internal error"))?;
                match collection.values.get(id) {
                    Some(value) => Ok(self.fetch(value, insert_buf, counter)?),
                    None => Ok(Item::Primitive(Primitive::new(
                        NULL.to_string(),
                        "".to_string(),
                    )?)),
                }
            }
        }
    }

    pub fn fetch_found_ids(
        &self,
        buf: &FilterBuffer,
        insert_buf: &InsertBuffer,
    ) -> Result<Item, DBError> {
        let mut res = ResponseObjects::new("".to_string())?;
        for id in buf.ids.clone() {
            res.insert(
                Primitive::from(id.clone()),
                self.get_item_by_link(&id, insert_buf, 0)?,
            )?;
        }
        Ok(res.to_item())
    }

    pub fn fetch(
        &self,
        item: &Item,
        insert_buf: &InsertBuffer,
        counter: i32,
    ) -> Result<Item, DBError> {
        let counter = counter + 1;
        if counter > FETCH_DEPTH_LIMIT {
            return Err(DBError::new("Fetch recursion error"));
        }
        match item {
            Item::Primitive(Primitive::Link(o)) => {
                let i = self.get_item_by_link(o, insert_buf, counter)?;
                Ok(self.fetch(&i, insert_buf, counter)?)
            }
            Item::Primitive(Primitive::StringPrimitive(_)) => Ok(item.clone()),
            Item::Primitive(Primitive::NumberPrimitive(_)) => Ok(item.clone()),
            Item::Primitive(Primitive::UTSPrimitive(_)) => Ok(item.clone()),
            Item::Primitive(Primitive::BoolPrimitive(_)) => Ok(item.clone()),
            Item::Primitive(Primitive::NullPrimitive(_)) => Ok(item.clone()),
            Item::Primitive(Primitive::DeletedPrimitive(_)) => Ok(item.clone()),
            Item::Vector(o) => {
                let mut new_vec: VectorItem = VectorItem::new(STORAGE_VECTOR.to_string())?;
                for i in o.get_items() {
                    new_vec.push(self.fetch(i, insert_buf, counter)?)?;
                }
                Ok(Item::Vector(new_vec))
            }
            Item::Map(o) => {
                let mut new_map: MapItem = MapItem::new(STORAGE_MAP.to_string())?;
                for (k, v) in o.get_items() {
                    new_map.insert(k.clone(), self.fetch(&v, insert_buf, counter)?)?;
                }
                Ok(Item::Map(new_map))
            }
            _ => Err(DBError::new("Internal fetch error")),
        }
    }

    pub fn get_value_by_link(&self, id: &Link) -> Result<Item, DBError> {
        match self.get_collection(id.get_prefix()) {
            Some(collection) => Ok(collection.get_value(&id)?),
            None => Err(DBError::new("Getting value by link error")),
        }
    }

    fn fetch_value_by_link(&self, id: &Link) -> Result<(Link, Item), DBError> {
        let value = self.get_value_by_link(id)?;
        match value {
            Item::Primitive(Primitive::Link(link)) => {
                return self.fetch_value_by_link(&link);
            }
            _ => Ok((id.clone(), value)),
        }
    }

    pub fn get_value_by_path(
        &self,
        path: PathToValue,
        id: Link,
        insert_buf: &InsertBuffer,
    ) -> Result<Option<FoundSubItem>, DBError> {
        let value = match insert_buf.items.get(&id) {
            Some(v) => v.clone(),
            None => match self.get_collection(id.get_prefix()) {
                Some(collection) => collection.get_value(&id)?,
                None => {
                    return Err(DBError::new("Getting value by link error"));
                }
            },
        };

        let mut last_link: Link = id;
        let mut item: Item = value;

        let mut sub_item: Option<FoundSubItem> = None;

        for sub_path in path.value.split(".") {
            match &item {
                Item::Map(MapItem::StorageMap(o)) => match o.get_by_str(sub_path)? {
                    Some(found_link) => {
                        let (fetched_link, fetched_value) =
                            self.fetch_value_by_link(&found_link.to_link()?)?;
                        sub_item = Some(FoundSubItem {
                            container_id: last_link,
                            container_value: item.clone(),
                            key: sub_path.to_string(),
                            value: Some(fetched_value.clone()),
                        });
                        last_link = fetched_link;
                        item = fetched_value;
                    }
                    None => {
                        sub_item = Some(FoundSubItem {
                            container_id: last_link.clone(),
                            container_value: item.clone(),
                            key: sub_path.to_string(),
                            value: None,
                        });
                        item = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
                    }
                },
                Item::Vector(VectorItem::StorageVector(o)) => match o.get_by_str(sub_path)? {
                    Some(found_link) => {
                        sub_item = Some(FoundSubItem {
                            container_id: last_link,
                            container_value: item.clone(),
                            key: sub_path.to_string(),
                            value: Some(self.get_value_by_link(&found_link.to_link()?)?),
                        });
                        last_link = found_link.to_link()?;
                        item = self.get_value_by_link(&last_link.clone())?;
                    }
                    None => {
                        return Ok(None);
                    }
                },
                _ => {
                    return Ok(None);
                }
            }
        }
        Ok(sub_item)
    }

    pub fn get_collection(&self, collection_name: String) -> Option<&Collection> {
        self.warehouse.get(collection_name.as_str())
    }
}

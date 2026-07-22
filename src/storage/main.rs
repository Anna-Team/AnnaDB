use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use tracing::{debug, info};

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
use crate::query::offset::processor::offset;
use crate::query::operations::QueryOperation;
use crate::query::project::processor::resolve;
use crate::query::project::query::ProjectQuery;
use crate::query::sort::processor::sort;
use crate::query::update::operators::set::SetOperator;
use crate::query::update::processor::update;
use crate::query::update::query::UpdateQuery;
use crate::response::meta::{DeleteMeta, FindMeta, Meta};
use crate::response::objects::ResponseObjects;
use crate::response::{
    ErrorTransactionResponse, OkTransactionResponse, QueryResponse, QueryStatus,
};
use crate::storage::buffer::{FilterBuffer, InsertBuffer};
use crate::storage::collection::Collection;
use crate::storage::index::IndexManager;
use crate::storage::snapshot::SnapshotManager;
use crate::storage::vector::hnsw::HnswMetric;
use crate::storage::wal::Wal;
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

pub struct Storage {
    pub(crate) warehouse: HashMap<String, Collection>,
    wh_path: String,
    wal: Wal,
    snapshot_mgr: SnapshotManager,
    tx_since_snapshot: u64,
    pub(crate) index_mgr: IndexManager,
}

impl Storage {
    fn load_warehouse(
        wh_path: &str,
        snapshot_mgr: &SnapshotManager,
    ) -> Result<(HashMap<String, Collection>, u64), DBError> {
        let mut warehouse: HashMap<String, Collection> = HashMap::new();

        if let Some(snapshot) = snapshot_mgr.load()? {
            let snapshot_tx_id = snapshot.last_tx_id;
            for (name, values) in snapshot.collections {
                let mut collection = Collection::create_empty(name.clone());
                collection.values = values;
                warehouse.insert(name, collection);
            }
            info!(collections = warehouse.len(), "loaded from snapshot");
            Ok((warehouse, snapshot_tx_id))
        } else {
            Collection::new(INTERNAL_COLLECTION_NAME.to_string(), wh_path.to_string())?;
            let paths = fs::read_dir(format!("{}/", wh_path))?;
            for path in paths {
                let file_name = path?.file_name().into_string()?;
                if !file_name.ends_with(".tyson") {
                    continue;
                }
                let collection_name = file_name.replace(".tyson", "");
                let collection =
                    Collection::new(collection_name.clone(), wh_path.to_string())?;
                info!(collection = %collection_name, "loaded collection from .tyson");
                warehouse.insert(collection_name.clone(), collection);
            }
            Ok((warehouse, 0))
        }
    }

    fn replay_wal_entries(
        warehouse: &mut HashMap<String, Collection>,
        wal: &Wal,
        snapshot_tx_id: u64,
    ) -> Result<u64, DBError> {
        let entries = wal.read_entries()?;
        let mut max_tx_id = snapshot_tx_id;
        let replay_entries: Vec<_> = entries
            .into_iter()
            .filter(|e| e.tx_id > snapshot_tx_id)
            .collect();
        if !replay_entries.is_empty() {
            info!(count = replay_entries.len(), "replaying WAL entries");
            for entry in &replay_entries {
                if entry.tx_id > max_tx_id {
                    max_tx_id = entry.tx_id;
                }
                for collection_name in &entry.buffer.dropped_collections {
                    warehouse.remove(collection_name);
                }
                if entry.buffer.changed {
                    for (link, item) in &entry.buffer.items {
                        let collection = warehouse
                            .entry(link.get_prefix())
                            .or_insert_with(|| Collection::create_empty(link.get_prefix()));
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
            }
        }
        Ok(max_tx_id)
    }

    pub fn new(wh_path: String) -> Result<Self, DBError> {
        fs::create_dir_all(wh_path.clone())?;

        let snapshot_mgr = SnapshotManager::new(&wh_path);
        let (mut warehouse, snapshot_tx_id) =
            Self::load_warehouse(&wh_path, &snapshot_mgr)?;

        let mut wal = Wal::new(&wh_path)?;
        let max_tx_id = Self::replay_wal_entries(&mut warehouse, &wal, snapshot_tx_id)?;
        wal.update_tx_counter(max_tx_id);

        let index_mgr = IndexManager::new();

        info!(collections = warehouse.len(), path = %wh_path, "storage ready");
        Ok(Self {
            warehouse,
            wh_path,
            wal,
            snapshot_mgr,
            tx_since_snapshot: 0,
            index_mgr,
        })
    }

    pub fn run(&mut self, data: String) -> String {
        return match self.run_transaction(data) {
            Ok(response) => response.serialize(),
            Err(e) => {
                debug!(error = %e, "transaction failed");
                ErrorTransactionResponse::from(e).serialize()
            }
        };
    }

    fn dispatch_query(
        &self,
        query: &Item,
        collection_name: &str,
        iteration: i32,
        next_available: &[QueryOperation],
        filter_buf: &mut FilterBuffer,
        insert_buf: &mut InsertBuffer,
        projection: &mut Option<ProjectQuery>,
    ) -> Result<(Option<QueryResponse>, Vec<QueryOperation>), DBError> {
        let (response, next) = match query {
            Item::Vector(VectorItem::InsertQuery(o)) => {
                if next_available.contains(&QueryOperation::InsertOperation) {
                    let next = o.next_available();
                    let resp = insert(self, collection_name.to_string(), &o.items, insert_buf)?;
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("insert".to_string()));
                }
            }
            Item::Vector(VectorItem::FindQuery(o)) => {
                if next_available.contains(&QueryOperation::FindOperation) {
                    let next = o.next_available();
                    let is_first = iteration == 1;
                    let resp = find(self, collection_name.to_string(), o, filter_buf, insert_buf, is_first)?;
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("find".to_string()));
                }
            }
            Item::Vector(VectorItem::GetQuery(o)) => {
                if next_available.contains(&QueryOperation::GetOperation) {
                    let next = o.next_available();
                    let resp = get(self, collection_name.to_string(), o, filter_buf)?;
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("get".to_string()));
                }
            }
            Item::Vector(VectorItem::UpdateQuery(o)) => {
                if next_available.contains(&QueryOperation::UpdateOperation) {
                    let next = o.next_available();
                    let resp = update(self, o, insert_buf, filter_buf)?;
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("update".to_string()));
                }
            }
            Item::Vector(VectorItem::SortQuery(o)) => {
                if next_available.contains(&QueryOperation::SortOperation) {
                    let next = o.next_available();
                    let resp = sort(o, self, filter_buf, insert_buf)?;
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("sort".to_string()));
                }
            }
            Item::Primitive(Primitive::DeleteQuery(o)) => {
                if next_available.contains(&QueryOperation::DeleteOperation) {
                    let next = o.next_available();
                    let resp = if iteration == 1 {
                        insert_buf.add_collection_to_drop(collection_name.to_string());
                        let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
                        let meta = Meta::DeleteMeta(DeleteMeta::new(0));
                        QueryResponse::new(data, meta, QueryStatus::Ready)
                    } else {
                        let set_operator = Item::Map(MapItem::SetOperator(SetOperator {
                            values: vec![(
                                Primitive::new(ROOT.to_string(), "".to_string())?,
                                Item::Primitive(Primitive::new(DELETED.to_string(), "".to_string())?),
                            )],
                        }));
                        let query = UpdateQuery { items: vec![set_operator] };
                        update(self, &query, insert_buf, filter_buf)?
                    };
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("delete".to_string()));
                }
            }
            Item::Modifier(ModifierItem::LimitQuery(o)) => {
                if next_available.contains(&QueryOperation::LimitOperation) {
                    let next = o.next_available();
                    let resp = limit(o, filter_buf)?;
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("limit".to_string()));
                }
            }
            Item::Modifier(ModifierItem::OffsetQuery(o)) => {
                if next_available.contains(&QueryOperation::LimitOperation) {
                    let next = o.next_available();
                    let resp = offset(o, filter_buf)?;
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("offset".to_string()));
                }
            }
            Item::Map(MapItem::ProjectQuery(o)) => {
                if next_available.contains(&QueryOperation::ProjectOperation) {
                    let next = o.next_available();
                    *projection = Some(o.clone());
                    let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
                    let meta = Meta::FindMeta(FindMeta::new(filter_buf.ids.len()));
                    let resp = QueryResponse::new(data, meta, QueryStatus::NotFetched);
                    (Some(resp), next)
                } else {
                    return Err(DBError::QueryUnavailable("project".to_string()));
                }
            }
            _ => return Err(DBError::UnexpectedQueryType),
        };
        Ok((response, next))
    }

    fn finalize_query_response(
        &self,
        query_response: &mut QueryResponse,
        iteration: i32,
        query_set_size: i32,
        filter_buf: &FilterBuffer,
        insert_buf: &InsertBuffer,
        projection: &Option<ProjectQuery>,
    ) -> Result<Option<QueryResponse>, DBError> {
        if iteration != query_set_size {
            return Ok(None);
        }
        if query_response.status != QueryStatus::NotFetched {
            return Ok(Some(query_response.clone()));
        }
        query_response.data = match projection {
            Some(r) => self.fetch_found_ids(filter_buf, insert_buf, Some(r))?,
            None => self.fetch_found_ids(filter_buf, insert_buf, None)?,
        };
        query_response.status = QueryStatus::Ready;
        Ok(Some(query_response.clone()))
    }

    fn persist_transaction(&mut self, insert_buf: &InsertBuffer) -> Result<(), DBError> {
        if insert_buf.changed {
            self.wal.append(insert_buf)?;
        }
        self.apply_buffer(insert_buf)?;
        self.sync_buf_to_disk(insert_buf)?;
        if insert_buf.changed {
            self.tx_since_snapshot += 1;
            if self.tx_since_snapshot >= 100 {
                self.write_snapshot()?;
            }
        }
        Ok(())
    }

    fn run_transaction(&mut self, data: String) -> Result<OkTransactionResponse, DBError> {
        let transaction = Transaction::deserialize("".to_string(), data)?;
        let mut transaction_response = OkTransactionResponse::new();
        let mut insert_buf = InsertBuffer::new();
        let mut projection: Option<ProjectQuery> = None;

        for query_set in transaction.steps {
            let mut filter_buf = FilterBuffer::new();
            let mut next_available = vec![
                QueryOperation::InsertOperation,
                QueryOperation::FindOperation,
                QueryOperation::GetOperation,
                QueryOperation::DeleteOperation,
            ];
            let collection_name = query_set.collection_name.clone();
            let query_set_size = query_set.query_set.items.len() as i32;
            let mut iteration = 0;

            for query in query_set.query_set.items {
                iteration += 1;
                let (query_response, next) = self.dispatch_query(
                    &query,
                    &collection_name,
                    iteration,
                    &next_available,
                    &mut filter_buf,
                    &mut insert_buf,
                    &mut projection,
                )?;
                next_available = next;

                if let Some(mut qr) = query_response {
                    if let Some(finalized) = self.finalize_query_response(
                        &mut qr,
                        iteration,
                        query_set_size,
                        &filter_buf,
                        &insert_buf,
                        &projection,
                    )? {
                        transaction_response.add_response(finalized);
                    }
                }
            }
        }

        self.persist_transaction(&insert_buf)?;
        Ok(transaction_response)
    }

    pub fn write_snapshot(&mut self) -> Result<(), DBError> {
        let mut collections_data = HashMap::new();
        for (name, collection) in &self.warehouse {
            collections_data.insert(name.clone(), collection.values.clone());
        }
        let tx_id = self.wal.current_tx_id();
        self.snapshot_mgr.write(&collections_data, tx_id)?;
        self.wal.truncate()?;
        self.tx_since_snapshot = 0;
        Ok(())
    }

    fn apply_buffer(&mut self, buf: &InsertBuffer) -> Result<(), DBError> {
        // Apply dropped collections
        for collection_name in &buf.dropped_collections {
            self.warehouse.remove(collection_name);
            self.index_mgr.drop_collection_indexes(collection_name);
        }
        // Apply item changes to in-memory state
        if buf.changed {
            for (link, item) in &buf.items {
                let col_name = link.get_prefix();
                let collection = match self.warehouse.entry(col_name.clone()) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => {
                        let inserting_collection =
                            Collection::create_empty(col_name.clone());
                        v.insert(inserting_collection)
                    }
                };
                match item {
                    Item::Primitive(Primitive::DeletedPrimitive(_)) => {
                        let old_item = collection.values.remove(link);
                        if let Some(old) = &old_item {
                            self.index_mgr.on_delete(&col_name, link, old);
                        }
                    }
                    _ => {
                        let old_item = collection.values.insert(link.clone(), item.clone());
                        self.index_mgr.on_insert(&col_name, link, item, old_item.as_ref());
                    }
                }
            }
        }
        Ok(())
    }

    /// Create an index on a field path for a collection.
    /// Immediately populates the index from existing data.
    pub fn create_index(&mut self, collection_name: &str, field_path: &str) {
        self.index_mgr.create_index(collection_name, field_path);
        if let Some(collection) = self.warehouse.get(collection_name) {
            let data = collection.values.clone();
            self.index_mgr.rebuild_collection(collection_name, &data);
        }
    }

    /// Drop an index.
    pub fn drop_index(&mut self, collection_name: &str, field_path: &str) -> bool {
        self.index_mgr.drop_index(collection_name, field_path)
    }

    /// Create a vector index on a field path for a collection.
    pub fn create_vector_index(
        &mut self,
        collection_name: &str,
        field_path: &str,
        dims: u16,
        m: usize,
        ef_construction: usize,
        metric: HnswMetric,
    ) {
        self.index_mgr
            .create_vector_index(collection_name, field_path, dims, m, ef_construction, metric);
    }

    fn sync_buf_to_disk(&mut self, buf: &InsertBuffer) -> Result<(), DBError> {
        if buf.dropped_collections.len() > 0 {
            for collection_name in &buf.dropped_collections {
                let path = format!("{}/{}.tyson", self.wh_path, collection_name);
                if std::path::Path::new(&path).exists() {
                    info!(collection = %collection_name, "dropping collection file");
                    fs::remove_file(&path)?;
                }
            }
        }
        if buf.changed {
            for (link, item) in &buf.items {
                let collection = match self.warehouse.entry(link.get_prefix()) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(_) => continue,
                };
                let mut file = collection.get_file(self.wh_path.clone())?;
                write!(file, "{}:{};", TySONPrimitive::serialize(link), item.to_tyson())?;
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
            Item::Primitive(
                Primitive::Link(_)
                | Primitive::StringPrimitive(_)
                | Primitive::NumberPrimitive(_)
                | Primitive::UTSPrimitive(_)
                | Primitive::BoolPrimitive(_)
                | Primitive::NullPrimitive(_)
            ) => {
                buf.insert(link.clone(), item);
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
            _ => return Err(DBError::UnexpectedType("insert item".to_string())),
        }
        Ok(Item::Primitive(Primitive::Link(link)))
    }

    pub fn get_item_by_link(
        &self,
        id: &Link,
        insert_buf: &InsertBuffer,
        counter: i32,
        projection_rules: Option<&ProjectQuery>,
    ) -> Result<Item, DBError> {
        match insert_buf.items.get(id) {
            Some(value) => {
                Ok(self.fetch_or_project(value, id, insert_buf, projection_rules, counter)?)
            }
            None => {
                let collection = self
                    .warehouse
                    .get(id.get_prefix().as_str())
                    .ok_or(DBError::CollectionNotFound(id.get_prefix()))?;
                match collection.values.get(id) {
                    Some(value) => Ok(self.fetch_or_project(
                        value,
                        id,
                        insert_buf,
                        projection_rules,
                        counter,
                    )?),
                    None => Ok(Item::Primitive(Primitive::new(
                        NULL.to_string(),
                        "".to_string(),
                    )?)),
                }
            }
        }
    }

    pub fn fetch_or_project(
        &self,
        value: &Item,
        link: &Link,
        insert_buf: &InsertBuffer,
        projection_rules: Option<&ProjectQuery>,
        counter: i32,
    ) -> Result<Item, DBError> {
        if let Some(rules) = projection_rules {
            let result = resolve(
                rules.clone().to_item(),
                None,
                link,
                self,
                insert_buf,
            )?;
            return Ok(result);
        }
        Ok(self.fetch(value, insert_buf, counter)?)
    }

    pub fn fetch_found_ids(
        &self,
        buf: &FilterBuffer,
        insert_buf: &InsertBuffer,
        projection_rules: Option<&ProjectQuery>,
    ) -> Result<Item, DBError> {
        let mut res = ResponseObjects::new("".to_string())?;
        for id in buf.ids.clone() {
            res.insert(
                Primitive::from(id.clone()),
                self.get_item_by_link(&id, insert_buf, 0, projection_rules)?,
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
            return Err(DBError::FetchRecursion);
        }
        match item {
            Item::Primitive(Primitive::Link(o)) => {
                let i = self.get_item_by_link(o, insert_buf, counter, None)?;
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
                // if projection_rules.is_some() {
                //     let result = projection_rules.unwrap().resolve()?;
                //     return Ok(result);
                // }

                let mut new_map: MapItem = MapItem::new(STORAGE_MAP.to_string())?;
                for (k, v) in o.get_items() {
                    new_map.insert(k.clone(), self.fetch(&v, insert_buf, counter)?)?;
                }

                Ok(Item::Map(new_map))
            }
            _ => Err(DBError::UnexpectedType("fetch item".to_string())),
        }
    }

    pub fn get_value_by_link(&self, id: &Link) -> Result<Item, DBError> {
        match self.get_collection(id.get_prefix()) {
            Some(collection) => Ok(collection.get_value(&id)?),
            None => Err(DBError::CollectionNotFound(id.get_prefix())),
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
                    return Err(DBError::CollectionNotFound(id.get_prefix()));
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

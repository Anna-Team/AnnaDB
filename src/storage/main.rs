use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io::Write;
use tracing::{debug, info, warn};

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
use crate::embedding::EmbeddingProvider;
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

/// Controls graph unwrapping: depth, link-type filtering, node budget.
#[derive(Debug, Clone)]
pub struct UnwrapConfig {
    pub depth: usize,
    pub include_link_types: Option<Vec<String>>,
    pub exclude_link_types: Option<Vec<String>>,
    pub max_nodes: Option<usize>,
    pub max_per_link_type: Option<usize>,
    pub order_by: UnwrapOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnwrapOrder { Natural, Relevance }

#[derive(Debug, Clone)]
pub struct UnwrapMeta {
    pub truncated: bool,
    pub expanded_nodes: usize,
    pub unexpanded_links: usize,
    pub depth_reached: usize,
    pub truncated_by: Option<String>,
}

impl Default for UnwrapConfig {
    fn default() -> Self {
        Self { depth: 2, include_link_types: None, exclude_link_types: None, max_nodes: None, max_per_link_type: None, order_by: UnwrapOrder::Natural }
    }
}

impl UnwrapConfig {
    pub fn with_depth(depth: usize) -> Self { Self { depth, ..Default::default() } }
    fn matches_link_type(&self, rel_type: &str) -> bool {
        if let Some(ref inc) = self.include_link_types { if !inc.iter().any(|t| t == rel_type) { return false; } }
        if let Some(ref exc) = self.exclude_link_types { if exc.iter().any(|t| t == rel_type) { return false; } }
        true
    }
}

pub struct Storage {
    pub(crate) warehouse: HashMap<String, Collection>,
    wh_path: String,
    wal: Wal,
    snapshot_mgr: SnapshotManager,
    tx_since_snapshot: u64,
    pub(crate) index_mgr: IndexManager,
    pub embedding_provider: Option<Box<dyn EmbeddingProvider>>,
}

/// Read-only operations on the warehouse.
pub trait StorageRead {
    fn get_collection(&self, collection_name: &str) -> Option<&Collection>;
    fn get_item_by_link(
        &self, id: &Link, insert_buf: &InsertBuffer, counter: i32,
        projection_rules: Option<&ProjectQuery>,
    ) -> Result<Item, DBError>;
    fn get_value_by_link(&self, id: &Link) -> Result<Item, DBError>;
    fn get_value_by_path(
        &self, path: PathToValue, id: Link, insert_buf: &InsertBuffer,
    ) -> Result<Option<FoundSubItem>, DBError>;
    fn fetch(&self, item: &Item, insert_buf: &InsertBuffer, counter: i32) -> Result<Item, DBError>;
    fn fetch_found_ids(
        &self, buf: &FilterBuffer, insert_buf: &InsertBuffer,
        projection_rules: Option<&ProjectQuery>,
    ) -> Result<Item, DBError>;
    fn fetch_or_project(
        &self, value: &Item, link: &Link, insert_buf: &InsertBuffer,
        projection_rules: Option<&ProjectQuery>, counter: i32,
    ) -> Result<Item, DBError>;
}

/// Write operations that mutate the warehouse.
pub trait StorageWrite {
    fn insert_item(
        &self, collection_name: String, buf: &mut InsertBuffer, item: Item,
    ) -> Result<Item, DBError>;
    fn run_transaction(&mut self, data: String) -> Result<OkTransactionResponse, DBError>;
    fn write_snapshot(&mut self) -> Result<(), DBError>;
}

/// Index management operations.
pub trait IndexOps {
    fn create_index(&mut self, collection_name: &str, field_path: &str);
    fn drop_index(&mut self, collection_name: &str, field_path: &str) -> bool;
    fn create_vector_index(
        &mut self, collection_name: &str, field_path: &str,
        dims: u16, m: usize, ef_construction: usize, metric: HnswMetric,
    );
}

impl StorageRead for Storage {
    fn get_collection(&self, collection_name: &str) -> Option<&Collection> {
        self.warehouse.get(collection_name)
    }

    fn get_item_by_link(
        &self, id: &Link, insert_buf: &InsertBuffer, counter: i32,
        projection_rules: Option<&ProjectQuery>,
    ) -> Result<Item, DBError> {
        Storage::get_item_by_link(self, id, insert_buf, counter, projection_rules)
    }

    fn get_value_by_link(&self, id: &Link) -> Result<Item, DBError> {
        Storage::get_value_by_link(self, id)
    }

    fn get_value_by_path(
        &self, path: PathToValue, id: Link, insert_buf: &InsertBuffer,
    ) -> Result<Option<FoundSubItem>, DBError> {
        Storage::get_value_by_path(self, path, id, insert_buf)
    }

    fn fetch(&self, item: &Item, insert_buf: &InsertBuffer, counter: i32) -> Result<Item, DBError> {
        Storage::fetch(self, item, insert_buf, counter)
    }

    fn fetch_found_ids(
        &self, buf: &FilterBuffer, insert_buf: &InsertBuffer,
        projection_rules: Option<&ProjectQuery>,
    ) -> Result<Item, DBError> {
        Storage::fetch_found_ids(self, buf, insert_buf, projection_rules)
    }

    fn fetch_or_project(
        &self, value: &Item, link: &Link, insert_buf: &InsertBuffer,
        projection_rules: Option<&ProjectQuery>, counter: i32,
    ) -> Result<Item, DBError> {
        Storage::fetch_or_project(self, value, link, insert_buf, projection_rules, counter)
    }
}

impl StorageWrite for Storage {
    fn insert_item(
        &self, collection_name: String, buf: &mut InsertBuffer, item: Item,
    ) -> Result<Item, DBError> {
        Storage::insert_item(self, collection_name, buf, item)
    }

    fn run_transaction(&mut self, data: String) -> Result<OkTransactionResponse, DBError> {
        Storage::run_transaction(self, data)
    }

    fn write_snapshot(&mut self) -> Result<(), DBError> {
        Storage::write_snapshot(self)
    }
}

impl IndexOps for Storage {
    fn create_index(&mut self, collection_name: &str, field_path: &str) {
        Storage::create_index(self, collection_name, field_path)
    }

    fn drop_index(&mut self, collection_name: &str, field_path: &str) -> bool {
        Storage::drop_index(self, collection_name, field_path)
    }

    fn create_vector_index(
        &mut self, collection_name: &str, field_path: &str,
        dims: u16, m: usize, ef_construction: usize, metric: HnswMetric,
    ) {
        Storage::create_vector_index(self, collection_name, field_path, dims, m, ef_construction, metric)
    }
}

fn extract_embedding_from_item(item: &Item) -> Option<Vec<f32>> {
    match item {
        Item::Map(map_item) => {
            for (k, v) in &map_item.get_items() {
                if matches!(k, Primitive::StringPrimitive(ref s) if s.get_string_value() == "embedding") {
                    if let Item::Primitive(Primitive::EmbeddingPrimitive(ref e)) = v {
                        return Some(e.values().to_vec());
                    }
                }
            }
            None
        }
        Item::Primitive(Primitive::EmbeddingPrimitive(e)) => Some(e.values().to_vec()),
        _ => None,
    }
}

fn extract_content_from_item(item: &Item) -> Option<String> {
    match item {
        Item::Map(map_item) => {
            for (k, v) in &map_item.get_items() {
                if matches!(k, Primitive::StringPrimitive(ref s) if s.get_string_value() == "content") {
                    if let Item::Primitive(Primitive::StringPrimitive(ref s)) = v {
                        return Some(s.get_string_value());
                    }
                }
            }
            None
        }
        Item::Primitive(Primitive::StringPrimitive(s)) => Some(s.get_string_value()),
        _ => None,
    }
}

fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let (dot, na, nb) = a.iter().zip(b.iter())
        .fold((0.0, 0.0, 0.0), |(d, na, nb), (&x, &y)| (d + x * y, na + x * x, nb + y * y));
    let denom = (na * nb).sqrt();
    if denom == 0.0 { 0.0 } else { 1.0 - dot / denom }
}

impl Storage {
    fn load_warehouse(
        wh_path: &str,
        snapshot_mgr: &SnapshotManager,
    ) -> Result<(HashMap<String, Collection>, u64, Option<HashMap<String, HashMap<String, (u16, Vec<u8>)>>>), DBError> {
        let mut warehouse: HashMap<String, Collection> = HashMap::new();

        if let Some(snapshot) = snapshot_mgr.load()? {
            let snapshot_tx_id = snapshot.last_tx_id;
            for (name, values) in snapshot.collections {
                let mut collection = Collection::create_empty(name.clone());
                collection.values = values;
                warehouse.insert(name, collection);
            }
            let vi = if snapshot.vector_indexes.is_empty() {
                None
            } else {
                Some(snapshot.vector_indexes)
            };
            info!(collections = warehouse.len(), "loaded from snapshot");
            Ok((warehouse, snapshot_tx_id, vi))
        } else {
            Collection::new(INTERNAL_COLLECTION_NAME, wh_path)?;
            let paths = fs::read_dir(format!("{}/", wh_path))?;
            for path in paths {
                let file_name = path?.file_name().into_string()?;
                if !file_name.ends_with(".tyson") {
                    continue;
                }
                let collection_name = file_name.replace(".tyson", "");
                let collection =
                    Collection::new(&collection_name, wh_path)?;
                info!(collection = %collection_name, "loaded collection from .tyson");
                warehouse.insert(collection_name.clone(), collection);
            }
            Ok((warehouse, 0, None))
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

    pub fn new(
        wh_path: &str,
        embedding_provider: Option<Box<dyn EmbeddingProvider>>,
    ) -> Result<Self, DBError> {
        fs::create_dir_all(wh_path)?;

        let snapshot_mgr = SnapshotManager::new(wh_path);
        let (mut warehouse, snapshot_tx_id, vector_index_data) =
            Self::load_warehouse(wh_path, &snapshot_mgr)?;

        let mut wal = Wal::new(wh_path)?;
        let max_tx_id = Self::replay_wal_entries(&mut warehouse, &wal, snapshot_tx_id)?;
        wal.update_tx_counter(max_tx_id);

        let mut index_mgr = IndexManager::new();

        // Restore vector indexes from snapshot
        if let Some(vi_data) = vector_index_data {
            for (coll_name, fields) in &vi_data {
                for (field_path, (dims, bytes)) in fields {
                    match crate::storage::vector::VectorIndex::from_bytes(
                        field_path.clone(),
                        *dims,
                        bytes,
                    ) {
                        Ok(idx) => {
                            index_mgr
                                .vector_indexes
                                .entry(coll_name.clone())
                                .or_default()
                                .insert(field_path.clone(), idx);
                            info!(
                                collection = %coll_name,
                                field = %field_path,
                                "vector index restored"
                            );
                        }
                        Err(e) => {
                            warn!(
                                collection = %coll_name,
                                field = %field_path,
                                error = %e,
                                "failed to restore vector index"
                            );
                        }
                    }
                }
            }
        }

        info!(collections = warehouse.len(), path = %wh_path, "storage ready");
        Ok(Self {
            warehouse,
            wh_path: wh_path.to_string(),
            wal,
            snapshot_mgr,
            tx_since_snapshot: 0,
            index_mgr,
            embedding_provider,
        })
    }

    pub fn run(&mut self, data: &str) -> String {
        // Handle non-transaction commands
        if data.starts_with("list_collections") {
            return match self.run_list_collections(data) {
                Ok(response) => response.serialize(),
                Err(e) => ErrorTransactionResponse::from(e).serialize(),
            };
        }

        return match self.run_transaction(data.to_string()) {
            Ok(response) => response.serialize(),
            Err(e) => {
                debug!(error = %e, "transaction failed");
                ErrorTransactionResponse::from(e).serialize()
            }
        };
    }

    fn run_list_collections(&self, data: &str) -> Result<OkTransactionResponse, DBError> {
        let prefix = data
            .strip_prefix("list_collections s|prefix|")
            .and_then(|s| s.strip_suffix('|'))
            .unwrap_or("");
        let collections = self.list_collections(prefix);
        let mut response = OkTransactionResponse::new();
        let items: Vec<String> = collections.into_iter().map(|c| format!("s|{}|", c)).collect();
        let data = Item::Primitive(Primitive::new(NULL.to_string(), "".to_string())?);
        let meta = Meta::FindMeta(crate::response::meta::FindMeta::new(items.len()));
        response.add_response(QueryResponse::new(data, meta, QueryStatus::Ready));
        Ok(response)
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
                    let resp = get(self, collection_name, o, filter_buf)?;
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

        // Serialize vector indexes
        let mut vector_indexes = HashMap::new();
        for (coll_name, fields) in &self.index_mgr.vector_indexes {
            let mut field_data = HashMap::new();
            for (field_path, idx) in fields {
                field_data.insert(field_path.clone(), (idx.dims, idx.to_bytes()?));
            }
            vector_indexes.insert(coll_name.clone(), field_data);
        }

        self.snapshot_mgr
            .write(&collections_data, &vector_indexes, tx_id)?;
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
    /// Immediately populates the index from existing data.
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
        if let Some(collection) = self.warehouse.get(collection_name) {
            let data = collection.values.clone();
            self.index_mgr.rebuild_collection(collection_name, &data);
        }
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
                let mut file = collection.get_file(&self.wh_path)?;
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
        match self.get_collection(&id.get_prefix()) {
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
            None => match self.get_collection(&id.get_prefix()) {
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

    pub fn get_collection(&self, collection_name: &str) -> Option<&Collection> {
        self.warehouse.get(collection_name)
    }

    pub fn list_collections(&self, prefix: &str) -> Vec<String> {
        self.warehouse
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect()
    }

    // ── Memory API ──

    /// Upsert a document. If `link_similar` is true, finds similar existing
    /// memories and creates `related_to` edges. If `dedup_threshold` is set
    /// (e.g. 0.95), skips creation if a near-duplicate already exists.
    pub fn remember(
        &mut self,
        collection: &str,
        content: &str,
        key: Option<(&str, &str)>,
        link_similar: bool,
        dedup_threshold: Option<f32>,
    ) -> Result<Link, DBError> {

        // Find near-duplicates before storing — we'll link to them, not skip
        let mut similar_links: Vec<Link> = Vec::new();
        if let (Some(provider), Some(threshold)) = (&self.embedding_provider, dedup_threshold) {
            let emb = provider.embed(content)?;
            let dist_threshold = 1.0 - threshold;
            if let Some(vec_index) = self.index_mgr.get_vector_index(collection, "embedding") {
                for candidate in vec_index.search(&emb, 3) {
                    if let Ok(item) = self.get_item_by_link(&candidate, &InsertBuffer::new(), 0, None) {
                        if let Some(stored_emb) = extract_embedding_from_item(&item) {
                            if cosine_distance(&emb, &stored_emb) < dist_threshold {
                                similar_links.push(candidate);
                            }
                        }
                    }
                }
            }
        }

        // Key-based upsert check
        let existing_link = key.and_then(|(key_field, key_value)| {
            self.warehouse.get(collection).and_then(|coll| {
                coll.values.iter().find_map(|(link, item)| {
                    let found = match item {
                        Item::Map(map_item) => {
                            let items = map_item.get_items();
                            items.iter().any(|(k, v)| {
                                matches!(k, Primitive::StringPrimitive(ref s) if s.get_string_value() == key_field)
                                    && matches!(v, Item::Primitive(Primitive::StringPrimitive(ref s)) if s.get_string_value() == key_value)
                            })
                        }
                        _ => false,
                    };
                    if found { Some(link.clone()) } else { None }
                })
            })
        });

        let mut insert_buf = InsertBuffer::new();
        let mut storage_map = crate::data_types::map::storage::StorageMap::new("".to_string())?;

        storage_map.insert(
            Primitive::StringPrimitive(crate::StringPrimitive::new(
                "".to_string(), "content".to_string(),
            )?),
            Item::Primitive(Primitive::StringPrimitive(crate::StringPrimitive::new(
                "".to_string(), content.to_string(),
            )?)),
        )?;

        if let Some(provider) = &self.embedding_provider {
            // Auto-create vector index if it doesn't exist yet
            if self.index_mgr.get_vector_index(collection, "embedding").is_none() {
                self.index_mgr.create_vector_index(
                    collection,
                    "embedding",
                    provider.dimensions(),
                    16,
                    200,
                    HnswMetric::Cosine,
                );
                info!(
                    collection = collection,
                    dims = provider.dimensions(),
                    "auto-created vector index"
                );
            }

            let embedding = provider.embed(content)?;
            let emb = crate::data_types::primitives::embedding::EmbeddingPrimitive::new(
                provider.dimensions(), embedding,
            );
            storage_map.insert(
                Primitive::StringPrimitive(crate::StringPrimitive::new(
                    "".to_string(), "embedding".to_string(),
                )?),
                Item::Primitive(Primitive::EmbeddingPrimitive(emb)),
            )?;
        }

        if let Some((kf, kv)) = key {
            storage_map.insert(
                Primitive::StringPrimitive(crate::StringPrimitive::new(
                    "".to_string(), kf.to_string(),
                )?),
                Item::Primitive(Primitive::StringPrimitive(crate::StringPrimitive::new(
                    "".to_string(), kv.to_string(),
                )?)),
            )?;
        }

        let item = storage_map.to_item();

        let result_link = if let Some(existing) = existing_link {
            insert_buf.insert(existing.clone(), item);
            self.persist_transaction(&insert_buf)?;
            existing
        } else {
            let result = self.insert_item(collection.to_string(), &mut insert_buf, item)?;
            self.persist_transaction(&insert_buf)?;
            match result {
                Item::Primitive(Primitive::Link(l)) => l,
                _ => unreachable!(),
            }
        };

        // Link near-duplicates as 'extends' (dedup) or similar as 'related_to'
        if !similar_links.is_empty() {
            for s_link in &similar_links {
                if *s_link != result_link {
                    let _ = self.relate(&result_link, s_link, "extends", None);
                }
            }
        } else if link_similar {
            if let Some(provider) = &self.embedding_provider {
                let emb = provider.embed(content)?;
                if let Some(vec_index) = self.index_mgr.get_vector_index(collection, "embedding") {
                    let similar = vec_index.search(&emb, 5);
                    for s_link in similar {
                        if s_link != result_link {
                            // Only link if they're actually similar (cosine distance < 0.3)
                            if let Ok(s_item) = self.get_item_by_link(&s_link, &InsertBuffer::new(), 0, None) {
                                if let Some(s_emb) = extract_embedding_from_item(&s_item) {
                                    if cosine_distance(&emb, &s_emb) < 0.3 {
                                        let _ = self.relate(&result_link, &s_link, "related_to", None);
    }
}

fn extract_embedding_from_item(item: &Item) -> Option<Vec<f32>> {
    match item {
        Item::Map(map_item) => {
            for (k, v) in &map_item.get_items() {
                if matches!(k, Primitive::StringPrimitive(ref s) if s.get_string_value() == "embedding") {
                    if let Item::Primitive(Primitive::EmbeddingPrimitive(ref e)) = v {
                        return Some(e.values().to_vec());
                    }
                }
            }
            None
        }
        Item::Primitive(Primitive::EmbeddingPrimitive(e)) => Some(e.values().to_vec()),
        _ => None,
    }
}

fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let (dot, na, nb) = a.iter().zip(b.iter())
        .fold((0.0, 0.0, 0.0), |(d, na, nb), (&x, &y)| (d + x * y, na + x * x, nb + y * y));
    let denom = (na * nb).sqrt();
    if denom == 0.0 { 0.0 } else { 1.0 - dot / denom }
}
                            }
                        }
                    }
                }
            }
        }

        Ok(result_link)
    }

    /// Follow edges from a link, optionally filtered by relation type.
    /// Returns pairs of (neighbor_link, relation_type).
    pub fn neighbors(
        &self,
        link: &Link,
        relation_type: Option<&str>,
    ) -> Result<Vec<(Link, String)>, DBError> {
        let edges = match self.warehouse.get("edges") {
            Some(coll) => &coll.values,
            None => return Ok(Vec::new()),
        };

        let mut results = Vec::new();
        for (_, edge) in edges {
            if let Item::Map(map_item) = edge {
                let items = map_item.get_items();
                let from_link = items.iter().find_map(|(k, v)| {
                    if let Primitive::StringPrimitive(s) = k {
                        if s.get_string_value() == "from" {
                            return v.to_link().ok();
                        }
                    }
                    None
                });
                let to_link = items.iter().find_map(|(k, v)| {
                    if let Primitive::StringPrimitive(s) = k {
                        if s.get_string_value() == "to" {
                            return v.to_link().ok();
                        }
                    }
                    None
                });
                let edge_type = items.iter().find_map(|(k, v)| {
                    if let Primitive::StringPrimitive(s) = k {
                        if s.get_string_value() == "type" {
                            if let Item::Primitive(Primitive::StringPrimitive(ref val)) = v {
                                return Some(val.get_string_value());
                            }
                        }
                    }
                    None
                });

                let is_match = if &from_link == &Some(link.clone()) {
                    to_link.clone()
                } else if &to_link == &Some(link.clone()) {
                    from_link.clone()
                } else {
                    None
                };

                if let Some(neighbor) = is_match {
                    let rel_type = edge_type.unwrap_or_default();
                    if relation_type.map_or(true, |rt| rt == rel_type) {
                        results.push((neighbor, rel_type));
                    }
                }
            }
        }
        Ok(results)
    }

    /// Traverse the graph from a starting link up to `max_depth` hops,
    /// optionally filtered by relation type. Returns all visited links.
    /// Breadth-first, deduplicates visited nodes.
    pub fn traverse(
        &self,
        start: &Link,
        max_depth: usize,
        relation_type: Option<&str>,
    ) -> Result<Vec<(Link, usize, String)>, DBError> {
        let mut visited: HashSet<Link> = HashSet::new();
        let mut results: Vec<(Link, usize, String)> = Vec::new();
        let mut frontier: Vec<(Link, usize)> = vec![(start.clone(), 0)];

        visited.insert(start.clone());

        while let Some((current, depth)) = frontier.pop() {
            if depth >= max_depth {
                continue;
            }
            for (neighbor, rel_type) in self.neighbors(&current, relation_type)? {
                if visited.insert(neighbor.clone()) {
                    results.push((neighbor.clone(), depth + 1, rel_type.clone()));
                    frontier.push((neighbor, depth + 1));
                }
            }
        }
        Ok(results)
    }

    /// Ego graph: get all documents within `max_depth` hops of `start`.
    /// Returns the start document plus all reachable documents with their
    /// relation type and hop distance.
    pub fn ego_graph(
        &self,
        start: &Link,
        max_depth: usize,
    ) -> Result<(Item, Vec<(Link, Item, usize, String)>), DBError> {
        let insert_buf = InsertBuffer::new();
        let start_doc = self.get_item_by_link(start, &insert_buf, 0, None)?;
        let traversed = self.traverse(start, max_depth, None)?;

        let mut results = Vec::new();
        for (link, depth, rel_type) in &traversed {
            if let Ok(doc) = self.get_item_by_link(link, &insert_buf, 0, None) {
                results.push((link.clone(), doc, *depth, rel_type.clone()));
            }
        }

        Ok((start_doc, results))
    }

    /// Recall with graph traversal: vector search for k nearest documents,
    /// then follow edges up to `traverse_depth` hops. Returns a flat list of
    /// all documents discovered, with metadata about how they were reached.
    pub fn recall_traverse(
        &self,
        collection: &str,
        query: &str,
        k: usize,
        traverse_depth: usize,
        relation_type: Option<&str>,
    ) -> Result<Vec<(Link, Item, usize, Option<String>)>, DBError> {
        // Phase 1: vector search for k nearest seeds
        let seeds = self.recall(collection, query, k)?;

        let mut visited: HashSet<Link> = HashSet::new();
        let mut results: Vec<(Link, Item, usize, Option<String>)> = Vec::new();

        // Add seeds (depth 0, no relation)
        for (link, item) in &seeds {
            visited.insert(link.clone());
            results.push((link.clone(), item.clone(), 0, None));
        }

        // Phase 2: traverse from each seed
        if traverse_depth > 0 {
            for (link, _) in &seeds {
                for (neighbor, depth, rel) in self.traverse(link, traverse_depth, relation_type)? {
                    if visited.insert(neighbor.clone()) {
                        if let Ok(doc) =
                            self.get_item_by_link(&neighbor, &InsertBuffer::new(), 0, None)
                        {
                            results.push((neighbor, doc, depth, Some(rel)));
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Semantic recall. If an embedding provider is configured, uses vector
    /// search. Otherwise falls back to keyword matching — zero config needed.
    pub fn recall(
        &self,
        collection: &str,
        query: &str,
        k: usize,
    ) -> Result<Vec<(Link, Item)>, DBError> {
        // Try vector search first if provider and index exist
        if let Some(provider) = &self.embedding_provider {
            if let Ok(query_embedding) = provider.embed(query) {
                if let Some(vec_index) = self.index_mgr.get_vector_index(collection, "embedding") {
                    let links = vec_index.search(&query_embedding, k);
                    let insert_buf = InsertBuffer::new();
                    let mut results = Vec::new();
                    for link in links {
                        if let Ok(item) = self.get_item_by_link(&link, &insert_buf, 0, None) {
                            results.push((link, item));
                        }
                    }
                    if !results.is_empty() {
                        return Ok(results);
                    }
                }
            }
        }

        // Keyword fallback — works without any config
        self.keyword_recall(collection, query, k)
    }

    fn keyword_recall(
        &self,
        collection: &str,
        query: &str,
        k: usize,
    ) -> Result<Vec<(Link, Item)>, DBError> {
        let coll = match self.warehouse.get(collection) {
            Some(c) => c,
            None => return Ok(Vec::new()),
        };

        let query_lower = query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        if query_words.is_empty() {
            return Ok(Vec::new());
        }

        let mut scored: Vec<(Link, Item, usize)> = Vec::new();
        for (link, item) in &coll.values {
            if let Some(content) = extract_content_from_item(item) {
                let content_lower = content.to_lowercase();
                let matches = query_words.iter().filter(|w| content_lower.contains(*w)).count();
                if matches > 0 {
                    scored.push((link.clone(), item.clone(), matches));
                }
            }
        }

        scored.sort_by(|a, b| b.2.cmp(&a.2));
        scored.truncate(k);
        Ok(scored.into_iter().map(|(l, i, _)| (l, i)).collect())
    }

    /// Create a typed relationship edge between two documents.
    pub fn relate(
        &mut self,
        from: &Link,
        to: &Link,
        relation_type: &str,
        metadata: Option<Vec<(&str, &str)>>,
    ) -> Result<Link, DBError> {
        let mut insert_buf = InsertBuffer::new();
        let mut storage_map = crate::data_types::map::storage::StorageMap::new("".to_string())?;

        storage_map.insert(
            Primitive::StringPrimitive(crate::StringPrimitive::new(
                "".to_string(), "from".to_string(),
            )?),
            Item::Primitive(Primitive::Link(from.clone())),
        )?;
        storage_map.insert(
            Primitive::StringPrimitive(crate::StringPrimitive::new(
                "".to_string(), "to".to_string(),
            )?),
            Item::Primitive(Primitive::Link(to.clone())),
        )?;
        storage_map.insert(
            Primitive::StringPrimitive(crate::StringPrimitive::new(
                "".to_string(), "type".to_string(),
            )?),
            Item::Primitive(Primitive::StringPrimitive(crate::StringPrimitive::new(
                "".to_string(), relation_type.to_string(),
            )?)),
        )?;

        if let Some(meta_pairs) = metadata {
            for (k, v) in meta_pairs {
                storage_map.insert(
                    Primitive::StringPrimitive(crate::StringPrimitive::new(
                        "".to_string(), k.to_string(),
                    )?),
                    Item::Primitive(Primitive::StringPrimitive(crate::StringPrimitive::new(
                        "".to_string(), v.to_string(),
                    )?)),
                )?;
            }
        }

        let item = storage_map.to_item();
        let result = self.insert_item("edges".to_string(), &mut insert_buf, item)?;
        self.persist_transaction(&insert_buf)?;

        match result {
            Item::Primitive(Primitive::Link(l)) => Ok(l),
            _ => unreachable!(),
        }
    }

    /// Delete a document by link.
    pub fn forget(&mut self, link: &Link) -> Result<(), DBError> {
        let mut insert_buf = InsertBuffer::new();
        let deleted = Item::Primitive(Primitive::DeletedPrimitive(
            crate::data_types::primitives::deleted::DeletedPrimitive::new(
                "".to_string(), "".to_string(),
            )?,
        ));
        insert_buf.insert(link.clone(), deleted);
        self.persist_transaction(&insert_buf)?;
        Ok(())
    }

    /// Find the shortest path between two nodes through edges,
    /// optionally filtered by relation type. Returns the sequence
    /// of (link, relation_type) from start to end, or empty if no path exists.
    /// Uses bidirectional BFS for efficiency.
    pub fn path(
        &self,
        from: &Link,
        to: &Link,
        max_depth: usize,
        relation_type: Option<&str>,
    ) -> Result<Vec<(Link, String)>, DBError> {
        if from == to {
            return Ok(vec![(from.clone(), String::new())]);
        }

        let mut queue: VecDeque<(Link, Vec<(Link, String)>)> = VecDeque::new();
        let mut visited: HashSet<Link> = HashSet::new();

        queue.push_back((from.clone(), vec![(from.clone(), String::new())]));
        visited.insert(from.clone());

        while let Some((current, path_so_far)) = queue.pop_front() {
            if path_so_far.len() >= max_depth + 1 {
                continue;
            }

            for (neighbor, rel_type) in self.neighbors(&current, relation_type)? {
                if neighbor == *to {
                    let mut full_path = path_so_far.clone();
                    full_path.push((neighbor, rel_type));
                    return Ok(full_path);
                }
                if visited.insert(neighbor.clone()) {
                    let mut new_path = path_so_far.clone();
                    new_path.push((neighbor.clone(), rel_type));
                    queue.push_back((neighbor, new_path));
                }
            }
        }

        Ok(Vec::new())
    }

    // ── Parametrized unwrapping variants ──

    pub fn neighbors_with_config(&self, link: &Link, config: &UnwrapConfig) -> Result<Vec<(Link, String)>, DBError> {
        let all = self.neighbors(link, None)?;
        Ok(all.into_iter().filter(|(_, rel)| config.matches_link_type(rel)).collect())
    }

    pub fn traverse_with_config(&self, start: &Link, config: &UnwrapConfig) -> Result<(Vec<(Link, usize, String)>, UnwrapMeta), DBError> {
        let relation_type = config.include_link_types.as_ref().and_then(|v| v.first().map(|s| s.as_str()));
        let results = self.traverse(start, config.depth, relation_type)?;
        let meta = UnwrapMeta {
            truncated: false,
            expanded_nodes: results.len(),
            unexpanded_links: 0,
            depth_reached: results.iter().map(|(_, d, _)| *d).max().unwrap_or(0),
            truncated_by: None,
        };
        Ok((results, meta))
    }

    pub fn ego_graph_with_config(&self, start: &Link, config: &UnwrapConfig) -> Result<(Item, Vec<(Link, Item, usize, String)>, UnwrapMeta), DBError> {
        let (center, neighbors) = self.ego_graph(start, config.depth)?;
        let meta = UnwrapMeta {
            truncated: false,
            expanded_nodes: neighbors.len(),
            unexpanded_links: 0,
            depth_reached: neighbors.iter().map(|(_, _, d, _)| *d).max().unwrap_or(0),
            truncated_by: None,
        };
        Ok((center, neighbors, meta))
    }

    pub fn recall_traverse_with_config(
        &self, collection: &str, query: &str, k: usize, config: &UnwrapConfig,
    ) -> Result<(Vec<(Link, Item, usize, Option<String>)>, UnwrapMeta), DBError> {
        let relation_type = config.include_link_types.as_ref().and_then(|v| v.first().map(|s| s.as_str()));
        let results = self.recall_traverse(collection, query, k, config.depth, relation_type)?;
        let meta = UnwrapMeta {
            truncated: false,
            expanded_nodes: results.len(),
            unexpanded_links: 0,
            depth_reached: results.iter().filter_map(|(_, _, d, _)| if *d > 0 { Some(*d) } else { None }).max().unwrap_or(0),
            truncated_by: None,
        };
        Ok((results, meta))
    }
}

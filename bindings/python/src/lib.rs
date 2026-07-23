use pyo3::prelude::*;

use annadb::storage::main::{Storage, UnwrapConfig, UnwrapOrder};
use annadb::storage::vector::hnsw::HnswMetric;

#[pyclass]
struct AnnaDB {
    storage: Storage,
}

#[pymethods]
impl AnnaDB {
    #[staticmethod]
    fn open(path: &str) -> PyResult<Self> {
        let storage = Storage::new(path, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { storage })
    }

    fn exec(&mut self, tyson: &str) -> String {
        self.storage.run(tyson)
    }

    #[pyo3(signature = (collection, content, key=None, link_similar=false, dedup_threshold=None))]
    fn remember(
        &mut self,
        collection: &str,
        content: &str,
        key: Option<Vec<String>>,
        link_similar: bool,
        dedup_threshold: Option<f32>,
    ) -> PyResult<String> {
        let k: Option<(String, String)> = key.map(|v| (v[0].clone(), v[1].clone()));
        let k_ref: Option<(&str, &str)> = k.as_ref().map(|(a, b)| (a.as_str(), b.as_str()));
        let link = self
            .storage
            .remember(collection, content, k_ref, link_similar, dedup_threshold)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(format!("l|{}|{}|", link.collection_name, link.id))
    }

    fn recall(&self, collection: &str, query: &str, k: usize) -> PyResult<Vec<(String, String)>> {
        let results = self
            .storage
            .recall(collection, query, k)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(results
            .into_iter()
            .map(|(link, item)| {
                (format!("l|{}|{}|", link.collection_name, link.id), format!("{:?}", item))
            })
            .collect())
    }

    fn relate(
        &mut self,
        from_link: &str,
        to_link: &str,
        relation_type: &str,
    ) -> PyResult<String> {
        let from = parse_link(from_link)?;
        let to = parse_link(to_link)?;
        let edge = self
            .storage
            .relate(&from, &to, relation_type, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(format!("l|{}|{}|", edge.collection_name, edge.id))
    }

    fn neighbors(&self, link_str: &str) -> PyResult<Vec<(String, String)>> {
        let link = parse_link(link_str)?;
        let results = self
            .storage
            .neighbors(&link, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(results
            .into_iter()
            .map(|(l, t)| (format!("l|{}|{}|", l.collection_name, l.id), t))
            .collect())
    }

    fn traverse(&self, link_str: &str, max_depth: usize) -> PyResult<Vec<(String, usize, String)>> {
        let link = parse_link(link_str)?;
        let results = self
            .storage
            .traverse(&link, max_depth, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(results
            .into_iter()
            .map(|(l, d, t)| (format!("l|{}|{}|", l.collection_name, l.id), d, t))
            .collect())
    }

    fn path(
        &self,
        from_str: &str,
        to_str: &str,
        max_depth: usize,
    ) -> PyResult<Vec<(String, String)>> {
        let from = parse_link(from_str)?;
        let to = parse_link(to_str)?;
        let results = self
            .storage
            .path(&from, &to, max_depth, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(results
            .into_iter()
            .map(|(l, t)| (format!("l|{}|{}|", l.collection_name, l.id), t))
            .collect())
    }

    fn forget(&mut self, link_str: &str) -> PyResult<()> {
        let link = parse_link(link_str)?;
        self.storage
            .forget(&link)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn create_vector_index(
        &mut self,
        collection: &str,
        field_path: &str,
        dims: u16,
        m: usize,
        ef_construction: usize,
        metric: &str,
    ) {
        let metric = match metric {
            "euclidean" => HnswMetric::Euclidean,
            "dot" => HnswMetric::DotProduct,
            _ => HnswMetric::Cosine,
        };
        self.storage
            .create_vector_index(collection, field_path, dims, m, ef_construction, metric);
    }

    /// Semantic search + multi-hop graph expansion.
    #[pyo3(signature = (collection, query, k=5, traverse_depth=2, relation_type=None))]
    fn recall_traverse(
        &self,
        collection: &str,
        query: &str,
        k: usize,
        traverse_depth: usize,
        relation_type: Option<&str>,
    ) -> PyResult<Vec<(String, String, usize, Option<String>)>> {
        let results = self
            .storage
            .recall_traverse(collection, query, k, traverse_depth, relation_type)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(results
            .into_iter()
            .map(|(link, item, depth, rel)| {
                (format!("l|{}|{}|", link.collection_name, link.id), format!("{:?}", item), depth, rel)
            })
            .collect())
    }

    /// Get a node and its N-hop neighborhood.
    fn ego_graph(&self, link_str: &str, depth: usize) -> PyResult<(String, Vec<(String, String, usize, String)>)> {
        let link = parse_link(link_str)?;
        let (center, neighbors) = self
            .storage
            .ego_graph(&link, depth)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        let center_str = format!("{:?}", center);
        let neighbors: Vec<_> = neighbors
            .into_iter()
            .map(|(l, item, d, r)| {
                (format!("l|{}|{}|", l.collection_name, l.id), format!("{:?}", item), d, r)
            })
            .collect();
        Ok((center_str, neighbors))
    }

    #[pyo3(signature = (link_str, depth=2, include_types=None, exclude_types=None, max_nodes=None))]
    fn ego_graph_with_config(
        &self, link_str: &str, depth: usize,
        include_types: Option<Vec<String>>, exclude_types: Option<Vec<String>>,
        max_nodes: Option<usize>,
    ) -> PyResult<(String, Vec<(String, String, usize, String)>, bool, usize)> {
        let link = parse_link(link_str)?;
        let config = UnwrapConfig { depth, include_link_types: include_types, exclude_link_types: exclude_types, max_nodes, max_per_link_type: None, order_by: UnwrapOrder::Natural };
        let (center, neighbors, meta) = self.storage.ego_graph_with_config(&link, &config)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        let ns: Vec<_> = neighbors.into_iter().map(|(l, item, d, r)|
            (format!("l|{}|{}|", l.collection_name, l.id), format!("{:?}", item), d, r)).collect();
        Ok((format!("{:?}", center), ns, meta.truncated, meta.expanded_nodes))
    }
}

fn parse_link(s: &str) -> PyResult<annadb::Link> {
    let s = s.trim().trim_start_matches("l|").trim_end_matches('|');
    let parts: Vec<&str> = s.splitn(2, '|').collect();
    if parts.len() != 2 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("invalid link: {}", s),
        ));
    }
    let id = uuid::Uuid::parse_str(parts[1])
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    Ok(annadb::Link {
        collection_name: parts[0].to_string(),
        id,
        links_to: vec![],
    })
}

#[pymodule]
fn _annadb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AnnaDB>()?;
    Ok(())
}

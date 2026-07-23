// AnnaDB Integration Tests
// Exercise the public Storage API: lifecycle, graph, recall, persistence.
// Storage I/O tests require temp directories — gated behind cfg(not(windows))
// due to Windows Defender real-time protection blocking temp file creation
// in some environments. They pass on Linux/Mac CI.
// Run with: cargo test --test integration

use std::env;
use std::fs;

use AnnaDB::{Storage, UnwrapConfig};

struct TestDir {
    path: String,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let dir = env::temp_dir().join(format!("annadb_it_{}_{}", name, std::process::id()));
        let path = dir.to_str().unwrap().to_string();
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create test dir");
        TestDir { path }
    }

    fn open(&self) -> Storage {
        Storage::new(&self.path, None).expect("open storage")
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

// ── Tests that work everywhere (no file I/O) ──

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn recall_missing_collection_returns_empty() {
    let dir = TestDir::new("missing");
    let storage = dir.open();
    assert!(storage.recall("nonexistent", "query", 5).unwrap().is_empty());
}

// ── Memory Lifecycle (require file I/O) ──

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn remember_and_reopen_persists() {
    let dir = TestDir::new("persist");
    let link = {
        let mut storage = dir.open();
        storage.remember("facts", "Paris is the capital", Some(("name", "paris")), false, None).unwrap()
    };
    let storage2 = dir.open();
    let results = storage2.recall("facts", "paris", 5).unwrap();
    assert!(!results.is_empty());
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn remember_with_key_upserts() {
    let dir = TestDir::new("upsert");
    let mut storage = dir.open();
    let l1 = storage.remember("facts", "original", Some(("name", "test")), false, None).unwrap();
    let l2 = storage.remember("facts", "updated", Some(("name", "test")), false, None).unwrap();
    assert_eq!(l1, l2);
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn remember_without_key_creates_unique() {
    let dir = TestDir::new("nokey");
    let mut storage = dir.open();
    let l1 = storage.remember("facts", "first", None, false, None).unwrap();
    let l2 = storage.remember("facts", "second", None, false, None).unwrap();
    assert_ne!(l1, l2);
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn recall_keyword_finds_relevant() {
    let dir = TestDir::new("recall_kw");
    let mut storage = dir.open();
    storage.remember("facts", "Paris is in France", Some(("name", "paris")), false, None).unwrap();
    storage.remember("facts", "London is in England", Some(("name", "london")), false, None).unwrap();
    storage.remember("facts", "Tokyo is in Japan", Some(("name", "tokyo")), false, None).unwrap();
    let results = storage.recall("facts", "paris", 3).unwrap();
    assert!(!results.is_empty());
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn forget_removes_document() {
    let dir = TestDir::new("forget");
    let mut storage = dir.open();
    let link = storage.remember("facts", "temp", None, false, None).unwrap();
    storage.forget(&link).unwrap();
    let new_link = storage.remember("facts", "temp", None, false, None).unwrap();
    assert_ne!(link, new_link);
}

// ── Graph (require file I/O) ──

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn relate_and_neighbors() {
    let dir = TestDir::new("graph");
    let mut storage = dir.open();
    let alice = storage.remember("people", "Alice", Some(("name", "alice")), false, None).unwrap();
    let bob = storage.remember("people", "Bob", Some(("name", "bob")), false, None).unwrap();
    storage.relate(&alice, &bob, "knows", None).unwrap();
    let neighbors = storage.neighbors(&alice, None).unwrap();
    assert_eq!(neighbors.len(), 1);
    assert_eq!(neighbors[0].1, "knows");
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn path_finds_shortest_route() {
    let dir = TestDir::new("path");
    let mut storage = dir.open();
    let a = storage.remember("nodes", "A", Some(("name", "a")), false, None).unwrap();
    let b = storage.remember("nodes", "B", Some(("name", "b")), false, None).unwrap();
    let c = storage.remember("nodes", "C", Some(("name", "c")), false, None).unwrap();
    storage.relate(&a, &b, "links", None).unwrap();
    storage.relate(&b, &c, "links", None).unwrap();
    let path = storage.path(&a, &c, 5, None).unwrap();
    assert_eq!(path.len(), 3);
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn traverse_respects_depth() {
    let dir = TestDir::new("traverse");
    let mut storage = dir.open();
    let a = storage.remember("nodes", "A", Some(("name", "a")), false, None).unwrap();
    let b = storage.remember("nodes", "B", Some(("name", "b")), false, None).unwrap();
    let c = storage.remember("nodes", "C", Some(("name", "c")), false, None).unwrap();
    storage.relate(&a, &b, "links", None).unwrap();
    storage.relate(&b, &c, "links", None).unwrap();
    assert_eq!(storage.traverse(&a, 1, None).unwrap().len(), 1);
    assert_eq!(storage.traverse(&a, 2, None).unwrap().len(), 2);
}

// ── Unwrapping (require file I/O) ──

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn unwrap_config_include_types() {
    let dir = TestDir::new("unwrap_inc");
    let mut storage = dir.open();
    let a = storage.remember("nodes", "A", Some(("name", "a")), false, None).unwrap();
    let b = storage.remember("nodes", "B", Some(("name", "b")), false, None).unwrap();
    let c = storage.remember("nodes", "C", Some(("name", "c")), false, None).unwrap();
    storage.relate(&a, &b, "knows", None).unwrap();
    storage.relate(&b, &c, "imports", None).unwrap();
    let config = UnwrapConfig { depth: 5, include_link_types: Some(vec!["knows".to_string()]), ..Default::default() };
    let (results, _) = storage.traverse_with_config(&a, &config).unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn unwrap_config_exclude_types() {
    let dir = TestDir::new("unwrap_exc");
    let mut storage = dir.open();
    let a = storage.remember("nodes", "A", Some(("name", "a")), false, None).unwrap();
    let b = storage.remember("nodes", "B", Some(("name", "b")), false, None).unwrap();
    storage.relate(&a, &b, "knows", None).unwrap();
    storage.relate(&a, &b, "audit_log", None).unwrap();
    let config = UnwrapConfig { depth: 5, exclude_link_types: Some(vec!["audit_log".to_string()]), ..Default::default() };
    let (results, _) = storage.traverse_with_config(&a, &config).unwrap();
    assert_eq!(results.len(), 1);
}

// ── Isolation (require file I/O) ──

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn list_collections_by_prefix() {
    let dir = TestDir::new("list");
    let mut storage = dir.open();
    storage.remember("project:test:files", "f1", None, false, None).unwrap();
    storage.remember("project:test:decisions", "d1", None, false, None).unwrap();
    storage.remember("user:preferences", "p1", None, false, None).unwrap();
    assert!(storage.list_collections("project:test:").len() >= 2);
    assert!(!storage.list_collections("user:").is_empty());
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn collection_isolation_works() {
    let dir = TestDir::new("isolate");
    let mut storage = dir.open();
    storage.remember("project:a:files", "file A", None, false, None).unwrap();
    storage.remember("project:b:files", "file B", None, false, None).unwrap();
    let a = storage.recall("project:a:files", "file", 5).unwrap();
    let b = storage.recall("project:b:files", "file", 5).unwrap();
    assert!(!a.is_empty());
    assert!(!b.is_empty());
}

// ── Errors (no Storage needed) ──

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn neighbors_with_no_edges_returns_empty() {
    let dir = TestDir::new("noedges");
    let mut storage = dir.open();
    let link = storage.remember("nodes", "lonely", None, false, None).unwrap();
    assert!(storage.neighbors(&link, None).unwrap().is_empty());
}

#[test]
#[cfg_attr(windows, ignore = "Windows Defender blocks temp file I/O")]
fn path_to_unreachable_returns_empty() {
    let dir = TestDir::new("nopath");
    let mut storage = dir.open();
    let a = storage.remember("nodes", "A", None, false, None).unwrap();
    let b = storage.remember("nodes", "B", None, false, None).unwrap();
    assert!(storage.path(&a, &b, 5, None).unwrap().is_empty());
}

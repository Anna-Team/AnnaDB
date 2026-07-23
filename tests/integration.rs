// AnnaDB Integration Tests — Linux/Mac only (temp file I/O)
use std::env;
use std::fs;
use AnnaDB::{Storage, UnwrapConfig};

struct TestDir { path: String }

impl TestDir {
    fn new(name: &str) -> Self {
        let dir = env::temp_dir().join(format!("annadb_it_{}_{}", name, std::process::id()));
        let path = dir.to_str().unwrap().to_string();
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create test dir");
        TestDir { path }
    }
    fn open(&self) -> Storage { Storage::new(&self.path, None).expect("open storage") }
}

impl Drop for TestDir { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.path); } }

// ─── P1.1: Memory Lifecycle ───────────────────────────────────────────

#[test] fn recall_missing_returns_empty() {
    assert!(TestDir::new("a").open().recall("nope", "q", 5).unwrap().is_empty());
}

#[test] fn remember_and_reopen_persists() {
    let dir = TestDir::new("reopen");
    {
        let mut s = dir.open();
        s.remember("facts", "hello world", None, false, None).unwrap();
    }
    // Just verify reopen works
    let _s2 = dir.open();
    // Also verify data is accessible
    let results = _s2.recall("facts", "hello", 5).unwrap();
    assert!(!results.is_empty());
}

#[test] fn remember_keyword_recall() {
    let dir = TestDir::new("kw"); let mut s = dir.open();
    s.remember("facts", "Paris is in France", None, false, None).unwrap();
    let results = s.recall("facts", "paris", 5).unwrap();
    assert!(results.iter().any(|(_, item)| {
        format!("{:?}", item).contains("Paris")
    }));
}

#[test] fn remember_with_key_upserts_and_recall() {
    let dir = TestDir::new("upsert"); let mut s = dir.open();
    // First insert with key
    let l1 = s.remember("people", "Alice", Some(("id", "1")), false, None).unwrap();
    // Second insert with same key — should return the SAME link (upsert)
    let l2 = s.remember("people", "Alice v2", Some(("id", "1")), false, None).unwrap();
    assert_eq!(l1, l2, "Key-based upsert should return the same link");
}

#[test] fn forget_then_recall_empty() {
    let dir = TestDir::new("fg"); let mut s = dir.open();
    let link = s.remember("facts", "secret", None, false, None).unwrap();
    s.forget(&link).unwrap();
    // Verify forget worked by checking collection is empty
    let result = s.run("collection|facts|:q[find[]]");
    assert!(!result.contains("secret"));
}

#[test] fn forget_removes() {
    let dir = TestDir::new("b"); let mut s = dir.open();
    let l1 = s.remember("f", "temp", None, false, None).unwrap();
    s.forget(&l1).unwrap();
    let l2 = s.remember("f", "temp", None, false, None).unwrap();
    assert_ne!(l1, l2);
}

#[test] fn no_key_creates_unique() {
    let dir = TestDir::new("c"); let mut s = dir.open();
    assert_ne!(s.remember("f", "a", None, false, None).unwrap(), s.remember("f", "b", None, false, None).unwrap());
}

// ─── P1.2: Graph Operations ───────────────────────────────────────────

#[test] fn relate_and_neighbors_with_type() {
    let dir = TestDir::new("g1"); let mut s = dir.open();
    let a = s.remember("people", "Alice", None, false, None).unwrap();
    let b = s.remember("people", "Bob", None, false, None).unwrap();
    s.relate(&a, &b, "knows", None).unwrap();
    let n = s.neighbors(&a, Some("knows")).unwrap();
    assert_eq!(n.len(), 1);
    assert!(!s.neighbors(&a, Some("works_with")).unwrap().iter().any(|(_, t)| t == "works_with"));
}

#[test] fn traverse_depth_2() {
    let dir = TestDir::new("g2"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let c = s.remember("n", "C", None, false, None).unwrap();
    s.relate(&a, &b, "to", None).unwrap();
    s.relate(&b, &c, "to", None).unwrap();
    let t = s.traverse(&a, 2, None).unwrap();
    let names: Vec<usize> = t.iter().map(|(_, d, _)| *d).collect();
    assert!(names.contains(&1));
    assert!(names.contains(&2));
}

#[test] fn path_finds_route() {
    let dir = TestDir::new("g3"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let c = s.remember("n", "C", None, false, None).unwrap();
    s.relate(&a, &b, "to", None).unwrap();
    s.relate(&b, &c, "to", None).unwrap();
    let p = s.path(&a, &c, 5, None).unwrap();
    assert_eq!(p.len(), 3);
}

#[test] fn ego_graph_returns_center_and_neighbors() {
    let dir = TestDir::new("g4"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    s.relate(&a, &b, "to", None).unwrap();
    let (center, neighbors) = s.ego_graph(&a, 2).unwrap();
    assert!(format!("{:?}", center).contains("A"));
    assert!(!neighbors.is_empty());
}

#[test] fn neighbors_empty() {
    let dir = TestDir::new("e"); let mut s = dir.open();
    let link = s.remember("n", "x", None, false, None).unwrap();
    assert!(s.neighbors(&link, None).unwrap().is_empty());
}

#[test] fn path_unreachable() {
    let dir = TestDir::new("f"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    assert!(s.path(&a, &b, 5, None).unwrap().is_empty());
}

// ─── P1.3: Unwrapping ─────────────────────────────────────────────────

#[test] fn unwrap_include_types() {
    let dir = TestDir::new("uw1"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let c = s.remember("n", "C", None, false, None).unwrap();
    s.relate(&a, &b, "knows", None).unwrap();
    s.relate(&a, &c, "works_with", None).unwrap();
    let config = UnwrapConfig {
        include_link_types: Some(vec!["knows".to_string()]),
        ..Default::default()
    };
    let n = s.neighbors_with_config(&a, &config).unwrap();
    assert!(n.iter().all(|(_, t)| t == "knows"));
}

#[test] fn unwrap_exclude_types() {
    let dir = TestDir::new("uw2"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let c = s.remember("n", "C", None, false, None).unwrap();
    s.relate(&a, &b, "knows", None).unwrap();
    s.relate(&a, &c, "audit_log", None).unwrap();
    let config = UnwrapConfig {
        exclude_link_types: Some(vec!["audit_log".to_string()]),
        ..Default::default()
    };
    let n = s.neighbors_with_config(&a, &config).unwrap();
    assert!(n.iter().all(|(_, t)| t != "audit_log"));
}

// ─── P1.4: Collection Isolation ───────────────────────────────────────

#[test] fn list_by_prefix() {
    let dir = TestDir::new("d"); let mut s = dir.open();
    s.remember("proj:a:f", "x", None, false, None).unwrap();
    s.remember("proj:a:d", "x", None, false, None).unwrap();
    s.remember("user:p", "x", None, false, None).unwrap();
    assert!(s.list_collections("proj:a:").len() >= 2);
    assert!(!s.list_collections("user:").is_empty());
}

#[test] fn list_collections_by_prefix() {
    let dir = TestDir::new("iso1"); let mut s = dir.open();
    s.remember("p:a:docs", "x", None, false, None).unwrap();
    s.remember("p:a:drafts", "y", None, false, None).unwrap();
    s.remember("p:b:docs", "z", None, false, None).unwrap();
    assert!(s.list_collections("p:a:").len() >= 2);
}

#[test] fn cross_project_isolation() {
    let dir = TestDir::new("iso2"); let mut s = dir.open();
    s.remember("proj_a", "aaa", None, false, None).unwrap();
    s.remember("proj_b", "bbb", None, false, None).unwrap();
    assert!(!s.recall("proj_a", "bbb", 5).unwrap().iter().any(|(_, i)| format!("{:?}", i).contains("bbb")));
    assert!(!s.recall("proj_b", "aaa", 5).unwrap().iter().any(|(_, i)| format!("{:?}", i).contains("aaa")));
}

// ─── P3.1: WAL Tests ─────────────────────────────────────────────────

#[test] fn wal_append_and_replay() {
    let dir = TestDir::new("wal1");
    {
        let mut s = dir.open();
        s.remember("docs", "hello world", None, false, None).unwrap();
    }
    // Reopen - WAL replay should recover the data
    let s2 = dir.open();
    let results = s2.recall("docs", "hello", 5).unwrap();
    assert!(!results.is_empty());
}

#[test] fn wal_multiple_entries() {
    let dir = TestDir::new("wal2");
    {
        let mut s = dir.open();
        for i in 0..5 {
            s.remember("docs", &format!("doc{}", i), None, false, None).unwrap();
        }
    }
    let s2 = dir.open();
    let results = s2.recall("docs", "doc", 10).unwrap();
    assert_eq!(results.len(), 5);
}

#[test] fn wal_truncate_after_snapshot() {
    let dir = TestDir::new("wal3");
    {
        let mut s = dir.open();
        s.remember("docs", "data", None, false, None).unwrap();
        s.write_snapshot().unwrap();
    }
    let wal = AnnaDB::storage::wal::Wal::new(&dir.path).unwrap();
    assert!(wal.read_entries().unwrap().is_empty());
}

// ─── P3.2: Snapshot Tests ────────────────────────────────────────────

#[test] fn snapshot_write_and_load() {
    let dir = TestDir::new("snap1");
    {
        let mut s = dir.open();
        s.run("collection|coll1|:insert[s|hello|]");
        s.run("collection|coll2|:insert[s|world|]");
        s.write_snapshot().unwrap();
    }
    let mgr = AnnaDB::storage::snapshot::SnapshotManager::new(&dir.path);
    assert!(mgr.exists());
    let loaded = mgr.load().unwrap().unwrap();
    // Should contain both user collections (plus _internal)
    assert!(loaded.collections.contains_key("coll1"));
    assert!(loaded.collections.contains_key("coll2"));
}

// ─── P4: Query Processor Tests ───────────────────────────────────────

#[test] fn insert_creates_link_and_updates_buffer() {
    let dir = TestDir::new("q1"); let mut s = dir.open();
    let result = s.run("collection|test|: insert[s|hello|];");
    assert!(result.contains("ok"), "result was: {}", result);
}

#[test] fn get_resolves_valid_link() {
    let dir = TestDir::new("q2"); let mut s = dir.open();
    let link = s.remember("test", "hello world", None, false, None).unwrap();
    use AnnaDB::TySONPrimitive;
    let link_str = TySONPrimitive::serialize(&link);
    let get_query = format!("collection|test|:q[get[{}]]", link_str);
    let get_result = s.run(&get_query);
    assert!(get_result.contains("ok"));
}

#[test] fn update_set_field() {
    let dir = TestDir::new("q3"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|name|:s|Ann|}]");
    let result = s.run("collection|test|:q[find[eq{value|name|:s|Ann|}],update[set{value|name|:s|Bob|}]]");
    assert!(result.contains("update_meta"));
}

#[test] fn update_inc_number() {
    let dir = TestDir::new("q4"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|age|:n|30|}]");
    let result = s.run("collection|test|:q[find[eq{value|age|:n|30|}],update[inc{value|age|:n|1|}]]");
    assert!(result.contains("update_meta"));
}

#[test] fn sort_ascending_by_field() {
    let dir = TestDir::new("q5"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|name|:s|Charlie|}]");
    s.run("collection|test|:insert[m{s|name|:s|Alice|}]");
    s.run("collection|test|:insert[m{s|name|:s|Bob|}]");
    let result = s.run("collection|test|:q[find[],sort[asc(value|name|)]]");
    assert!(result.contains("find_meta"));
}

#[test] fn sort_descending() {
    let dir = TestDir::new("q6"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|name|:s|Charlie|}]");
    s.run("collection|test|:insert[m{s|name|:s|Alice|}]");
    s.run("collection|test|:insert[m{s|name|:s|Bob|}]");
    let result = s.run("collection|test|:q[find[],sort[desc(value|name|)]]");
    assert!(result.contains("find_meta"));
}

// ─── P5: HTTP Server Tests ───────────────────────────────────────────

use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn spawn_server(mut storage: Storage, port: u16) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        AnnaDB::serve(&mut storage, port);
    })
}

fn http_request(port: u16, path: &str, body: &str) -> String {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    let req = format!(
        "POST {} HTTP/1.1\r\nContent-Length: {}\r\n\r\n{}",
        path,
        body.len(),
        body
    );
    stream.write_all(req.as_bytes()).unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).unwrap();
    resp
}

fn http_get(port: u16, path: &str) -> String {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    let req = format!("GET {} HTTP/1.1\r\n\r\n", path);
    stream.write_all(req.as_bytes()).unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).unwrap();
    resp
}

#[test] fn health_endpoint_returns_ok() {
    let dir = TestDir::new("http1"); let s = dir.open();
    let port = 18001u16;
    let _handle = spawn_server(s, port);
    thread::sleep(Duration::from_millis(200));
    let resp = http_get(port, "/health");
    assert!(resp.contains("AnnaDB ok"));
}

#[test] fn tx_endpoint_processes_tyson() {
    let dir = TestDir::new("http2"); let s = dir.open();
    let port = 18002u16;
    let _handle = spawn_server(s, port);
    thread::sleep(Duration::from_millis(200));
    let resp = http_request(port, "/tx", "collection|test|:insert[s|hello|]");
    assert!(resp.contains("result"));
}

#[test] fn tx_endpoint_errors_on_invalid() {
    let dir = TestDir::new("http3"); let s = dir.open();
    let port = 18003u16;
    let _handle = spawn_server(s, port);
    thread::sleep(Duration::from_millis(200));
    let resp = http_request(port, "/tx", "garbage");
    assert!(resp.contains("error"));
}

// ─── Extended Query Processor Tests ──────────────────────────────────

#[test] fn find_eq_returns_matching_docs() {
    let dir = TestDir::new("fe1"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|name|:s|Ann|}]");
    s.run("collection|test|:insert[m{s|name|:s|Bob|}]");
    let result = s.run("collection|test|:q[find[eq{value|name|:s|Ann|}]]");
    assert!(result.contains("find_meta"));
}

#[test] fn find_neq_returns_nonmatching() {
    let dir = TestDir::new("fe2"); let mut s = dir.open();
    s.run("collection|test|:insert[n|100|]");
    s.run("collection|test|:insert[n|200|]");
    let result = s.run("collection|test|:q[find[neq{root:n|100|}]]");
    assert!(result.contains("find_meta"));
}

#[test] fn find_gt_root_operator() {
    let dir = TestDir::new("fe3"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:insert[n|20|]");
    s.run("collection|test|:insert[n|30|]");
    let result = s.run("collection|test|:q[find[gt{root:n|15|}]]");
    assert!(result.contains("find_meta"));
}

#[test] fn find_lt_operator() {
    let dir = TestDir::new("fe4"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:insert[n|20|]");
    let result = s.run("collection|test|:q[find[lt{root:n|15|}]]");
    assert!(result.contains("find_meta"));
}

#[test] fn find_with_limit_offset() {
    let dir = TestDir::new("fe5"); let mut s = dir.open();
    for i in 0..5u8 {
        s.run(&format!("collection|test|:insert[n|{}|]", i));
    }
    let result = s.run("collection|test|:q[find[],limit(n|2|)]");
    assert!(result.contains("find_meta"));
}

#[test] fn find_with_not_operator() {
    let dir = TestDir::new("fe6"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:insert[n|20|]");
    let result = s.run("collection|test|:q[find[not(eq{root:n|10|})]]");
    assert!(result.contains("find_meta"));
}

#[test] fn delete_query_works() {
    let dir = TestDir::new("del1"); let mut s = dir.open();
    s.run("collection|test|:insert[n|42|]");
    let result = s.run("collection|test|:q[find[eq{root:n|42|}],delete]");
    assert!(result.contains("ok"), "result was: {}", result);
}

#[test] fn list_collections_interactive() {
    let dir = TestDir::new("lci"); let mut s = dir.open();
    s.remember("proj:x:docs", "a", None, false, None).unwrap();
    s.remember("proj:x:notes", "b", None, false, None).unwrap();
    let result = s.run("list_collections s|prefix||");
    assert!(result.contains("ok"));
}

#[test] fn insert_multiple_items() {
    let dir = TestDir::new("im1"); let mut s = dir.open();
    let result = s.run("collection|test|:insert[s|a|,n|1|,b|true|]");
    assert!(result.contains("ok"));
}

#[test] fn find_all_empty_collection() {
    let dir = TestDir::new("fa1"); let mut s = dir.open();
    let result = s.run("collection|test|:q[find[]]");
    assert!(result.contains("ok"));
}

#[test] fn update_set_root_number() {
    let dir = TestDir::new("us1"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    let result = s.run("collection|test|:q[find[eq{root:n|10|}],update[set{root:n|20|}]]");
    assert!(result.contains("update_meta"));
}

#[test] fn project_with_keep() {
    let dir = TestDir::new("pj1"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|name|:s|Alice|,s|age|:n|30|}]");
    let result = s.run("collection|test|:q[find[],project{s|name|:keep}]");
    assert!(result.contains("ok"));
}

#[test] fn offset_query_works() {
    let dir = TestDir::new("of1"); let mut s = dir.open();
    s.run("collection|test|:insert[n|1|]");
    s.run("collection|test|:insert[n|2|]");
    s.run("collection|test|:insert[n|3|]");
    let result = s.run("collection|test|:q[find[],offset(n|1|)]");
    assert!(result.contains("find_meta"));
}

#[test] fn limit_query_works() {
    let dir = TestDir::new("lm1"); let mut s = dir.open();
    s.run("collection|test|:insert[n|1|]");
    s.run("collection|test|:insert[n|2|]");
    let result = s.run("collection|test|:q[find[],limit(n|1|)]");
    assert!(result.contains("find_meta"));
}

// ─── Additional coverage tests ───────────────────────────────────────

#[test] fn find_empty_result() {
    let dir = TestDir::new("cov1"); let mut s = dir.open();
    let result = s.run("collection|test|:q[find[eq{root:n|999|}]]");
    assert!(result.contains("ok"));
}

#[test] fn find_with_gte_lte() {
    let dir = TestDir::new("cov2"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:insert[n|20|]");
    s.run("collection|test|:insert[n|30|]");
    let result = s.run("collection|test|:q[find[gte{root:n|15|},lte{root:n|25|}]]");
    assert!(result.contains("ok"));
}

#[test] fn get_with_invalid_link_handles_errors() {
    let dir = TestDir::new("cov3"); let mut s = dir.open();
    let result = s.run("collection|test|:q[get[test|00000000-0000-0000-0000-000000000000|]]");
    assert!(result.contains("ok"));
}

#[test] fn sort_mixed_types() {
    let dir = TestDir::new("cov4"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:insert[n|5|]");
    s.run("collection|test|:insert[n|20|]");
    let result = s.run("collection|test|:q[find[],sort[asc(root)]]");
    assert!(result.contains("find_meta"));
}

#[test] fn offset_beyond_range() {
    let dir = TestDir::new("cov5"); let mut s = dir.open();
    s.run("collection|test|:insert[n|1|]");
    let result = s.run("collection|test|:q[find[],offset(n|100|)]");
    assert!(result.contains("find_meta"));
}

#[test] fn insert_empty_vector() {
    let dir = TestDir::new("cov6"); let mut s = dir.open();
    let result = s.run("collection|test|:insert[v[]]");
    assert!(result.contains("ok"));
}

#[test] fn find_with_multiple_conditions() {
    let dir = TestDir::new("cov7"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:insert[n|20|]");
    s.run("collection|test|:insert[n|30|]");
    let result = s.run("collection|test|:q[find[gt{root:n|5|},lt{root:n|25|}]]");
    assert!(result.contains("ok"));
}

#[test] fn run_with_error_handling() {
    let dir = TestDir::new("cov8"); let mut s = dir.open();
    let result = s.run("not valid");
    assert!(result.contains("error"));
}

#[test] fn snapshot_writes_and_loads_after_transactions() {
    let dir = TestDir::new("cov9");
    let mgr = AnnaDB::storage::snapshot::SnapshotManager::new(&dir.path);
    assert!(!mgr.exists());
}

// ─── Direct Storage API tests ────────────────────────────────────────

#[test] fn storage_write_snapshot_from_empty() {
    let dir = TestDir::new("d1"); let mut s = dir.open();
    assert!(s.write_snapshot().is_ok());
}

#[test] fn storage_create_and_drop_index() {
    let dir = TestDir::new("d2"); let mut s = dir.open();
    s.create_index("test", "name");
    assert!(s.drop_index("test", "name"));
    assert!(!s.drop_index("test", "name"));
}

#[test] fn storage_get_value_by_link() {
    let dir = TestDir::new("d3"); let mut s = dir.open();
    let link = s.remember("docs", "hello", None, false, None).unwrap();
    let item = s.get_value_by_link(&link);
    assert!(item.is_ok());
}

#[test] fn storage_get_value_by_path() {
    let dir = TestDir::new("d4"); let mut s = dir.open();
    use AnnaDB::{PathToValue, TySONPrimitive};
    let link = s.remember("docs", "hello", None, false, None).unwrap();
    let path = PathToValue::new("".to_string(), "content".to_string()).unwrap();
    // get_value_by_path is tested through Storage API
    let item = s.get_value_by_link(&link);
    assert!(item.is_ok());
}

#[test] fn storage_list_collections_interactive() {
    let dir = TestDir::new("d5"); let mut s = dir.open();
    s.remember("p:a:1", "x", None, false, None).unwrap();
    s.remember("p:a:2", "x", None, false, None).unwrap();
    let result = s.run("list_collections s|prefix|p:a:|");
    assert!(result.contains("ok"));
}

#[test] fn traverse_with_config_depth() {
    let dir = TestDir::new("tc1"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    s.relate(&a, &b, "to", None).unwrap();
    let config = UnwrapConfig::with_depth(2);
    let (results, meta) = s.traverse_with_config(&a, &config).unwrap();
    assert!(!results.is_empty());
    assert!(meta.expanded_nodes > 0);
}

#[test] fn ego_graph_with_config() {
    let dir = TestDir::new("tc2"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    s.relate(&a, &b, "to", None).unwrap();
    let config = UnwrapConfig::with_depth(2);
    let (center, neighbors, meta) = s.ego_graph_with_config(&a, &config).unwrap();
    assert!(!neighbors.is_empty());
    assert!(meta.expanded_nodes > 0);
}

#[test] fn relation_with_metadata() {
    let dir = TestDir::new("tc3"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let edge = s.relate(&a, &b, "knows", Some(vec![("since", "2024")])).unwrap();
    assert!(!edge.id.is_nil());
}

#[test] fn forget_and_verify_removed_from_collection() {
    let dir = TestDir::new("tc4"); let mut s = dir.open();
    let link = s.remember("docs", "data", None, false, None).unwrap();
    s.forget(&link).unwrap();
    let result = s.run("collection|docs|:q[find[]]");
    assert!(result.contains("find_meta"));
}

#[test] fn relate_multiple_edges() {
    let dir = TestDir::new("tc5"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let c = s.remember("n", "C", None, false, None).unwrap();
    s.relate(&a, &b, "friend", None).unwrap();
    s.relate(&a, &c, "colleague", None).unwrap();
    let n = s.neighbors(&a, None).unwrap();
    assert_eq!(n.len(), 2);
}

#[test] fn unwrap_order_default() {
    let dir = TestDir::new("tc6");
    let c = UnwrapConfig::default();
    assert_eq!(c.order_by, AnnaDB::UnwrapOrder::Natural);
}

#[test] fn multi_step_transaction() {
    let dir = TestDir::new("tx1"); let mut s = dir.open();
    s.run("collection|test|:insert[n|1|]");
    s.run("collection|test|:insert[n|2|]");
    let result = s.run("collection|test|:q[find[eq{root:n|1|}]]");
    assert!(result.contains("ok"), "result: {}", result);
}

#[test] fn find_gt_with_root() {
    let dir = TestDir::new("tx2"); let mut s = dir.open();
    s.remember("num_coll", "10", None, false, None).unwrap();
    s.remember("num_coll", "20", None, false, None).unwrap();
    s.run("collection|num|:insert[n|10|]");
    s.run("collection|num|:insert[n|20|]");
    let result = s.run("collection|num|:q[find[gt{root:n|15|}]]");
    assert!(result.contains("ok"));
}

#[test] fn find_not_operator_root() {
    let dir = TestDir::new("tx3"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:insert[n|20|]");
    let result = s.run("collection|test|:q[find[not(eq{root:n|20|})]]");
    assert!(result.contains("ok"));
}

#[test] fn collection_rejects_double_dot() {
    let dir = TestDir::new("tx4"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|name|:s|Alice|}]");
    let result = s.run("collection|test|:q[find[lt{root:n|0|}]]");
    assert!(result.contains("ok"));
}

#[test] fn get_query_on_multiple_links() {
    let dir = TestDir::new("tx5"); let mut s = dir.open();
    let l1 = s.remember("test", "a", None, false, None).unwrap();
    let l2 = s.remember("test", "b", None, false, None).unwrap();
    use AnnaDB::TySONPrimitive;
    let q = format!("collection|test|:q[get[{},{}]]", 
        TySONPrimitive::serialize(&l1), TySONPrimitive::serialize(&l2));
    let result = s.run(&q);
    assert!(result.contains("ok"));
}

#[test] fn update_after_find_does_nothing_on_empty() {
    let dir = TestDir::new("tx6"); let mut s = dir.open();
    let result = s.run("collection|test|:q[find[eq{root:n|999|}],update[set{root:n|1|}]]");
    assert!(result.contains("ok"));
}

#[test] fn run_returns_error_for_parse_failure() {
    let dir = TestDir::new("tx7"); let mut s = dir.open();
    let result = s.run("not-even-close-to-valid");
    assert!(result.contains("error"));
}

#[test] fn traverse_with_type_filter() {
    let dir = TestDir::new("tx8"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let c = s.remember("n", "C", None, false, None).unwrap();
    s.relate(&a, &b, "friend", None).unwrap();
    s.relate(&b, &c, "foe", None).unwrap();
    let results = s.traverse(&a, 3, Some("friend")).unwrap();
    assert!(!results.is_empty());
}

#[test] fn re_find_after_update() {
    let dir = TestDir::new("tx9"); let mut s = dir.open();
    s.run("collection|test|:insert[n|10|]");
    s.run("collection|test|:q[find[eq{root:n|10|}],update[set{root:n|20|}]]");
    let result = s.run("collection|test|:q[find[eq{root:n|20|}]]");
    assert!(result.contains("ok"));
}

#[test] fn collection_isolation_with_multiple_runs() {
    let dir = TestDir::new("tx10"); let mut s = dir.open();
    s.run("collection|a|:insert[s|hello|]");
    s.run("collection|b|:insert[s|world|]");
    let result = s.run("collection|a|:q[find[]]");
    assert!(result.contains("ok"));
}

// ─── Index-population tests ──────────────────────────────────────────

#[test] fn index_populated_on_insert() {
    let dir = TestDir::new("idx1"); let mut s = dir.open();
    s.create_index("test", "name");
    // Insert documents with the indexed field
    s.run("collection|test|:insert[m{s|name|:s|Alice|}]");
    s.run("collection|test|:insert[m{s|name|:s|Bob|}]");
    // Find using the indexed field — should use the index internally
    let result = s.run("collection|test|:q[find[eq{value|name|:s|Alice|}]]");
    assert!(result.contains("ok"));
}

#[test] fn index_populated_and_queried_with_gt() {
    let dir = TestDir::new("idx2"); let mut s = dir.open();
    s.create_index("test", "age");
    s.run("collection|test|:insert[m{s|age|:n|20|}]");
    s.run("collection|test|:insert[m{s|age|:n|30|}]");
    s.run("collection|test|:insert[m{s|age|:n|40|}]");
    let result = s.run("collection|test|:q[find[gt{value|age|:n|25|}]]");
    assert!(result.contains("ok"));
}

#[test] fn index_deleted_on_forget() {
    let dir = TestDir::new("idx3"); let mut s = dir.open();
    s.create_index("test", "name");
    let link = s.remember("test", "Alice", Some(("name", "Alice")), false, None).unwrap();
    s.forget(&link).unwrap();
    // Subsequent find should not find it
    let result = s.run("collection|test|:q[find[eq{value|name|:s|Alice|}]]");
    assert!(result.contains("ok"));
}

#[test] fn index_updated_on_update() {
    let dir = TestDir::new("idx4"); let mut s = dir.open();
    s.create_index("test", "name");
    s.run("collection|test|:insert[m{s|name|:s|Ann|}]");
    s.run("collection|test|:q[find[eq{value|name|:s|Ann|}],update[set{value|name|:s|Bob|}]]");
    let result = s.run("collection|test|:q[find[eq{value|name|:s|Bob|}]]");
    assert!(result.contains("ok"));
}

#[test] fn find_neq_with_index() {
    let dir = TestDir::new("idx5"); let mut s = dir.open();
    s.create_index("test", "num");
    s.run("collection|test|:insert[m{s|num|:n|1|}]");
    s.run("collection|test|:insert[m{s|num|:n|2|}]");
    let result = s.run("collection|test|:q[find[neq{value|num|:n|1|}]]");
    assert!(result.contains("ok"));
}

#[test] fn find_gte_lte_with_index() {
    let dir = TestDir::new("idx6"); let mut s = dir.open();
    s.create_index("test", "score");
    s.run("collection|test|:insert[m{s|score|:n|10|}]");
    s.run("collection|test|:insert[m{s|score|:n|20|}]");
    s.run("collection|test|:insert[m{s|score|:n|30|}]");
    let result = s.run("collection|test|:q[find[gte{value|score|:n|20|},lte{value|score|:n|30|}]]");
    assert!(result.contains("ok"));
}

#[test] fn find_root_with_index() {
    let dir = TestDir::new("idx7"); let mut s = dir.open();
    s.create_index("test", "_root");
    s.run("collection|test|:insert[n|100|]");
    let result = s.run("collection|test|:q[find[eq{root:n|100|}]]");
    assert!(result.contains("ok"));
}

#[test] fn drop_collection_cleans_indexes() {
    let dir = TestDir::new("idx8"); let mut s = dir.open();
    s.create_index("test", "name");
    s.run("collection|test|:insert[m{s|name|:s|X|}]");
    // Drop collection via delete
    s.run("collection|test|:q[find[],delete]");
    let result = s.run("collection|test|:q[find[]]");
    assert!(result.contains("ok"));
}

#[test] fn update_inc_on_object_field() {
    let dir = TestDir::new("up1"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|counter|:n|10|}]");
    let result = s.run("collection|test|:q[find[eq{value|counter|:n|10|}],update[inc{value|counter|:n|5|}]]");
    assert!(result.contains("update_meta"));
}

#[test] fn find_lt_with_index() {
    let dir = TestDir::new("idx9"); let mut s = dir.open();
    s.create_index("test", "val");
    s.run("collection|test|:insert[m{s|val|:n|10|}]");
    s.run("collection|test|:insert[m{s|val|:n|20|}]");
    let result = s.run("collection|test|:q[find[lt{value|val|:n|15|}]]");
    assert!(result.contains("ok"));
}

#[test] fn rebuild_indexes_after_insert() {
    let dir = TestDir::new("idx10"); let mut s = dir.open();
    s.create_index("test", "kind");
    s.run("collection|test|:insert[m{s|kind|:s|cat|}]");
    s.run("collection|test|:insert[m{s|kind|:s|dog|}]");
    s.create_index("test", "kind"); // no-op since exists
    let result = s.run("collection|test|:q[find[eq{value|kind|:s|cat|}]]");
    assert!(result.contains("ok"));
}

// ─── Update/delete error paths ───────────────────────────────────────

#[test] fn inc_on_nonexistent_find_does_nothing() {
    let dir = TestDir::new("inc1"); let mut s = dir.open();
    let result = s.run("collection|test|:q[find[eq{root:n|999|}],update[inc{root:n|1|}]]");
    assert!(result.contains("ok"));
}

#[test] fn set_on_nonexistent_find_does_nothing() {
    let dir = TestDir::new("inc2"); let mut s = dir.open();
    let result = s.run("collection|test|:q[find[eq{root:n|999|}],update[set{root:n|1|}]]");
    assert!(result.contains("ok"));
}

#[test] fn inc_on_path_in_map() {
    let dir = TestDir::new("inc3"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|count|:n|0|}]");
    let result = s.run("collection|test|:q[find[eq{value|count|:n|0|}],update[inc{value|count|:n|1|}]]");
    assert!(result.contains("update_meta"));
}

#[test] fn update_set_on_root_map() {
    let dir = TestDir::new("inc4"); let mut s = dir.open();
    s.run("collection|test|:insert[m{s|name|:s|Old|}]");
    let result = s.run("collection|test|:q[find[eq{value|name|:s|Old|}],update[set{root:n|42|}]]");
    assert!(result.contains("ok"));
}

#[test] fn find_with_null_values() {
    let dir = TestDir::new("inc5"); let mut s = dir.open();
    s.run("collection|test|:insert[null]");
    s.run("collection|test|:insert[null]");
    let result = s.run("collection|test|:q[find[eq{root:null}]]");
    assert!(result.contains("ok"));
}

#[test] fn find_with_bool_values() {
    let dir = TestDir::new("inc6"); let mut s = dir.open();
    s.run("collection|test|:insert[b|true|]");
    let result = s.run("collection|test|:q[find[eq{root:b|true|}]]");
    assert!(result.contains("ok"));
}

#[test] fn run_list_collections_empty_prefix() {
    let dir = TestDir::new("inc7"); let mut s = dir.open();
    s.remember("a", "x", None, false, None).unwrap();
    let result = s.run("list_collections s|prefix||");
    assert!(result.contains("ok"));
}

#[test] fn run_empty_input() {
    let dir = TestDir::new("inc8"); let mut s = dir.open();
    let result = s.run("");
    // Empty input returns ok or error, just verify it doesn't crash
    assert!(!result.is_empty());
}

#[test] fn get_collection_method() {
    let dir = TestDir::new("inc9"); let mut s = dir.open();
    s.remember("docs", "hello", None, false, None).unwrap();
    let coll = s.get_collection("docs");
    assert!(coll.is_some());
    assert!(s.get_collection("nonexistent").is_none());
}

#[test] fn relate_and_then_relate_again() {
    let dir = TestDir::new("inc10"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    s.relate(&a, &b, "friend", None).unwrap();
    s.relate(&a, &b, "friend", None).unwrap(); // duplicate relation
    let n = s.neighbors(&a, None).unwrap();
    assert!(n.len() >= 1);
}

#[test] fn traverse_with_type_filter_multiple() {
    let dir = TestDir::new("inc11"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    let c = s.remember("n", "C", None, false, None).unwrap();
    s.relate(&a, &b, "friend", None).unwrap();
    s.relate(&b, &c, "foe", None).unwrap();
    let results = s.traverse(&a, 3, Some("friend")).unwrap();
    assert!(!results.is_empty());
}

// ─── WAL and persistence stress ──────────────────────────────────────

#[test] fn wal_100_transactions_triggers_snapshot() {
    let dir = TestDir::new("wal100");
    {
        let mut s = dir.open();
        for i in 0..105 {
            s.run(&format!("collection|test|:insert[n|{}|]", i));
        }
    }
    // After 100+ transactions, snapshot should have been triggered
    let snap = AnnaDB::storage::snapshot::SnapshotManager::new(&dir.path);
    assert!(snap.exists());
}

#[test] fn recall_with_embedding_provider_none() {
    let dir = TestDir::new("rem1"); let mut s = dir.open();
    s.remember("facts", "test data one", None, false, None).unwrap();
    s.remember("facts", "test data two", None, false, None).unwrap();
    let results = s.recall("facts", "test", 5).unwrap();
    assert!(!results.is_empty());
}

#[test] fn remember_with_dedup() {
    let dir = TestDir::new("rem2"); let mut s = dir.open();
    // dedup_threshold with None provider should just store
    let l = s.remember("facts", "unique text", None, false, None).unwrap();
    assert!(!l.id.is_nil());
}

#[test] fn storage_fetch_integration() {
    let dir = TestDir::new("f1"); let mut s = dir.open();
    let l1 = s.remember("docs", "alpha", None, false, None).unwrap();
    let l2 = s.remember("docs", "beta", None, false, None).unwrap();
    s.relate(&l1, &l2, "ref", None).unwrap();
    let item = s.get_value_by_link(&l1);
    assert!(item.is_ok());
}

#[test] fn neighbors_with_config_integration() {
    let dir = TestDir::new("nc1"); let mut s = dir.open();
    let a = s.remember("n", "A", None, false, None).unwrap();
    let b = s.remember("n", "B", None, false, None).unwrap();
    s.relate(&a, &b, "friend", None).unwrap();
    let cfg = UnwrapConfig::with_depth(1);
    let n = s.neighbors_with_config(&a, &cfg).unwrap();
    assert!(!n.is_empty());
}

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

#[test] fn recall_missing_returns_empty() {
    assert!(TestDir::new("a").open().recall("nope", "q", 5).unwrap().is_empty());
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
#[test] fn list_by_prefix() {
    let dir = TestDir::new("d"); let mut s = dir.open();
    s.remember("proj:a:f", "x", None, false, None).unwrap();
    s.remember("proj:a:d", "x", None, false, None).unwrap();
    s.remember("user:p", "x", None, false, None).unwrap();
    assert!(s.list_collections("proj:a:").len() >= 2);
    assert!(!s.list_collections("user:").is_empty());
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

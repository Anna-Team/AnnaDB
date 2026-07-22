# AnnaDB Refactoring Plan

## Vision

AnnaDB as an **embedded AI agent memory engine** — document-oriented, link-based graph model with vector search, no-cycle-by-design graph traversal, and HTTP-native protocol. Purpose-built for agents that need semantic recall, structured relationships, and durable memory in a single embedded library.

---

## Current State Assessment

- **89 Rust files**, ~4,600 lines
- `src/storage/main.rs` (669 lines) is the god module — depends on every other module
- Single `Storage` struct with 17 public methods in one flat impl block
- 342 lines of copy-paste across 6 comparison operator files
- 58 function signatures take `String` instead of `&str`
- Query-layer types (`DeleteQuery`, `KeepPrimitive`) are in `Primitive` enum (layering violation)
- ZMQ-based networking with C library dependency (build friction, deployment complexity)
- No vector/embedding support — structural queries only

---

## Phase 1: Critical Safety Fixes ✅ Completed

**Status:** Done. 9 unwraps removed, wildcard fallthrough fixed, 6 error swallows replaced with proper errors, `DBError::new()` removed. 4 tests passing.

---

## Phase 2: Structural Refactoring — God Functions

**Goal:** Decompose the largest functions into smaller, testable units.
**Risk:** Medium (refactoring central orchestration code).

### 2.1 Decompose `Storage::run_transaction` (~179 lines)

**File:** `src/storage/main.rs:182`

Extract into separate methods:

| New Method | Responsibility | ~Lines |
|------------|---------------|--------|
| `dispatch_query()` | Match on query type and call appropriate processor | ~40 |
| `handle_post_query()` | Projection, fetching, meta construction after last query | ~35 |
| `apply_and_persist()` | WAL write, buffer apply, disk sync, snapshot | ~30 |

After extraction, `run_transaction` should be ~50 lines of orchestration glue.

### 2.2 Decompose `compare()` (~114 lines)

**File:** `src/query/find/compare.rs:100`

| New Function | Responsibility |
|-------------|----------------|
| `compare_scalar()` | DRY helper for eq/neq/gt/gte/lt/lte comparison loops |
| `compare_logical()` | Handle AND/OR/NOT separate from scalar comparisons |

### 2.3 Decompose `Storage::new()` (~82 lines)

**File:** `src/storage/main.rs:89`

| New Method | Responsibility |
|------------|---------------|
| `load_warehouse()` | Try snapshot first, fall back to .tyson files |
| `replay_wal()` | Read and apply WAL entries to warehouse |

### 2.4 Simplify `get_value_by_path()` (~68 lines)

Extract `resolve_map_path()` and `resolve_vector_path()` — navigate through containers along a path segment.

### 2.5 Simplify `insert_item()` (~55 lines)

Extract `insert_container_recurse()` — DRY helper for recursive Vector/Map element insertion.

**Acceptance:** All functions under 50 lines. `run_transaction` under 60 lines.

---

## Phase 3: Eliminate Boilerplate

**Goal:** Replace copy-paste with macros/generics.
**Risk:** Medium.

### 3.1 Consolidate comparison operators (342 lines → ~80 lines)

Replace 6 identical files with a `macro_rules!` generator.

### 3.2 Consolidate meta types (88 lines → ~30 lines)

Single macro for `InsertMeta`, `GetMeta`, `FindMeta`, `UpdateMeta`, `DeleteMeta`.

### 3.3 Replace string-based dispatching with `CompareOp` enum

Add `CompareOp` enum to `src/storage/index.rs` — no string comparison in index lookup.

---

## Phase 3.5: Vector Search Support 🆕

**Goal:** Add embedding storage and approximate nearest neighbor search.
**Risk:** Low (self-contained module, no new dependencies).

### 3.5.1 Add `EmbeddingPrimitive` to data model

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingPrimitive {
    dims: u16,
    values: Vec<f32>,
}
// TySON prefix: "e", value format: "dim|val1,val2,..."
// Primitive::EmbeddingPrimitive(EmbeddingPrimitive)
```

### 3.5.2 Implement distance metrics

```rust
trait DistanceMetric {
    fn distance(a: &[f32], b: &[f32]) -> f32;
}
struct CosineDistance;
struct EuclideanDistance;
struct DotProduct;
```

### 3.5.3 Implement HNSW index (~300 lines, zero deps)

- Multi-layer navigable small-world graph
- Per-document insertion/deletion (incremental, no retraining)
- Parameters: `M` (connections per node), `ef_construction`, `ef_search`
- 95-99% recall at millisecond speeds for millions of vectors
- Same approach as pgvector, Weaviate, Qdrant, FAISS

### 3.5.4 Pluggable index trait

```rust
pub trait IndexBackend {
    fn insert(&mut self, key: &IndexKey, link: &Link);
    fn remove(&mut self, key: &IndexKey, link: &Link);
    fn lookup(&self, op: &IndexOp, key: &IndexKey) -> Vec<Link>;
}

pub enum TypedIndex {
    BTree(BTreeIndex),
    Vector(VectorIndex),
}
```

Replace current `BTreeIndex` in `IndexManager` with `TypedIndex`.

### 3.5.5 Add `knn` query operator

```tyson
find{
    s|embedding|:knn{k:10, of:e|384|0.1,0.2,...|, using:cosine}
}
```

### 3.5.6 Hybrid query support (vector + structural)

```tyson
find{
    and[
        {s|embedding|:knn{k:50, of:e|384|...|}},
        {s|type|:eq{s|value|:memory}},
        {s|timestamp|:gt{uts|1700000000|}}
    ]
}
```

Vector index narrows candidates, B-tree filters by structure, results intersected.

### 3.5.7 Persistent vector index via snapshots

HNSW graph serializes alongside warehouse state in `snapshot.bin`.

---

## Phase 4: Architecture Cleanup

**Goal:** Fix layering violations, establish clean module boundaries.
**Risk:** Medium-High.

### 4.1 Move query-layer types out of `Primitive` enum

Types to extract: `DeleteQuery`, `KeepPrimitive`, `CollectionName`.
Keep true primitives: Link, String, Number, Bool, Null, UTS, Deleted, Embedding, PathToValue, Root.

### 4.2 Split `Storage` into traits

```rust
trait StorageRead { ... }
trait StorageWrite { ... }
trait IndexOps { ... }
```

Enables swapping protocol layer (ZMQ → HTTP) without touching storage.

### 4.3 Replace `String` parameters with `&str` (58 signatures)

### 4.4 Replace ZMQ with HTTP + WebSocket 🆕

**Remove:** `zmq`, `zmq-sys`, `libzmq` (C dependency, build friction).
**Add:** `axum` + `tokio` (or `actix-web`).

```
POST /tx              → run transaction, return response
WS   /ws              → streaming agent session
GET  /health          → liveness check
```

Benefits: no native deps, standard protocol, AI SDKs speak HTTP natively.

---

## Phase 5: Memory API Semantics 🆕

**Goal:** Add agent-oriented operations on top of raw TySON queries.
**Risk:** Medium (new abstraction layer, existing queries still work).

### 5.1 High-level memory operations

| Operation | Semantics |
|-----------|-----------|
| `remember` | Upsert: find by key, update if exists, insert if not |
| `recall` | Semantic recall: vector search + structural filter + link traversal |
| `relate` | Create typed, weighted edge between two documents |
| `forget` | TTL-based removal or explicit delete with cascade option |

### 5.2 Enhanced Link semantics

```rust
struct Link {
    collection_name: String,
    id: Uuid,
    relation_type: Option<String>,        // "knows", "contains", "child_of"
    metadata: HashMap<String, Primitive>,  // weight, confidence, timestamp
    links_to: Vec<Link>,
}
```

### 5.3 Namespacing / multi-tenancy

```rust
Storage::new(path, namespace: Option<String>)
```

Collection-level: `agent:alice:sessions`, `agent:bob:memories`.

---

## Phase 6: Error Handling Consistency

**Goal:** Unify error handling patterns.
**Risk:** Low.

- Remove broken `DBError::new()` — already done in Phase 1
- Merge `UnexpectedParsing` + `Deserialization` → `Deserialization(String)`
- Add `WalSerialization`, `SnapshotError` variants
- Restore `CanNotCompare` checks in `compare()`

---

## Phase 7: Clone Reduction & Performance

**Goal:** Reduce allocations in hot paths.
**Risk:** Low.

- Return `&Item` / `&Link` from getters instead of clones
- Eliminate `link.clone()` in sort comparator
- Store `wh_path` as struct field, pass `&str` to helpers

---

## Phase 8: Test Infrastructure

- Unit tests for vector distance metrics
- HNSW insertion/deletion/search correctness
- Integration tests for hybrid queries
- Snapshot/WAL round-trip with vector indexes
- `cargo test` with no ZMQ dependency

---

## Execution Order & Dependencies

```
Phase 1 (Safety)     ✅ COMPLETED
    ↓
Phase 2 (God funcs)  ← IN PROGRESS
    ↓
Phase 3 (Boilerplate)
    ↓
Phase 3.5 (Vector)   ← can start in parallel with Phase 4 if desired
    ↓
Phase 4 (Architecture + HTTP)
    ↓
Phase 5 (Memory API)
    ↓
Phase 6 (Errors)
    ↓
Phase 7 (Clones)
    ↓
Phase 8 (Tests)
```

---

## Progress Tracking

| Phase | Status | Started | Completed | Notes |
|-------|--------|---------|-----------|-------|
| 1: Safety Fixes | ✅ Done | 2026-07-22 | 2026-07-22 | 9 unwraps, bug fix, 6 error swallows, 4 tests |
| 2: God Functions | ✅ Done | 2026-07-22 | 2026-07-22 | run_transaction 179→40L, compare 114→28L, Storage::new 82→18L |
| 3: Boilerplate | ✅ Done | 2026-07-22 | 2026-07-22 | 6 ops→1 macro, 5 metas→1 macro, CompareOp enum |
| 3.5: Vector Search | ✅ Done | 2026-07-22 | 2026-07-22 | EmbeddingPrimitive, HNSW, knn op, hybrid queries, snapshot persistence |
| 4: Architecture | ✅ Done | 2026-07-22 | 2026-07-22 | String→&str (15 allocations removed), Storage traits, ZMQ→HTTP deferred |
| 5: Memory API | ✅ Done | 2026-07-22 | 2026-07-22 | remember, recall, relate, forget |
| 6: Error Handling | ✅ Done | 2026-07-22 | 2026-07-22 | Merged variants, WalSerialization+SnapshotError, CanNotCompare restored |
| 7: Clone Reduction | ✅ Done | 2026-07-22 | 2026-07-22 | Sort comparator clones eliminated |
| 8: Test Coverage | 🔶 In Progress | 2026-07-22 | — | 4 tests |

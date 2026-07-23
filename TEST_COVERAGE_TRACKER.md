# AnnaDB Test Coverage Tracker

**Current:** 89 unit tests (26.5% Windows) + 6 integration tests (32.7% Linux Docker)
**Target:** 90%+ line coverage

## Priority 1 — Integration Tests (estimated +25% coverage)

These require `Storage::new()` with temp directories. Run on Linux (Docker or CI).
All test files go in `tests/integration.rs`.

### 1.1 Memory lifecycle (4 tests needed)
- [ ] `remember_and_reopen_persists` — store, close Storage, reopen, verify document exists
- [ ] `remember_keyword_recall` — store "Paris is in France", `recall("paris")` returns it
- [ ] `remember_with_key_upserts_and_recall` — upsert, verify content updated
- [ ] `forget_then_recall_empty` — forget, verify recall returns nothing

### 1.2 Graph operations (4 tests needed)
- [ ] `relate_and_neighbors_with_type` — create "knows" edge, neighbors filtered by type
- [ ] `traverse_depth_2` — three nodes A→B→C, depth=2 returns B and C
- [ ] `path_finds_route` — A→B→C, `path(A, C)` returns 3 nodes
- [ ] `ego_graph_returns_center_and_neighbors` — verify center doc + N hop neighbors

### 1.3 Unwrapping (2 tests needed)
- [ ] `unwrap_include_types` — UnwrapConfig with "knows" only follows "knows" edges
- [ ] `unwrap_exclude_types` — UnwrapConfig excluding "audit_log" skips those edges

### 1.4 Collection isolation (2 tests needed)
- [ ] `list_collections_by_prefix` — `list_collections("proj:a:")` returns >=2 collections
- [ ] `cross_project_isolation` — project A recall doesn't return project B documents

---

## Priority 2 — PEG Parser Tests (estimated +12% coverage)

Target file: `src/tyson/de.rs` (213 lines, currently 0%)

### 2.1 Primitive deserialization (5 tests)
Add tests to `src/tyson/de.rs` or a new `tests/tyson_parser.rs`:

- [ ] `deserialize_string_primitive` — `"s|hello|"` → StringPrimitive("hello")
- [ ] `deserialize_number_primitive` — `"n|42.5|"` → NumberPrimitive(42.5)
- [ ] `deserialize_bool_primitive` — `"b|true|"` → BoolPrimitive(true)
- [ ] `deserialize_null_primitive` — `"null|"` → NullPrimitive
- [ ] `deserialize_invalid_primitive_errors` — `"x|bad|"` → Err

### 2.2 Map deserialization (2 tests)
- [ ] `deserialize_empty_map` — `"m{}"` → StorageMap::empty
- [ ] `deserialize_map_with_entries` — `"m{s|k|:s|v|}"` → StorageMap with one entry

### 2.3 Vector deserialization (2 tests)
- [ ] `deserialize_empty_vector` — `"v[]"` → StorageVector::empty
- [ ] `deserialize_vector_with_items` — `"v[s|a|,s|b|]"` → StorageVector with 2 items

### 2.4 Query deserialization (4 tests)
- [ ] `deserialize_find_query` — `"find[eq{s|x|:s|y|}]"` → FindQuery
- [ ] `deserialize_insert_query` — `"insert[m{s|name|:s|Ann|}]"` → InsertQuery
- [ ] `deserialize_get_query` — `"get[l|coll|uuid|]"` → GetQuery
- [ ] `deserialize_transaction` — `"q[insert[...]]"` → Transaction with steps

---

## Priority 3 — WAL & Snapshot Tests (estimated +10% coverage)

Target files: `src/storage/wal.rs` (178L), `src/storage/snapshot.rs` (135L)

### 3.1 WAL tests (3 tests)
Add to `tests/integration.rs`:

- [ ] `wal_append_and_replay` — write entry, reopen, verify replay recovers data
- [ ] `wal_multiple_entries` — write 5 entries, verify all replayed in order
- [ ] `wal_truncate_after_snapshot` — take snapshot, verify WAL truncated

### 3.2 Snapshot tests (2 tests)
- [ ] `snapshot_write_and_load` — store 2 collections, snapshot, reload, verify both exist
- [ ] `snapshot_with_vector_indexes` — create vector index, snapshot, reload, verify index restored

---

## Priority 4 — Query Processor Tests (estimated +8% coverage)

Target files: `src/query/insert/processor.rs` (23L), `src/query/get/processor.rs` (35L),
`src/query/update/processor.rs` (186L), `src/query/sort/processor.rs` (148L),
`src/query/project/processor.rs` (89L)

### 4.1 Insert processor (1 test)
- [ ] `insert_creates_link_and_updates_buffer` — call insert(), verify link returned, buffer has item

### 4.2 Get processor (1 test)
- [ ] `get_resolves_valid_link` — insert document, get by link, verify content returned

### 4.3 Update processor (2 tests)
- [ ] `update_set_field` — set name field, verify updated
- [ ] `update_inc_number` — inc age by 1, verify incremented

### 4.4 Sort processor (2 tests)
- [ ] `sort_ascending_by_field` — sort documents by name ascending
- [ ] `sort_descending` — sort documents by name descending

---

## Priority 5 — HTTP Server Tests (estimated +5% coverage)

Target file: `src/server.rs` (137L)

### 5.1 HTTP tests (3 tests)
Add to `tests/integration.rs`:

- [ ] `health_endpoint_returns_ok` — GET /health → 200 "AnnaDB ok"
- [ ] `tx_endpoint_processes_tyson` — POST /tx with valid TySON → 200 with response
- [ ] `tx_endpoint_errors_on_invalid` — POST /tx with invalid TySON → error response

---

## Priority 6 — Remaining Unit Tests (estimated +5% coverage)

### 6.1 TySON types
- [ ] `tyson/modifier.rs`: serialize AscOperator, DescOperator
- [ ] `tyson/se.rs`: journal format serialization with entries

### 6.2 Data type primitives  
- [ ] `data_types/primitives/path.rs`: PathToValue creation and value extraction
- [ ] `data_types/primitives/root.rs`: RootPrimitive prefix
- [ ] `data_types/primitives/number.rs`: NumberPrimitive add/subtract, comparison
- [ ] `data_types/primitives/bool.rs`: BoolPrimitive val()
- [ ] `data_types/primitives/null.rs`: NullPrimitive prefix
- [ ] `data_types/primitives/deleted.rs`: DeletedPrimitive prefix
- [ ] `data_types/primitives/unix_timestamp.rs`: UTSPrimitive creation

### 6.3 Query types
- [ ] `query/sort/query.rs`: AscOperator, DescOperator, SortQuery
- [ ] `query/project/query.rs`: ProjectQuery
- [ ] `query/project/operators/keep.rs`: KeepPrimitive

---

## Coverage Trajectory

| Step | Tests added | Cumulative coverage | Time estimate |
|------|-------------|-------------------|---------------|
| Current | 95 (89 unit + 6 integration) | 27% W / 33% L | — |
| P1: Integration | +12 | ~58% Linux | 2 hours |
| P2: PEG parser | +13 | ~70% Linux | 1.5 hours |
| P3: WAL/snapshot | +5 | ~80% Linux | 1 hour |
| P4: Query processors | +6 | ~88% Linux | 1.5 hours |
| P5: HTTP server | +3 | ~92% Linux | 30 min |
| P6: Remaining unit | +15 | ~97% Linux | 1 hour |

## How to run

```bash
# Unit tests (Windows/Linux/Mac)
cargo test --lib

# Integration tests (Linux/Mac only)
cargo test --test integration

# Docker (Linux, full coverage)
docker run --rm -v .:/app -w /app rust:latest \
  bash -c "cargo test && cargo test --test integration"

# Coverage (needs cargo-llvm-cov)
cargo llvm-cov --summary-only
```

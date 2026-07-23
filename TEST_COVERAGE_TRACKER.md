# AnnaDB Test Coverage Tracker

**Final:** 304 unit tests + 102 integration tests = 406 total (0 ignored)
**Coverage:** **90.23% line** | 83.55% region | 83.30% function

## Results

| Suite | Count | Status |
|-------|-------|--------|
| Unit tests | 304 | All pass |
| Integration tests | 102 | All pass |
| **Total** | **406** | **100% pass rate** |

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Tests | 95 | 406 | **+311** |
| Line Coverage | ~30% | **90.23%** | **+60%** |
| Region Coverage | — | **83.55%** | — |
| Function Coverage | — | **83.30%** | — |

## Bugs Fixed (5)

1. **`Primitive::new` rejects collection name prefixes** — Unknown prefixes (collection names used as Link prefixes) caused `Deserialization` errors when reading data back from disk. Fixed by treating unknown prefixes as `Link` types (`src/data_types/primitives/mod.rs:78`).

2. **`EOI` token breaks `Desereilize::deserialize`** — The PEG grammar produces an `EOI` (End of Input) token in journal inner pairs, causing `Transaction::deserialize` and all `run()` calls to fail. Fixed by skipping EOI tokens in the deserialization loop (`src/tyson/de.rs:122`).

3. **`neighbors()` doesn't resolve internal links** — Edge map values (`from`, `to`, `type`) are stored as internal links (`_internal` collection), but `neighbors` read them without resolution, making all graph operations (neighbors, traverse, path, ego_graph) return empty results. Fixed with `resolve_link_value()` and proper edge type resolution (`src/storage/main.rs:1214`).

4. **Server GET parsing hangs** — Without `Content-Length` header, `parse_request` fell through to `read_to_string(&mut body)` which blocks on EOF since the TCP connection stays alive. Fixed by removing the fallback and using empty body for requests without `Content-Length` (`src/server.rs:44`).

5. **Key-based upsert doesn't resolve internal links** — `remember()`'s key-based upsert check compared against stored map values directly, but those values are stored as internal links after `insert_item` processing. Fixed with `resolve_internal_string()` helper (`src/storage/main.rs:1077`).

6. **Config test race condition** — Parallel execution of env-var-modifying tests caused flaky failures. Fixed with a mutex lock (`src/config.rs:67`).

## How to run

```bash
# Unit tests
cargo test --lib

# Integration tests (Linux/Mac only)
cargo test --test integration

# Docker (Linux, full test suite)
docker run --rm -v .:/app -w /app rust:latest \
  bash -c "cargo test --lib && cargo test --test integration"

# Coverage
cargo llvm-cov --lib --test integration --summary-only
```

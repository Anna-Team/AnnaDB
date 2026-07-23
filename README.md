# AnnaDB — Embedded AI Agent Memory Engine

[![tests](https://img.shields.io/badge/tests-406%20passing-brightgreen)]()
[![coverage](https://img.shields.io/badge/coverage-84%25-brightgreen)]()
[![license](https://img.shields.io/badge/license-BSL%201.1-blue)](LICENSE)

AnnaDB is an embedded, local-first memory engine for AI agents. Document-oriented,
link-based graph model with vector search, zero-cycle graph traversal, and
HTTP-native protocol. Purpose-built for agents that need semantic recall,
structured relationships, and durable memory.

## Quick start

```bash
# Download and start the server
./anna_db --port 10001 --wh-path ./warehouse

# Or use as an embedded Rust library
let mut db = AnnaDB::open("warehouse", None)?;
db.remember("facts", "Paris is the capital of France", None, false, None)?;
let results = db.recall("facts", "paris capital", 5)?;
```

```bash
# Python embedded (pip install)
pip install annadb
```

```python
from annadb import AnnaDB
db = AnnaDB.open("warehouse")
link = db.remember("facts", "Paris is the capital of France")
docs = db.recall("facts", "paris", k=5)
```

## Features

| Feature | Description |
|---------|-------------|
| **Vector search** | HNSW index, cosine/euclidean/dot similarity, optional OpenAI embeddings |
| **Graph operations** | Typed edges, neighbors, BFS traversal, shortest path, ego graph |
| **Zero config** | Keyword recall works immediately; vector search via env var |
| **Embedded mode** | SQLite-like `open()` API for Rust + PyO3 Python bindings |
| **HTTP server** | POST /tx with TySON, GET /health, zero native dependencies |
| **Persistence** | WAL + periodic snapshots, crash recovery |
| **BSL licensed** | Free for individuals, academics, non-profits, companies < $5M |

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    AnnaDB                            │
│                                                      │
│  HTTP Server (std::net)     Embedded (Rust/Python)   │
│  POST /tx ← TySON          Storage::open()           │
│  GET /health                                         │
│                                                      │
│  ┌──────────┐  ┌──────────┐  ┌───────────────────┐  │
│  │ Document │  │  Vector  │  │      Graph        │  │
│  │  Store   │  │  Index   │  │  typed edges, BFS │  │
│  │ TySON    │  │  HNSW    │  │  path, traverse   │  │
│  └──────────┘  └──────────┘  └───────────────────┘  │
│                                                      │
│  WAL ─► snapshot.bin ─► warehouse/                   │
└─────────────────────────────────────────────────────┘
```

## Usage

### Embedded (Rust)

```rust
use annadb::AnnaDB;

let mut db = AnnaDB::open("warehouse", None)?;

// Store a document
let link = db.remember("facts", "Paris is in France", Some(("name", "paris")), false, None)?;

// Keyword search
let results = db.recall("facts", "paris", 5)?;

// Graph operations
let alice = db.remember("people", "Alice", None, false, None)?;
let bob = db.remember("people", "Bob", None, false, None)?;
db.relate(&alice, &bob, "knows", None)?;
let neighbors = db.neighbors(&alice, None)?;
```

### HTTP server

```bash
# Start server
./anna_db --port 10001 --wh-path ./warehouse

# Store a memory
curl -X POST :10001/tx -d 'remember s|collection|facts| s|content|Paris is in France|'

# Recall
curl -X POST :10001/tx -d 'recall s|collection|facts| s|query|paris| n|5|'

# List collections
curl -X POST :10001/tx -d 'list_collections s|prefix|project:|'
```

### With vector search

```bash
# OpenAI embeddings
EMBEDDING_PROVIDER=openai OPENAI_API_KEY=sk-... ./anna_db

# Local model (build from source with --features embedding-local)
EMBEDDING_PROVIDER=local ./anna_db
```

## opencode Integration

AnnaDB serves as persistent memory for opencode coding sessions. The agent
remembers decisions, files, bugs, and conventions across sessions.

```bash
# Setup
cp extensions/opencode/skills/annadb.md ~/.opencode/skills/

# Launch
./extensions/opencode/scripts/start-annadb.sh && opencode
```

The skill teaches the agent to use these tools:

| Tool | TySON | Purpose |
|------|-------|---------|
| `memory_remember` | `remember s|collection|...| s|content|...|` | Store facts, decisions |
| `memory_recall` | `recall s|collection|...| s|query|...|` | Retrieve relevant context |
| `memory_relate` | `relate s|from|...| s|to|...|` | Connect memories |
| `memory_forget` | `forget s|link|...|` | Delete memories |
| `memory_inspect` | `list_collections s|prefix|...|` | View stored data |

See `extensions/opencode/DESIGN.md` for the full architecture.

## Data model

### Primitives
```
s|hello|      string    n|42.5|       number
b|true|       bool      null|          null
l|coll|uuid|  link      e|384|0.1,...| embedding
```

### Collections

```
project:myapp:files        ← file summaries
project:myapp:decisions    ← architecture decisions
project:myapp:bugs         ← bugs and fixes
project:myapp:session_{id} ← session summaries (disposable)
user:preferences           ← global preferences
user:conventions           ← global coding conventions
```

## License

Business Source License 1.1 → Apache 2.0 after 2030-07-22.
Free for: individuals, academics, non-profits, companies < $5M revenue.
Commercial license required for larger organizations in production use.
See [LICENSE](LICENSE) and [LICENSE_AD](LICENSE_AD).

## Development

```bash
# Build
cargo build

# Run tests (406 tests on Linux, 293 unit + 102 integration)
cargo test

# Coverage (84% on Linux)
cargo llvm-cov --summary-only

# Docker test
docker run --rm -v .:/app -w /app rust:latest cargo test
```

## Links

- [Documentation](https://annadb.dev)
- [GitHub Issues](https://github.com/Anna-Team/AnnaDB/issues)
- [OpenCode extension design](extensions/opencode/DESIGN.md)
- [Test coverage tracker](TEST_COVERAGE_TRACKER.md)

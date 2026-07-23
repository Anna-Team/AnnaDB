# opencode + AnnaDB Memory Extension — Design

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     opencode process                     │
│                                                          │
│  ┌──────────────┐    spawns     ┌─────────────────────┐ │
│  │  memory skill │──────────────►   anna_db process    │ │
│  │  (annadb.md) │    HTTP       │   :10001             │ │
│  │              │◄──────────────│                      │ │
│  │  tools:      │   TySON      │   warehouse/         │ │
│  │  remember    │              │     embeddings       │ │
│  │  recall      │              │     vectors          │ │
│  │  relate      │              │     edges            │ │
│  │  inspect     │              │                      │ │
│  │  forget      │              └─────────────────────┘ │
│  └──────────────┘                                       │
│         │                                                │
│         │ loads at startup                               │
│         ▼                                                │
│  .opencode/skills/annadb.md   ← skill definition        │
│  .opencode/annadb/             ← AnnaDB binary + data   │
│  .opencode/config.json         ← AnnaDB config          │
└─────────────────────────────────────────────────────────┘
```

## File layout

```
.opencode/
  skills/
    annadb.md              ← skill definition (tools + prompts)
  annadb/
    anna_db.exe            ← single binary
    warehouse/             ← data directory
      wal.bin
      snapshot.bin
  config.json              ← already exists, add:
    "annadb": {
      "enabled": true,
      "port": 10001,
      "embeddingProvider": "openai",
      "embeddingModel": "text-embedding-3-small",
      "autoCapture": true
    }
```

## Skill definition — annadb.md

```markdown
# anna memory

You have persistent memory via AnnaDB. Use these tools to remember and
recall information across sessions.

## Lifecycle

At session start:
- AnnaDB is already running on :10001 (started by opencode launcher)
- Run health() to confirm
- If unreachable, inform user memory is unavailable

During conversation:
- After significant findings, remember them automatically
- Before answering complex questions, recall relevant context
- Link related memories with relate()

## Tools

### remember — store a fact, decision, or finding

Use this when:
- User makes a technical decision: "We'll use Redis for caching"
- You discover project conventions: "Tests go in __tests__/"
- You complete a task worth remembering: "Fixed login timeout bug"
- User states a preference: "I prefer tabs over spaces"

Parameters:
  type: "decision" | "preference" | "project_fact" | "task" | "bug_fix"
  content: the fact to remember
  project: current project name (optional)
  key: unique identifier to update existing (optional)

### recall — retrieve relevant memories

Use this before:
- Answering a question about project architecture
- Making a decision that might have precedent
- Understanding why something was done a certain way
- Finding related files or decisions

Parameters:
  query: natural language search query
  type: filter by memory type (optional)
  k: number of results (default 5)
  depth: graph unwrap depth (default 1 — follow related links)

### relate — connect two memories

Use this when:
- A decision affects a file
- Two memories are about the same topic
- A bug was found in a specific file

Parameters:
  from: link to first memory
  to: link to second memory
  type: "affects" | "related_to" | "found_in" | "imports"

### inspect — view stored memories

Use this when:
- User asks "what do you remember about X?"
- User wants to verify memory is correct
- Debugging memory behavior

Parameters:
  type: filter by type (optional)
  limit: max results (default 20)

### forget — delete a memory

Use this when:
- User says "forget that"
- Memory is incorrect or outdated

Parameters:
  link: link to the memory to delete

## Collection schema

Memories use these types:
  decision      — architecture choices, technical decisions
  preference    — user preferences, coding style  
  project_fact  — project structure, conventions, dependencies
  task          — completed or pending tasks
  bug_fix       — bugs found and their fixes
  file_summary  — summary of a file's purpose

Edges use these relation types:
  affects       — decision affects a file or another decision
  related_to    — two memories are semantically connected
  found_in       — bug was found in a file
  imports       — file imports another file
  extends       — memory enriches an existing one
  references    — memory references an external resource

## Embedding

When EMBEDDING_PROVIDER is configured:
- All memories automatically get embeddings
- recall() uses semantic vector search
- Similar memories are auto-linked
- Near-duplicates are linked with extends edges

Without embedding provider:
- recall() falls back to text matching
- No auto-linking or dedup
```

## opencode launcher integration

The launcher (or bootstrap script) starts AnnaDB before opencode:

```bash
#!/bin/bash
# Launch script — runs before opencode

ANNADB_PATH="$HOME/.opencode/annadb/anna_db"
ANNADB_DATA="$HOME/.opencode/annadb/warehouse"

# Download binary if not present (first run)
if [ ! -f "$ANNADB_PATH" ]; then
    mkdir -p "$(dirname "$ANNADB_PATH")"
    curl -L "https://github.com/Anna-Team/AnnaDB/releases/latest/download/anna_db-$(uname -s)-$(uname -m)" -o "$ANNADB_PATH"
    chmod +x "$ANNADB_PATH"
fi

# Start AnnaDB in background
EMBEDDING_PROVIDER="${ANNADB_EMBEDDING_PROVIDER:-}" \
EMBEDDING_MODEL="${ANNADB_EMBEDDING_MODEL:-text-embedding-3-small}" \
OPENAI_API_KEY="${OPENAI_API_KEY:-}" \
"$ANNADB_PATH" --port 10001 --wh-path "$ANNADB_DATA" &
ANNADB_PID=$!

# Trap to clean up on exit
trap "kill $ANNADB_PID 2>/dev/null" EXIT

# Run opencode
opencode "$@"
```

## Implementation tasks

### AnnaDB side — already done
- [x] HTTP server with POST /tx, GET /health
- [x] Single portable binary (no native deps)
- [x] Memory API: remember, recall, relate, forget
- [x] Graph: traverse, ego_graph, path
- [x] Vector search with HNSW
- [x] Auto-embedding via OpenAI
- [x] Auto-linking and dedup
- [x] Parametrized unwrapping (UnwrapConfig)
- [x] Python bindings
- [ ] Release binary builds on CI (GitHub Actions)

### opencode side — to build
- [ ] `skills/annadb.md` — skill definition with tools
- [ ] `launcher.sh` / `launcher.ps1` — bootstrap script
- [ ] `config.json` schema — AnnaDB section
- [ ] End-to-end test: store decision in session 1, recall in session 2
- [ ] License: MIT for the extension, AnnaDB binary under BSL

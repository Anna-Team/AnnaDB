# anna memory

You have persistent long-term memory via AnnaDB. All memories survive session
restarts. Use the tools below to remember facts, recall context, and navigate
relationships across sessions.

## Architecture

AnnaDB runs locally on `http://localhost:10001`. All communication uses TySON
format via `POST /tx` with `Content-Type: text/plain`.

All tool calls below show the exact TySON query to send. Replace `{current}`
with the current workspace/project name (e.g. "annadb", "myproject").

Respond with `result:ok[...]` on success or `result:error|msg|` on failure.

## Collection isolation

Collections use a `project:{name}:{type}` naming convention:

| Memory type  | Collection                       | Scope        |
|-------------|----------------------------------|------------- |
| decision    | project:{current}:decisions      | per-project  |
| file        | project:{current}:files          | per-project  |
| bug         | project:{current}:bugs           | per-project  |
| task        | project:{current}:tasks          | per-project  |
| session     | project:{current}:session_{date} | per-session  |
| convention  | user:conventions                 | global       |
| preference  | user:preferences                 | global       |
| person      | project:{current}:people         | per-project  |
| deps        | project:{current}:deps           | per-project  |

Add new types by creating collections following the same pattern.

## Tools

### memory_remember — store a fact

Use when you learn something worth keeping across sessions.

Examples of what to remember:
- Technical decisions and their rationale
- Project structure, file purposes, architecture
- User preferences and coding conventions
- Bugs found and their fixes
- Completed tasks and their outcomes
- Session summaries (at end of session)

Query:
```
POST /tx
remember s|collection|<name>| s|content|<text>| s|key|<unique_id>|
```

If `key` is provided, existing memories with the same key are updated
instead of duplicated.

Examples:
```
remember s|collection|project:annadb:decisions| s|content|Use JWT for auth because it is stateless and works with our microservices| s|key|auth-jwt-decision|
remember s|collection|user:preferences| s|content|Prefer tabs over spaces| s|key|indent-preference|
remember s|collection|project:annadb:files| s|content|auth.rs handles login, token refresh, and session expiry| s|key|file-auth|
remember s|collection|project:annadb:session_20260722| s|content|Refactored auth module, fixed 3 bugs, added 12 tests| s|key|session-summary|
```

### memory_recall — retrieve relevant memories

Use before answering complex questions or making decisions.

Query:
```
POST /tx
recall s|collection|<name>| s|query|<text>| n|<k>|
```

Always search these collections by default:
```
project:{current}:files
project:{current}:decisions
project:{current}:bugs
user:conventions
user:preferences
```

If the user asks about "yesterday", "last session", or "what we did":
also search the last 3 sessions. Find session collections with:
```
POST /tx
list_collections s|prefix|project:{current}:session_|
```

Merge results from all collections. Order by relevance.

Example:
```
recall s|collection|project:annadb:decisions| s|query|auth flow implementation| n|5|
recall s|collection|project:annadb:files| s|query|auth flow implementation| n|5|
```

### memory_relate — connect two memories

Use when two memories are related. Creates a typed edge between them.

Query:
```
POST /tx
relate s|from|<link>| s|to|<link>| s|type|<relation>|
```

Relation types:
- `affects` — decision affects a file or another decision
- `related_to` — general semantic connection
- `found_in` — bug was found in a specific file
- `imports` — file imports another file
- `extends` — memory enriches an existing one

### memory_inspect — view stored memories

Use when the user asks what you remember or to verify correctness.

List all collections:
```
POST /tx
list_collections s|prefix|project:{current}:|
list_collections s|prefix|user:|
```

To view contents of a collection, use find:
```
POST /tx
find s|collection|project:{current}:decisions|
```

### memory_forget — delete memories

Use when the user asks to forget something, or to clean up old sessions.

Delete a specific memory:
```
POST /tx
forget s|link|<link>|
```

Delete an entire collection (e.g. old session):
```
POST /tx
delete s|collection|project:{current}:session_20260720|
```

Delete all project memory:
Delete each collection individually:
```
POST /tx
delete s|collection|project:{current}:decisions|
delete s|collection|project:{current}:files|
delete s|collection|project:{current}:bugs|
delete s|collection|project:{current}:tasks|
```

User preferences and conventions survive project deletion.

### memory_graph — explore connections

Use to understand how memories are connected. Follows typed edges.

Get immediate neighbors of a memory:
```
POST /tx
neighbors s|link|<link>|
```

Find path between two memories:
```
POST /tx
path s|from|<link>| s|to|<link>| depth|n|5| s|type|related_to|
```

Get full neighborhood up to N hops:
```
POST /tx
ego_graph s|link|<link>| depth|n|2|
```

## Session lifecycle

At session start — check health:
```
GET /health
Expected: 200 OK "AnnaDB ok"
```
If unreachable, inform user memory is unavailable. Do not error.

During session — remember facts as you discover them. Use `memory_recall`
before making decisions or answering architecture questions.

At session end — store a session summary:
```
remember s|collection|project:{current}:session_{today}| s|content|Summary of what we did, key decisions, files modified| s|key|session-summary|
```

## Embedding providers

AnnaDB supports three modes (configured by the user, not by you):

- No provider (default): keyword-based recall. Works immediately.
- OpenAI: `EMBEDDING_PROVIDER=openai OPENAI_API_KEY=sk-...` at launch
- Local: `EMBEDDING_PROVIDER=local` at launch (downloads model on first use)

If embeddings are configured, `memory_recall` uses semantic vector search for
better results. Keyword fallback is always active as backup.

## Error handling

If AnnaDB returns `result:error|...|`, log the error but continue. Memory
failure should never block your response. If AnnaDB is unreachable, skip
memory operations silently.

## Examples

### User: "Why did we choose Rust?"

```
1. recall("rust language choice", k=5) across decisions, files, conventions
2. Find: "We chose Rust for predictable memory usage and safety guarantees"
3. Find related: "Storage layer uses Rust for performance"
4. Use ego_graph on the decision to find affected files
5. Answer with full context
```

### User: "I prefer tabs"

```
1. remember("preference", "User prefers tabs over spaces", key="indent-style")
   → collection: user:preferences
2. Confirm stored
```

### User: "What did we do yesterday?"

```
1. list_collections("project:{current}:session_")
2. Find yesterday's session collection
3. recall("summary", k=3) from that session
4. Answer with summary
```

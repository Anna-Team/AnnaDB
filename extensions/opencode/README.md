# opencode + AnnaDB Memory Extension

Persistent, local-first memory for opencode coding sessions. Remember facts,
decisions, bugs, and conventions. Recall them across sessions automatically.

## Setup

1. Copy the skill file:
```bash
mkdir -p ~/.opencode/skills/
cp extensions/opencode/skills/annadb.md ~/.opencode/skills/
```

2. Make the launcher executable:
```bash
chmod +x extensions/opencode/scripts/start-annadb.sh
```

3. Start opencode with the launcher:
```bash
extensions/opencode/scripts/start-annadb.sh && opencode
```

On Windows PowerShell:
```powershell
.\extensions\opencode\scripts\start-annadb.ps1; opencode
```

The launcher downloads AnnaDB on first run (~3MB), starts it in the
background, and kills it when opencode exits.

## Optional: enable embeddings

Better recall with vector search:

```bash
# OpenAI (needs API key)
ANNADB_EMBEDDING_PROVIDER=openai \
ANNADB_EMBEDDING_MODEL=text-embedding-3-small \
OPENAI_API_KEY=sk-... \
extensions/opencode/scripts/start-annadb.sh && opencode

# Local model (needs feature build)
# See AnnaDB README for build instructions
ANNADB_EMBEDDING_PROVIDER=local \
extensions/opencode/scripts/start-annadb.sh && opencode
```

Without embeddings, keyword-based recall works immediately.

## What gets remembered

| Type | Example | Collection |
|------|---------|-----------|
| Decision | "Use JWT for auth" | project:name:decisions |
| File | "auth.rs handles login" | project:name:files |
| Bug | "Null pointer in parseUser" | project:name:bugs |
| Task | "Refactored auth module" | project:name:tasks |
| Convention | "Use snake_case" | user:conventions |
| Preference | "Prefer tabs" | user:preferences |

Everything persists to `~/.opencode/annadb/warehouse/`.

## Data location

```
~/.opencode/annadb/
  anna_db              ← AnnaDB binary
  warehouse/           ← all data
    wal.bin            ← write-ahead log
    snapshot.bin       ← periodic snapshot
```

## Cleanup

```bash
# Remove all memory
rm -rf ~/.opencode/annadb/warehouse/

# Remove binary + all memory
rm -rf ~/.opencode/annadb/
```

## License

AnnaDB: BSL 1.1 → Apache 2.0 after 2030.
Extension files (skill, launcher, README): MIT.

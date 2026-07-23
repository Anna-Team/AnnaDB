#!/usr/bin/env bash
# AnnaDB launcher for opencode
# Starts AnnaDB as a background process and cleans up on exit.

set -euo pipefail

ANNADB_DIR="${HOME}/.opencode/annadb"
ANNADB_BIN="${ANNADB_DIR}/anna_db"
ANNADB_DATA="${ANNADB_DIR}/warehouse"
ANNADB_PORT="${ANNADB_PORT:-10001}"

download_binary() {
    local os arch url
    case "$(uname -s)" in
        Linux)  os="linux" ;;
        Darwin) os="macos" ;;
        *)      echo "Unsupported OS: $(uname -s)"; return 1 ;;
    esac
    case "$(uname -m)" in
        x86_64)  arch="x86_64" ;;
        aarch64) arch="aarch64" ;;
        arm64)   arch="aarch64" ;;
        *)       echo "Unsupported arch: $(uname -m)"; return 1 ;;
    esac

    mkdir -p "${ANNADB_DIR}"
    url="https://github.com/Anna-Team/AnnaDB/releases/latest/download/anna_db-${os}-${arch}"
    echo "Downloading AnnaDB from ${url}..."
    curl -fsSL "${url}" -o "${ANNADB_BIN}"
    chmod +x "${ANNADB_BIN}"
    echo "AnnaDB installed to ${ANNADB_BIN}"
}

# Download on first run
if [ ! -f "${ANNADB_BIN}" ]; then
    download_binary
fi

# Start AnnaDB
echo "Starting AnnaDB on port ${ANNADB_PORT}..."
EMBEDDING_PROVIDER="${ANNADB_EMBEDDING_PROVIDER:-}" \
EMBEDDING_MODEL="${ANNADB_EMBEDDING_MODEL:-}" \
OPENAI_API_KEY="${OPENAI_API_KEY:-}" \
"${ANNADB_BIN}" --port "${ANNADB_PORT}" --wh-path "${ANNADB_DATA}" &
ANNADB_PID=$!

# Kill AnnaDB when opencode exits
trap "kill ${ANNADB_PID} 2>/dev/null; echo 'AnnaDB stopped'" EXIT

# Wait a moment for startup
sleep 1
echo "AnnaDB running (PID ${ANNADB_PID})"

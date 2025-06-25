#!/bin/bash

# Build WASM and serve with cargo
set -e

PORT=${1:-8000}

echo "Building WASM..."
wasm-pack build --target web --features wasm

echo "Copying WASM files to html folder..."
cp pkg/ascii_ansi_table.js pkg/ascii_ansi_table_bg.wasm pkg/ascii_ansi_table.d.ts html/pkg/

echo "Starting server on port $PORT..."
cargo install basic-http-server --quiet 2>/dev/null || true
basic-http-server ./html -a 0.0.0.0:$PORT
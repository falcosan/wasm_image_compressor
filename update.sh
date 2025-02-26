#!/usr/bin/env bash

set -euo pipefail

WASM_FOLDER="$(dirname "$0")/wasm"

echo "Starting dependency update process..."

if ! command -v cargo &> /dev/null; then
  echo "Error: cargo is not installed or not in PATH" >&2
  exit 1
fi

if [ ! -d "$WASM_FOLDER" ]; then
  echo "Error: WASM folder not found at $WASM_FOLDER" >&2
  exit 1
fi

echo "Entering WASM folder at $WASM_FOLDER"
cd "$WASM_FOLDER"

echo "Running cargo upgrade..."
if ! cargo upgrade -i allow; then
  echo "Error: cargo upgrade failed" >&2
  exit 1
fi

echo "Running cargo update..."
if cargo update; then
  echo "Dependency update completed successfully"
else
  echo "Error: cargo update failed" >&2
  exit 1
fi
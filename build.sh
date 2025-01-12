#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WASM_DIR="${SCRIPT_DIR}/wasm"
PKG_DIR="${WASM_DIR}/pkg"
LIB_DIR="${PKG_DIR}/lib"

# Build WebAssembly package
cd "${WASM_DIR}"
wasm-pack build --out-name index --target web --out-dir pkg/lib

# Copy and move files
cp "${SCRIPT_DIR}/README.md" "${PKG_DIR}/"
mv "${LIB_DIR}/.gitignore" "${PKG_DIR}/"
mv "${LIB_DIR}/package.json" "${PKG_DIR}/"

# Update package.json
cd "${PKG_DIR}"
jq '.files = ["lib/"] | .main = "lib/index.js" | .types = "lib/index.d.ts"' package.json > temp_package.json && mv temp_package.json package.json

#!/bin/bash

cd wasm

wasm-pack build --out-name index --target web --out-dir pkg/lib

cd pkg/lib

cp ../../../README.md ../

mv .gitignore ../
mv package.json ../

cd ..
jq '.files = ["lib/"] | .main = "lib/index.js" | .types = "lib/index.d.ts"' package.json > temp_package.json && mv temp_package.json package.json

npm publish --access=public

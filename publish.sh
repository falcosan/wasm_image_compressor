#!/bin/bash

cd wasm

wasm-pack build --out-name index --target web --out-dir dist

cd dist

npm publish --access=public

cd ..


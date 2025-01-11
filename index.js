"use_strict";

import wasmModule from "./wasm/pkg";

module.exports = {
  initWasm: wasmModule.default,
  convertImage: wasmModule.convertImage,
};

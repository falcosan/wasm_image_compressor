const wasmModule = require("./wasm/pkg");

module.exports = {
  initWasm: wasmModule.default,
  convertImage: wasmModule.convertImage,
};

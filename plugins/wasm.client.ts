import initWasm from "wasm_image_compressor";
import module_or_path from "wasm_image_compressor/lib/index_bg.wasm?url";

export default defineNuxtPlugin(async () => {
  await initWasm({ module_or_path });
});

import initWasm from "@@/wasm/pkg";

export default defineNuxtPlugin(async () => {
  await initWasm();
});

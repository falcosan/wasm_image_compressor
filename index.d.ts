declare module "wasm_image_compressor" {
  export function initWasm(): Promise<void>;
  export function convertImage(
    file: Uint8Array,
    src_type: string,
    target_type: string,
    compression: number,
    cb: Function
  ): Uint8Array;
}

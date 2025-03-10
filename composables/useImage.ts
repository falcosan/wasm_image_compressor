import { convertImage as wasmConvertImage } from "wasm_image_compressor";

export const useImage = () => {
  const inputFileEndings = {
    "image/png": "png",
    "image/webp": "webp",
    "image/jpeg": "jpg",
    "image/x-icon": "ico",
  } as const;

  const acceptList = "image/*";

  const convertImage = async (params: {
    outputType?: string;
    compressionFactor?: number;
    fileOrURL: string | Uint8Array;
    inputType: keyof typeof inputFileEndings;
  }) => {
    try {
      const {
        fileOrURL,
        inputType,
        compressionFactor = 0.5,
        outputType = "image/webp",
      } = params;

      const result = await wasmConvertImage(
        fileOrURL,
        inputType,
        outputType,
        compressionFactor
      );

      return { data: result, success: true };
    } catch (error) {
      return { success: false, error: String(error) };
    }
  };

  const downloadImage = (url: string, filename: string) => {
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  };

  return {
    acceptList,
    convertImage,
    downloadImage,
    inputFileEndings,
  };
};

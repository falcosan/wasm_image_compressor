import { convertImage } from "wasm_image_compressor";
import type { WorkerRequest, WorkerResponse } from "@/schema/convert";

export const useImage = () => {
  const inputFileEndings = {
    "image/webp": "webp",
    "image/jpeg": "jpeg",
    "image/png": "png",
    "image/x-icon": "ico",
  };

  const acceptList = ["image/*"].join(",");

  const imageConverter = async (
    request: WorkerRequest
  ): Promise<WorkerResponse> => {
    try {
      const { inputFile, inputType, outputType, compressionFactor } = request;
      const result = await new Promise<string>((resolve, reject) => {
        try {
          const res = convertImage(
            inputFile,
            inputType,
            outputType,
            compressionFactor
          );
          resolve(res);
        } catch (error) {
          reject(error);
        }
      });

      return {
        data: result,
        success: true,
      };
    } catch (error) {
      return {
        success: false,
        error: String(error),
      };
    }
  };

  const getMimeType = (
    file: File
  ): keyof typeof inputFileEndings | "application/octet-stream" => {
    if (file.type === "") {
      const extension = file.name.split(".").pop();

      if (extension !== undefined) {
        Object.entries(inputFileEndings).forEach(([mimeType, ext]) => {
          if (ext === extension.toLowerCase()) return mimeType;
        });
      }

      return "application/octet-stream";
    }

    return file.type as keyof typeof inputFileEndings;
  };

  const downloadImage = (url: string, filename: string) => {
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();
    a.remove();
    URL.revokeObjectURL(url);
  };

  return {
    acceptList,
    getMimeType,
    downloadImage,
    imageConverter,
    inputFileEndings,
  };
};

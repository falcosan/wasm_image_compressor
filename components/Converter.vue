<script setup lang="ts">
import type { WorkerRequest, WorkerResponse } from "@/schema/convert";

const { getMimeType, imageConverter, inputFileEndings, downloadImage } =
  useImage();

const file = ref<File>();
const outputType = ref("image/webp" as keyof typeof inputFileEndings);

const trimFileExtension = (filename: string) => {
  if (!filename) return "untitled_file_compressed";

  return filename.indexOf(".") === -1
    ? filename
    : filename.split(".").slice(0, -1).join(".");
};
const startConversion = async () => {
  if (file.value) {
    const reader = new FileReader();

    reader.onloadend = async (e) => {
      const res = e.target?.result as ArrayBuffer;
      const arr = new Uint8Array(res);

      if (!file.value) return;

      try {
        const params: WorkerRequest = {
          inputFile: arr,
          compressionFactor: 0.1,
          outputType: outputType.value,
          inputType: getMimeType(file.value),
        };

        const response: WorkerResponse = await imageConverter(params);

        if (response.success && response.data) {
          const filename = trimFileExtension(file.value.name);
          const filetype = inputFileEndings[outputType.value];

          downloadImage(response.data, `${filename}.${filetype}`);
        } else if (response.error) {
          console.error("Conversion error:", response.error);
        }
      } catch (error) {
        console.error("Unexpected error during conversion:", error);
      }
    };

    reader.readAsArrayBuffer(file.value);
  }
};
</script>

<template>
  <div class="w-3/4 max-w-2xl">
    <InputsFile v-model:file="file" />
    <div class="flex flex-row items-end space-x-10 pt-3">
      <div class="grow">
        <InputsSelect
          v-model="outputType"
          name="outputType"
          label="Select a File Type"
          placeholder="Select a File Type"
        >
          <option
            v-for="(imageType, ending) in inputFileEndings"
            :key="ending"
            :value="ending"
          >
            {{ imageType }}
          </option>
        </InputsSelect>
      </div>
      <InputsButton @click="startConversion">Convert</InputsButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WorkerRequest, WorkerResponse } from "@/schema/convert";

const { imageConverter, inputFileEndings } = useImage();

const file = ref<File>();
const outputType = ref("image/webp" as keyof typeof inputFileEndings);

const startDownload = (file: Uint8Array, filename: string) => {
  const blob = new Blob([file], { type: "application/octet-stream" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
};

const startConversion = async () => {
  if (file.value) {
    const reader = new FileReader();

    reader.onloadend = async (e) => {
      const res = e.target?.result as ArrayBuffer;
      const arr = new Uint8Array(res);

      if (!file.value) {
        return;
      }

      try {
        const params: WorkerRequest = {
          inputFile: arr,
          compressionStrength: 0.15,
          outputType: outputType.value,
          inputType: getMimeType(file.value),
        };

        const response: WorkerResponse = await imageConverter(params);

        if (response.success && response.data) {
          startDownload(
            response.data,
            `converted.${inputFileEndings[outputType.value]}`
          );
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

const getMimeType = (file: File): string => file.type;
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

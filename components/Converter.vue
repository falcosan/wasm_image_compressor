<script setup lang="ts">
const { convertImage, inputFileEndings, downloadImage } = useImage();

const file = ref<File>();
const compressionFactor = ref(1);
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
      const res = e.target?.result;

      if (!res || !file.value) return;

      try {
        const params = {
          outputType: outputType.value,
          compressionFactor: compressionFactor.value,
          fileOrURL: new Uint8Array(res as ArrayBuffer),
          inputType: file.value.type as keyof typeof inputFileEndings,
        };

        const response = await convertImage(params);

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
  <InputsFile v-model:file="file" />
  <div class="mt-5">
    <div class="flex flex-wrap grow gap-10">
      <InputsNumber
        v-model="compressionFactor"
        name="compressorFactor"
        label="Compression factor"
        placeholder="Compression factor"
      />
      <InputsSelect
        v-model="outputType"
        class="flex-auto"
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
    <div class="flex justify-end">
      <button
        type="button"
        class="flex items-center justify-center text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 py-2 mt-5 focus:outline-none"
        @click="startConversion"
      >
        Convert
      </button>
    </div>
  </div>
</template>

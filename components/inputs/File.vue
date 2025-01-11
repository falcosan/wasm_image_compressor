<script setup lang="ts">
const { acceptList } = useImage();

interface Props {
  file: File | undefined;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  (e: "update:file", file: File | undefined): void;
}>();

const data = useVModel(props, "file", emit);

const dropZoneRef = ref<HTMLLabelElement | null>(null);

const imgSource = computed(() =>
  data.value ? URL.createObjectURL(data.value) : undefined
);

const setFile = (files: File[] | null | FileList): void => {
  if (files?.length) {
    data.value = files[0];
  }
};
const removeFile = (): void => (data.value = undefined);
const onUpdate = (e: Event): void => {
  const target = e.target as HTMLInputElement;
  if (target.files) {
    setFile(target.files);
  }
};

const { isOverDropZone } = useDropZone(dropZoneRef, setFile);
</script>

<template>
  <div class="flex items-center justify-center w-full">
    <label
      ref="dropZoneRef"
      for="file-dropzone"
      class="flex flex-col items-center justify-center w-full h-64 border-2 border-dashed rounded-lg cursor-pointer bg-slate-600 hover:bg-slate-500"
      :class="{
        'border-primary-500': isOverDropZone,
        'border-gray-300': !isOverDropZone,
      }"
    >
      <div class="flex flex-col items-center justify-center pt-5 pb-6">
        <p v-if="!data" class="mb-2 text-sm text-gray-50">
          Click to upload or drag and drop
        </p>
        <div v-else class="grid place-items-center">
          <div class="relative group" @click.capture.prevent="removeFile">
            <div
              class="absolute text-white inset-0 hidden group-hover:grid place-items-center backdrop-brightness-50"
            >
              <span>Remove file</span>
            </div>
            <img
              :src="imgSource"
              class="object-scale-down max-h-56 max-w-xl min-h-24 min-w-24"
              alt="Uploaded file preview"
            />
          </div>
        </div>
      </div>
      <input
        id="file-dropzone"
        type="file"
        :accept="acceptList"
        class="hidden"
        @change="onUpdate"
      />
    </label>
  </div>
</template>

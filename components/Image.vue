<template>
  <img v-if="!isRasterImage" v-bind="attrs" ref="targetImage" />
  <img v-else v-bind="forwarded" ref="targetImage" :src="url" />
</template>

<script setup lang="ts">
import { useElementVisibility } from "@vueuse/core";

defineOptions({ inheritAttrs: false });

const props = withDefaults(
  defineProps<{
    compression?: number | string;
  }>(),
  { compression: 1 }
);

const nuxtApp = useNuxtApp();
const { convertImage, inputFileEndings } = useImage();
const attrs = useAttrs() as { src: string } & Record<
  string,
  string | number | undefined
>;

const url = ref(attrs.src || "");
const blobUrls = ref<string[]>([]);

const targetImage = useTemplateRef("targetImage");
const targetIsVisible = useElementVisibility(targetImage);

const { data, refresh } = await useAsyncData<{
  arr: ArrayBuffer;
  type: keyof typeof inputFileEndings;
}>(
  attrs.src,
  async () => {
    const response = await $fetch<Blob>("/api/image", {
      responseType: "blob",
      query: { url: attrs.src },
    });

    return {
      arr: await response.arrayBuffer(),
      type: response.type as keyof typeof inputFileEndings,
    };
  },
  {
    transform(data) {
      return { ...data, fetchedAt: new Date() };
    },
    getCachedData(key) {
      const data = nuxtApp.payload.data[key] || nuxtApp.static.data[key];

      if (!data) return;

      const expirationDate = new Date(data.fetchedAt);
      expirationDate.setTime(expirationDate.getTime() + 10 * 1000);
      const isExpired = expirationDate.getTime() < Date.now();

      if (isExpired) return;

      return data;
    },
  }
);

const reactiveURL = computed(() => attrs.src);
const forwarded = computed(() =>
  filterObject({ ...attrs }, ["src", "loading"], false)
);
const compression = computed(() => {
  const value =
    typeof props.compression === "string"
      ? Number(props.compression)
      : props.compression;
  if (isNaN(value) || value < 0 || value > 1) {
    console.warn(
      `${value} is an invalid compression value. It should be a number between 0 and 1. The fallback value of 1 will be used.`
    );
    return 1;
  }
  return value;
});
const isRasterImage = computed(() => {
  if (!data.value) return false;

  if (compression.value === 1) {
    const filteredTypes = filterObject(
      inputFileEndings,
      "image/webp",
      false
    ) as Omit<typeof inputFileEndings, "image/webp">;
    return Object.values(inputFileEndings).includes(
      filteredTypes[data.value.type as keyof typeof filteredTypes]
    );
  }

  const mimeType = inputFileEndings[data.value.type] || "";
  return Object.values(inputFileEndings).includes(mimeType);
});

const convertImageByCompression = async () => {
  if (!isRasterImage.value || !data.value) return;

  const newUrl = await convertImage({
    inputType: data.value.type,
    compressionFactor: compression.value,
    fileOrURL: new Uint8Array(data.value.arr),
  });

  if (newUrl.data) {
    url.value = newUrl.data;
    blobUrls.value.push(newUrl.data);
  }
};

const revokeBlobUrls = () => {
  blobUrls.value.forEach((blobUrl) => {
    URL.revokeObjectURL(blobUrl);
  });
  blobUrls.value = [];
};

onUnmounted(revokeBlobUrls);

watch(reactiveURL, async (val) => {
  url.value = val;
  await refresh().then(convertImageByCompression);
});
watch(targetIsVisible, async (val) => {
  if (val) await convertImageByCompression();
});
</script>

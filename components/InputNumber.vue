<script lang="ts" setup>
const props = defineProps<{
  name: string;
  label: string;
  placeholder: string;
  modelValue: number | string;
}>();

const data = useVModel(props, "modelValue");
</script>

<template>
  <div>
    <label :for="name" class="block mb-2 text-sm font-medium text-white">
      {{ label }}
    </label>
    <input
      v-model="data"
      class="w-full bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5"
      min="0"
      max="1"
      step="0.1"
      type="number"
    />
  </div>
</template>

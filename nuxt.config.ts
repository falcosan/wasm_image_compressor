export default defineNuxtConfig({
  modules: ["@vueuse/nuxt", "@nuxt/eslint", "@nuxtjs/tailwindcss"],

  future: {
    compatibilityVersion: 4,
  },

  experimental: {
    payloadExtraction: false,
  },

  devtools: { enabled: false },
  compatibilityDate: "2025-01-10",
});

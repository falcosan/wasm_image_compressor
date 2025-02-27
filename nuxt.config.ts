import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  devtools: { enabled: false },
  css: ["~/assets/css/main.css"],
  compatibilityDate: "2025-01-10",
  vite: { plugins: [tailwindcss()] },
  modules: ["@vueuse/nuxt", "@nuxt/eslint"],
});

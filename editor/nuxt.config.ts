import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  modules: [
    '@nuxt/eslint',
    '@nuxt/image',
    '@nuxtjs/color-mode',
    '@pinia/nuxt',
    '@vueuse/nuxt',
    'nuxt-lucide-icons'
  ],
  css: ['~/assets/css/tailwind.css', '~/assets/css/index.css'],
  vite: {
    plugins: [
      tailwindcss()
    ],
    server: {
      hmr: true,
      allowedHosts: ['lh.justxale.com'],
    },
    optimizeDeps: {
      include: [
        '@vue/devtools-core',
        '@vue/devtools-kit',
        'rete',
        'rete-area-plugin',
        'rete-connection-plugin',
        'rete-vue-plugin',
      ]
    }

  },
})
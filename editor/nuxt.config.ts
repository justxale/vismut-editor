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
    'nuxt-lucide-icons',
    'shadcn-nuxt',
    '@nuxt/fonts'
  ],
  css: ['~/assets/css/tailwind.css', '~/assets/css/index.css'],
  devServer: {
    port: 11812
  },
  app: {
    rootAttrs: {
      id: "vismut-root"
    }
  },
  vite: {
    plugins: [
      tailwindcss()
    ],
    server: {
      hmr: true,
      allowedHosts: ['lh.justxale.com', "http://localhost:11811"],
    },
    optimizeDeps: {
      include: [
        '@vue/devtools-core',
        '@vue/devtools-kit',
        '@vue-flow/core',
        '@vue-flow/background',
        'reka-ui',
        'lucide-vue-next',
        'clsx',
        'tailwind-merge'
      ]
    }
  },
  shadcn: {
    prefix: '',
    componentDir: '@/components/ui'
  }
})
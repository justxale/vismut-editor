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
        'rete-render-utils',
      ]
    }
  },
  shadcn: {
    /**
     * Prefix for all the imported component.
     * @default "Ui"
     */
    prefix: '',
    /**
     * Directory that the component lives in.
     * Will respect the Nuxt aliases.
     * @link https://nuxt.com/docs/api/nuxt-config#alias
     * @default "@/components/ui"
     */
    componentDir: '@/components/ui'
  }
})
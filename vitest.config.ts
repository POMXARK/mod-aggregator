import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from 'path';

export default defineConfig({
  plugins: [
    svelte({
      compilerOptions: {
        runes: true,
        compatibility: {
          componentApi: 4,
        },
      },
      hot: !process.env.VITEST,
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
    conditions: ['browser', 'development'],
    mainFields: ['browser', 'module', 'main'],
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    server: {
      deps: {
        inline: ['@testing-library/svelte', 'svelte'],
      },
    },
  },
  optimizeDeps: {
    exclude: ['@testing-library/svelte'],
  },
});


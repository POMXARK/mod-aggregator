import { defineConfig } from 'vite';
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
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  clearScreen: false,
  server: {
    port: 1421,
    strictPort: false, // Автоматически выберет свободный порт, если 1421 занят
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
});


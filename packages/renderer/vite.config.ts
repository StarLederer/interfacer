import { resolve } from 'path';
import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import unocss from '@unocss/vite';
import presetIcons from '@unocss/preset-icons';
import transformerDirective from '@unocss/transformer-directives'
import unocssPresetWrapp from "./unocss-preset";

// https://tauri.app/v1/guides/getting-started/setup/vite/
const tauriRequired = defineConfig({
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // Tauri expects a fixed port, fail if that port is not available
  server: {
    port: 5173,
    strictPort: true,
  },
  // to make use of `TAURI_PLATFORM`, `TAURI_ARCH`, `TAURI_FAMILY`,
  // `TAURI_PLATFORM_VERSION`, `TAURI_PLATFORM_TYPE` and `TAURI_DEBUG`
  // env variables
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: ['es2021', 'chrome100', 'safari13'],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});

export default defineConfig({
  ...tauriRequired,

  resolve: {
    alias: {
      '~': resolve('./src'),
    },
  },

  plugins: [
    solidPlugin(),
    unocss({
      presets: [
        presetIcons(),
        unocssPresetWrapp(),
      ],
      transformers: [
        transformerDirective(),
      ],
    }),
  ],
  build: {
    target: 'esnext',
  },
});

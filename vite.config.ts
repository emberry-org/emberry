import path from 'path';

import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import solidSvg from 'vite-plugin-solid-svg';

export default defineConfig({
    plugins: [solidPlugin(), solidSvg({
        defaultAsComponent: true
    })],
    server: {
        port: 5258,
        /* important for Tauri */
        strictPort: true,
    },
    build: {
        outDir: '.solid',
        /* slightly better minification */
        minify: 'terser',
        target: 'esnext',
    },
    resolve: {
        alias: [
            /* icon directory shortcut */
            { find: '@ico', replacement: path.resolve('./assets/icons') }
        ],
    },
    optimizeDeps: {
        // Add both @codemirror/state and @codemirror/view to included deps to optimize
        include: ['@codemirror/state', '@codemirror/view'],
    },
    logLevel: 'info'
});

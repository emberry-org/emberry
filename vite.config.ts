import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tsconfigPaths from 'vite-tsconfig-paths'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), tsconfigPaths()],
  optimizeDeps: { exclude: ["svelte-navigator"] },
  server: {
    hmr: { overlay: false },
    port: 3000
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: (id) => {
          if (id.includes("node_modules")) {
            if (id.includes("@tauri-apps")) {
              return "tauri";

            // Code Mirror & Lezer are very large // Code Mirror requires style-mod
            } else if (id.includes("codemirror") || id.includes("style-mod")) {
              return "codemirror";
            } else if (id.includes("@lezer")) {
              return "lezer"
            }
        
            return "vendor"; // all other package goes here
          }
        }
      }
    }
  }
})

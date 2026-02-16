import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig({
  root: ".",
  server: {
    port: 5173,
    strictPort: true,
    fs: {
      allow: [".."]   // VERY IMPORTANT → allow bubble folder
    }
  },
  resolve: {
    alias: {
      "@": resolve(__dirname, "./")
    }
  },
  build: {
    outDir: "dist",
    emptyOutDir: true
  }
});

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// The built viewer is embedded into the `furcule` binary and served at `/`.
export default defineConfig({
  plugins: [react()],
  base: "./",
  build: { outDir: "dist", emptyOutDir: true },
  server: {
    port: 5173,
    proxy: { "/api": "http://127.0.0.1:7429" },
  },
});

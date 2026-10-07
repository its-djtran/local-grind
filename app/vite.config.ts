import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  build: { target: "safari15", outDir: "dist", emptyOutDir: true },
  resolve: process.env.VITEST ? { conditions: ["browser"] } : undefined,
  test: {
    environment: "jsdom",
    setupFiles: ["src/test/setup.ts"],
    include: ["src/**/*.test.ts"],
    coverage: {
      provider: "v8",
      include: ["src/**/*.{ts,svelte}"],
      exclude: ["src/**/*.test.ts", "src/test/**", "src/mock/**", "src/main.ts", "src/vite-env.d.ts"],
      reporter: ["text-summary", "text"],
      thresholds: { lines: 90, statements: 90, functions: 90, branches: 75 },
    },
  },
});

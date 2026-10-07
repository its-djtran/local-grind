import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";

async function start() {
  // `npm run dev:mock` previews the UI in a browser against an in-memory backend.
  if (import.meta.env.VITE_MOCK === "1") {
    const { mockIPC } = await import("@tauri-apps/api/mocks");
    const { createBackend } = await import("./mock/backend");
    const backend = createBackend();
    mockIPC((cmd, args) => backend.handle(cmd, (args ?? {}) as Record<string, unknown>), { shouldMockEvents: true });
  }
  mount(App, { target: document.getElementById("app")! });
  const { maybeRunSmoke } = await import("./lib/smoke");
  await maybeRunSmoke();
}

start();

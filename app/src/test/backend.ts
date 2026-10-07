import { mockIPC } from "@tauri-apps/api/mocks";
import { createBackend, type Backend } from "../mock/backend";

/** Installs a fresh in-memory backend behind the Tauri IPC layer. */
export function installBackend(override?: (cmd: string, args: Record<string, unknown>) => unknown): Backend {
  const backend = createBackend();
  mockIPC(
    (cmd, args) => {
      const a = (args ?? {}) as Record<string, unknown>;
      if (override) {
        const r = override(cmd, a);
        if (r !== undefined) return r;
      }
      return backend.handle(cmd, a);
    },
    { shouldMockEvents: true },
  );
  return backend;
}

export const tick = (ms = 0) => new Promise((r) => setTimeout(r, ms));

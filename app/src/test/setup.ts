import "@testing-library/jest-dom/vitest";
import { afterEach, vi } from "vitest";
import { cleanup } from "@testing-library/svelte";
import { clearMocks } from "@tauri-apps/api/mocks";

// jsdom lacks layout APIs that CodeMirror measures with.
const rect = { x: 0, y: 0, top: 0, left: 0, bottom: 0, right: 0, width: 0, height: 0, toJSON: () => ({}) };
Range.prototype.getBoundingClientRect = () => rect as DOMRect;
Range.prototype.getClientRects = () => ({ length: 0, item: () => null, [Symbol.iterator]: [][Symbol.iterator] }) as unknown as DOMRectList;
Element.prototype.scrollIntoView = () => {};
if (!window.matchMedia) {
  window.matchMedia = (q: string) =>
    ({ matches: q.includes("dark"), media: q, addEventListener() {}, removeEventListener() {} }) as unknown as MediaQueryList;
}

afterEach(() => {
  cleanup();
  clearMocks();
  sessionStorage.clear();
  localStorage.clear();
});

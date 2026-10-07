import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { createRawSnippet } from "svelte";
import SplitPane from "./SplitPane.svelte";

const snip = (text: string) => createRawSnippet(() => ({ render: () => `<span>${text}</span>` }));

function setup(direction: "horizontal" | "vertical", storageKey?: string) {
  const r = render(SplitPane, { direction, initial: 0.5, storageKey, first: snip("A"), second: snip("B") });
  const sep = screen.getByRole("separator");
  const container = sep.parentElement!;
  container.getBoundingClientRect = () => ({ left: 0, top: 0, width: 1000, height: 500 }) as DOMRect;
  return { r, sep };
}

describe("SplitPane", () => {
  it("renders both panes and resizes by keyboard", async () => {
    const { sep } = setup("horizontal", "k1");
    expect(screen.getByText("A")).toBeInTheDocument();
    expect(sep).toHaveAttribute("aria-valuenow", "50");
    await fireEvent.keyDown(sep, { key: "ArrowRight" });
    expect(sep).toHaveAttribute("aria-valuenow", "52");
    await fireEvent.keyDown(sep, { key: "ArrowLeft", shiftKey: true });
    expect(sep).toHaveAttribute("aria-valuenow", "42");
    await fireEvent.keyDown(sep, { key: "x" });
    expect(Number(localStorage.getItem("k1"))).toBeCloseTo(0.42);
  });

  it("drags with the pointer and clamps", async () => {
    const { sep } = setup("vertical", "k2");
    await fireEvent.pointerMove(sep, { clientY: 100 });
    expect(sep).toHaveAttribute("aria-valuenow", "50");
    await fireEvent.pointerDown(sep, { pointerId: 1 });
    await fireEvent.pointerMove(sep, { clientY: 100 });
    expect(sep).toHaveAttribute("aria-valuenow", "20");
    await fireEvent.pointerMove(sep, { clientY: 1 });
    expect(sep).toHaveAttribute("aria-valuenow", "15");
    await fireEvent.pointerUp(sep);
    await fireEvent.pointerUp(sep);
    expect(localStorage.getItem("k2")).toBe("0.15");
    await fireEvent.keyDown(sep, { key: "ArrowDown" });
    expect(sep).toHaveAttribute("aria-valuenow", "17");
  });

  it("restores a stored ratio and ignores zero-size containers", async () => {
    localStorage.setItem("k3", "0.7");
    const { sep } = setup("horizontal", "k3");
    expect(sep).toHaveAttribute("aria-valuenow", "70");
    sep.parentElement!.getBoundingClientRect = () => ({ left: 0, top: 0, width: 0, height: 0 }) as DOMRect;
    await fireEvent.pointerDown(sep, { pointerId: 1 });
    await fireEvent.pointerMove(sep, { clientX: 10 });
    expect(sep).toHaveAttribute("aria-valuenow", "70");
  });

  it("works without persistence", async () => {
    const { sep } = setup("horizontal");
    await fireEvent.pointerDown(sep, { pointerId: 1 });
    await fireEvent.pointerMove(sep, { clientX: 600 });
    await fireEvent.pointerUp(sep);
    await fireEvent.keyDown(sep, { key: "ArrowLeft" });
    expect(sep).toHaveAttribute("aria-valuenow", "58");
  });
});

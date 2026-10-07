import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import TrendChart from "./TrendChart.svelte";

const points = [
  { label: "2026-03-02", value: 600_000 },
  { label: "2026-03-09", value: null },
  { label: "2026-03-16", value: 900_000 },
  { label: "2026-03-23", value: 1_200_000 },
];
const fmt = (v: number) => `${Math.round(v / 60000)} min`;

describe("TrendChart", () => {
  it("draws gap-aware lines and reads values by keyboard", async () => {
    const { container } = render(TrendChart, { title: "Average solve time", points, format: fmt });
    expect(container.querySelectorAll("path.line")).toHaveLength(2);
    expect(container.querySelectorAll("circle.dot")).toHaveLength(3);
    expect(container.querySelector("circle.end")).not.toBeNull();
    const chart = screen.getByRole("img", { name: "Average solve time by week" });
    await fireEvent.keyDown(chart, { key: "ArrowRight" });
    expect(screen.getByRole("status")).toHaveTextContent("10 min");
    expect(screen.getByRole("status")).toHaveTextContent("Week of Mar 2");
    await fireEvent.keyDown(chart, { key: "ArrowRight" });
    expect(screen.getByRole("status")).toHaveTextContent("No solves");
    await fireEvent.keyDown(chart, { key: "ArrowLeft" });
    await fireEvent.keyDown(chart, { key: "ArrowLeft" });
    expect(screen.getByRole("status")).toHaveTextContent("Week of Mar 2");
    await fireEvent.keyDown(chart, { key: "Enter" });
    await fireEvent.keyDown(chart, { key: "Escape" });
    expect(screen.queryByRole("status")).not.toBeInTheDocument();
    await fireEvent.keyDown(chart, { key: "ArrowLeft" });
    expect(screen.getByRole("status")).toHaveTextContent("20 min");
    await fireEvent.blur(chart);
    expect(screen.queryByRole("status")).not.toBeInTheDocument();
  });

  it("snaps the crosshair to the nearest week on hover", async () => {
    render(TrendChart, { title: "Error rate", points, format: (v: number) => `${v}%`, tickFormat: (v: number) => `${v}` });
    const chart = screen.getByRole("img");
    await fireEvent.pointerMove(chart, { clientX: 100 });
    expect(screen.queryByRole("status")).not.toBeInTheDocument();
    chart.getBoundingClientRect = () => ({ left: 0, top: 0, width: 560, height: 180 }) as DOMRect;
    await fireEvent.pointerMove(chart, { clientX: 550 });
    expect(screen.getByRole("status")).toHaveTextContent("1200000%");
    await fireEvent.pointerLeave(chart);
    expect(screen.queryByRole("status")).not.toBeInTheDocument();
  });

  it("offers a table view and an empty state", async () => {
    render(TrendChart, { title: "Average solve time", points, format: fmt });
    await fireEvent.click(screen.getByText("Table"));
    expect(screen.getByRole("table")).toHaveTextContent("Mar 9—");
    await fireEvent.click(screen.getByText("Chart"));
    expect(screen.getByRole("img")).toBeInTheDocument();
  });

  it("handles no data and a single point", () => {
    render(TrendChart, { title: "Empty", points: [{ label: "2026-01-05", value: null }], format: fmt });
    expect(screen.getByText("No timed solves in this period yet.")).toBeInTheDocument();
    const { container } = render(TrendChart, { title: "One", points: [{ label: "2026-01-05", value: 5 }], format: fmt });
    expect(container.querySelectorAll("circle.dot")).toHaveLength(1);
  });

  it("ignores keys when there are no points", async () => {
    render(TrendChart, { title: "None", points: [], format: fmt });
    expect(screen.getByText("No timed solves in this period yet.")).toBeInTheDocument();
  });
});

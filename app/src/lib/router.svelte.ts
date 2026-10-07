// Minimal hash router: #/problems, #/problems/<id>, #/stats, #/settings.

export type Route =
  | { name: "problems" }
  | { name: "problem"; id: number }
  | { name: "stats" }
  | { name: "settings" };

export function parseHash(hash: string): Route {
  const parts = hash.replace(/^#\/?/, "").split("/").filter(Boolean);
  if (parts[0] === "problems" && parts[1] && /^\d+$/.test(parts[1])) {
    return { name: "problem", id: Number(parts[1]) };
  }
  if (parts[0] === "stats") return { name: "stats" };
  if (parts[0] === "settings") return { name: "settings" };
  return { name: "problems" };
}

export function hrefFor(route: Route): string {
  switch (route.name) {
    case "problem":
      return `#/problems/${route.id}`;
    case "problems":
      return "#/problems";
    default:
      return `#/${route.name}`;
  }
}

class Router {
  route = $state<Route>(parseHash(typeof location === "undefined" ? "" : location.hash));

  sync = () => {
    this.route = parseHash(location.hash);
  };

  go(route: Route) {
    location.hash = hrefFor(route);
    this.route = route;
  }
}

export const router = new Router();

if (typeof window !== "undefined") window.addEventListener("hashchange", router.sync);

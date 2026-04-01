import { writable } from "svelte/store";

export const activeRibbonTab = writable("home");
export const showBackstage = writable(false);
export const theme = writable("light");
export const zoom = writable(0.35); // pixels per mm
export const editorPan = writable({ x: 40, y: 30 });

export function toggleTheme() {
  theme.update((t) => {
    const next = t === "light" ? "dark" : "light";
    document.documentElement.setAttribute("data-theme", next);
    return next;
  });
}

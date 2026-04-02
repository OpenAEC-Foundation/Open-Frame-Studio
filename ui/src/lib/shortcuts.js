/**
 * Keyboard shortcuts for Open Frame Studio.
 */
import { get } from "svelte/store";
import {
  currentKozijn,
  selectedCellIndex,
  selectedMember,
  createKozijn,
  addColumn,
  addRow,
  updateCellType,
} from "../stores/kozijn.js";
import { zoom } from "../stores/ui.js";

const PANEL_TYPE_KEYS = {
  "1": "fixed_glass",
  "2": "turn_tilt",
  "3": "turn",
  "4": "tilt",
  "5": "sliding",
  "6": "door",
  "7": "panel",
  "8": "ventilation",
};

let registered = false;

export function registerShortcuts({ onDuplicate, onDelete } = {}) {
  if (registered) return;
  registered = true;

  document.addEventListener("keydown", (e) => {
    // Ignore when typing in inputs
    const tag = e.target.tagName;
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return;

    const k = get(currentKozijn);

    // Escape: deselect all
    if (e.key === "Escape") {
      selectedCellIndex.set(null);
      selectedMember.set(null);
      return;
    }

    // Tab: next cell
    if (e.key === "Tab" && k) {
      e.preventDefault();
      const current = get(selectedCellIndex);
      const total = k.cells.length;
      if (current === null) {
        selectedCellIndex.set(0);
      } else {
        selectedCellIndex.set((current + (e.shiftKey ? total - 1 : 1)) % total);
      }
      selectedMember.set(null);
      return;
    }

    // 1-8: quick panel type
    if (PANEL_TYPE_KEYS[e.key] && get(selectedCellIndex) !== null) {
      const panelType = PANEL_TYPE_KEYS[e.key];
      const dir = panelType === "turn_tilt" ? "left" : panelType === "door" ? "inward" : null;
      updateCellType(get(selectedCellIndex), panelType, dir);
      return;
    }

    // Ctrl+D: duplicate
    if (e.key === "d" && (e.ctrlKey || e.metaKey) && k) {
      e.preventDefault();
      if (onDuplicate) onDuplicate();
      return;
    }

    // Delete: reset cell or deselect member
    if (e.key === "Delete") {
      const cellIdx = get(selectedCellIndex);
      if (cellIdx !== null) {
        updateCellType(cellIdx, "fixed_glass", null);
      }
      const member = get(selectedMember);
      if (member) {
        selectedMember.set(null);
      }
      return;
    }

    // Ctrl+Shift+C: add column
    if (e.key === "C" && e.ctrlKey && e.shiftKey && k) {
      e.preventDefault();
      const innerW = k.frame.outerWidth - 2 * k.frame.frameWidth;
      addColumn(innerW / 2);
      return;
    }

    // Ctrl+Shift+R: add row
    if (e.key === "R" && e.ctrlKey && e.shiftKey && k) {
      e.preventDefault();
      const innerH = k.frame.outerHeight - 2 * k.frame.frameWidth;
      addRow(innerH / 2);
      return;
    }

    // +/- or =/- : zoom
    if (e.key === "=" || e.key === "+") {
      zoom.update((z) => Math.min(2.0, z + 0.1));
      return;
    }
    if (e.key === "-") {
      zoom.update((z) => Math.max(0.05, z - 0.1));
      return;
    }
  });
}

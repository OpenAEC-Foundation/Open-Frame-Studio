# Open Frame Studio - Session Summary

**Date:** 2026-04-02
**Commit:** 5244736
**Files changed:** 101 (+9,166 / -2,317 lines)

---

## 1. Python Sidecar Removal

Rewrote all Python export/import code in native Rust, eliminating the Python runtime dependency for Windows installers.

### Exports (7 Rust modules in `ofs-core/src/export/`)
| Module | Replaces | Crate used |
|---|---|---|
| `gltf.rs` | `python/ofs_gltf/generator.py` | None (manual binary) |
| `dxf.rs` | `python/ofs_dxf/generator.py` | None (manual text) |
| `ifc.rs` | `python/ofs_ifc/generator.py` + `ils_properties.py` | None (manual IFC-SPF) |
| `pdf.rs` | `python/ofs_pdf/kozijnstaat.py` + `werkplaats_tekening.py` + production PDF | `printpdf 0.9` |
| `xlsx.rs` | `python/ofs_production/xlsx_generator.py` + kozijnstaat XLSX | `rust_xlsxwriter 0.94` |
| `csv_production.rs` | `python/ofs_production/csv_generator.py` | None (std) |

### Imports (2 Rust modules in `ofs-core/src/import/`)
| Module | Replaces | Crate used |
|---|---|---|
| `dxf_profile.rs` | `python/ofs_import/dxf_profile_parser.py` | None (manual DXF text parser) |
| `catalog.rs` | `python/ofs_import/catalog_parser.py` | `calamine 0.34` (XLSX reading) |

### Tauri commands updated
All 6 export commands + 2 import commands rewritten to call Rust directly. `python_command()` removed from `state.rs`. Tokio `process` feature removed then restored for Blender TCP only.

---

## 2. Internationalization (i18n)

### Setup
- Installed `svelte-i18n`
- Created `ui/src/lib/i18n.js` initialization
- 3 locale files: `nl.json` (Dutch), `en.json` (English), `de.json` (German)
- ~400 translation keys covering every UI string

### Components updated (20+ files)
Every hardcoded Dutch string replaced with `$_('key')` calls across all shell components, panels, editors, project views, and dialogs.

### Language switcher
Settings dialog allows language selection. Choice persisted to config file.

---

## 3. Theme System

### 7 themes (matching Open PDF Studio)
1. Default (warm dark, amber accent)
2. Light
3. Dark (navy)
4. Blue (ocean)
5. Amber Navy
6. Warm Ember (warm brown)
7. High Contrast (black/white/yellow)

### Implementation
- All CSS variables defined in `tokens.css` per theme
- Replaced all hardcoded colors with CSS variables
- Theme persisted to config file via Rust command
- Selectable from Settings dialog and View ribbon tab

---

## 4. Svelte 5 + Vite 6 Migration

### Package updates
| Package | Before | After |
|---|---|---|
| `svelte` | 4.2.20 | 5.55.1 |
| `vite` | 5.4.21 | 6.4.1 |
| `@sveltejs/vite-plugin-svelte` | 3.1.2 | 5.1.1 |

### Syntax migration (25 components)
- `on:click` → `onclick` (100+ instances)
- `on:change/input/mousedown/wheel/keydown` → native event syntax
- `on:click|stopPropagation` → inline `e.stopPropagation()`
- `<svelte:window on:*>` → `<svelte:window on*>`
- `export let` → `$props()` (12 components)
- `createEventDispatcher` → callback props (4 components)
- `$:` reactive → `$derived()` / `$effect()` (25 conversions in runes-mode files)
- `new App()` → `mount(App, {})` in entry point
- Removed all `svelte-ignore a11y*` comments
- Fixed button-inside-button in ProjectOverview

---

## 5. Rust Package Updates

| Crate | Before | After |
|---|---|---|
| `printpdf` | 0.7.0 | 0.9.1 (full API rewrite) |
| `rust_xlsxwriter` | 0.80.0 | 0.94.0 |
| `calamine` | 0.26.1 | 0.34.0 |
| `three` (npm) | CDN 0.160 | bundled 0.183.2 |

---

## 6. UI Improvements

### Title bar
- App icon on the left
- Settings gear button
- Undo/redo buttons (same icons as Open PDF Studio)
- Dirty indicator (`*` prefix when unsaved)
- Maximize button icon toggles (single rect / overlapping rects)
- Linux: window controls hidden (WM provides them)
- Webview2 resize dimension overlay suppressed

### File menu (AppMenu)
- Moved from title bar to ribbon (amber File tab before Home)
- Icons on all menu items
- Keyboard shortcuts shown (Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+Shift+S)
- Section headers (Project, Help)
- Help section: GitHub, Report Issue, Documentation (opens in browser via shell plugin)
- Welcome content with app branding and quick-start cards
- Opens below title bar, doesn't cover it

### Settings dialog
- Windows-style dialog form (title bar, close button, footer)
- Draggable by title bar
- Not dismissable by clicking outside
- Sidebar with General and About categories
- Language and theme dropdowns
- About page with app icon, version, org, license

### Panels
- Resizable left and right panels with drag handles
- Collapsible with header bar and chevron button
- Collapsed state shows vertical rotated title tab (VS Code style)
- Panel widths and open/closed state persisted to settings
- Properties panel redesigned (compact Windows property grid style)
- Project list redesigned (flat rows, active left accent bar, hover-reveal delete)

### Toast notifications
- `stores/toast.js` + `Toast.svelte` component
- Success (green), error (red), warning (amber), info (blue)
- Auto-dismiss with configurable duration
- Slide-in animation, click to dismiss
- Replaces all 20 `alert()` calls

---

## 7. Code Architecture

### New modules
| File | Purpose |
|---|---|
| `lib/api.js` | Centralized invoke wrapper with toast error handling |
| `lib/export.js` | All export/import service functions |
| `lib/labels.js` | Shared i18n-aware label lookups (panel, member, gasket types) |
| `lib/project-actions.js` | File operations (new, open, save, save-as) with unsaved changes protection |
| `lib/settings.js` | Persistent settings via Rust config file |
| `lib/web-dialogs.js` | Browser file open/save dialogs |
| `lib/i18n.js` | i18n initialization |
| `stores/toast.js` | Toast notification store |
| `src-tauri/src/commands/settings.rs` | Load/save settings JSON to app config directory |
| `src-tauri/src/commands/profiles.rs` | Load profile library from resource directory |

### Restructured
- `vliesgevel.rs` + 4 files → `vliesgevel/` directory with `mod.rs`
- `Backstage.svelte` → `AppMenu.svelte` (renamed)
- `tauri.js` mock layer: 468 lines → 130 lines (auto-stub pattern)

### Keyboard shortcuts
- `Ctrl+N` (new), `Ctrl+O` (open), `Ctrl+S` (save), `Ctrl+Shift+S` (save-as)
- `Ctrl+Z` / `Ctrl+Y` (undo/redo)
- `Ctrl+D` (duplicate), `Delete` (reset cell)
- `Tab` / `Shift+Tab` (cycle cells)
- `1-8` (quick panel type)
- `+/-` (zoom)
- All browser defaults blocked (Ctrl+P, F5, Ctrl+F, etc.)
- Right-click context menu disabled

### Unsaved changes protection
- `isDirty` flag in project store
- `*` indicator in title bar
- Save prompt on: close, new, open
- Uses Tauri native `ask()` dialog with Save/Don't Save

---

## 8. Security & Production

### CSP
- Set proper Content-Security-Policy: `self` only
- No external CDN dependencies

### Offline
- Three.js bundled via npm (was CDN)
- Fonts self-hosted in `/fonts/` (was Google Fonts)
- Zero internet required for desktop app

### Profile loading
- New Rust command `load_profile_library` reads from resource directory
- Profiles bundled via `bundle.resources` in `tauri.conf.json`
- No more relative path `readTextFile("../profiles/...")`

### Window controls
- Tauri capabilities: minimize, toggle-maximize, is-maximized, close, destroy
- Shell plugin for opening external URLs

---

## 9. WebAssembly (WASM) Support

### `ofs-wasm` crate
- Thin `wasm-bindgen` wrapper over `ofs-core`
- Exposes: project CRUD, kozijn CRUD, geometry, production, thermal
- `default-features = false` (no PDF/XLSX/calamine in WASM)
- State managed via `Mutex<Option<Project>>`

### Frontend integration
- `tauri.js` detects Tauri vs browser, loads WASM in web mode
- Dynamic import with `@vite-ignore` to prevent build-time resolution
- Falls back to sensible defaults if WASM not available

### Build commands
- `npm run build:wasm` — compile WASM via wasm-pack
- `npm run build:web` — WASM + Vite build
- `npm run dev:web` — WASM + Vite dev server

---

## 10. GitHub Workflows

### `ci.yml` — CI (push/PR to main)
- Ubuntu, Windows, macOS
- macOS universal binary (Intel + Apple Silicon)
- Rust cache

### `release.yml` — Build & Release (v* tags)
- 4 targets: Linux, macOS, Windows (system + user installer)
- Authenticode signing via Azure Trusted Signing
- NSIS installers

### `live.yml` — Web deploy (push to main)
- Uses shared OpenAEC deploy workflow
- Builds WASM + frontend
- Deploys to `open-frame-studio.open-aec.com` via SSH

---

## 11. Cursor & UX
- Default cursor everywhere (no pointer, grab, move, resize, not-allowed)
- Zoom pivots around mouse cursor position
- `user-select: none` on title bar and controls

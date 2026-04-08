# Klikbare Onderdelen + Profielsysteem + Leveranciersimport

## Context

De gebruiker wil elk kozijnonderdeel (stijl, dorpel, bovendorpel, tussenstijl, tussendorpel) kunnen aanklikken in de 2D editor en dan per onderdeel het profiel, de sponningen en afmetingen wijzigen. Daarnaast moeten leveranciersprofielen en -onderdelen (beslag, dorpels, glas) importeerbaar zijn via DXF (tekeningen) en JSON/Excel (technische data).

## Huidige staat

- Frame members zijn `<rect>` elementen in KozijnCanvas.svelte maar **niet klikbaar**
- Dividers zijn `<rect>` elementen maar **niet klikbaar**
- Alleen cellen zijn klikbaar (`handleCellClick`)
- `selectedCellIndex` store bestaat, maar geen `selectedMember` store
- PropertiesPanel toont alleen kozijn-eigenschappen en cel-eigenschappen
- ProfileSelector bestaat maar werkt alleen voor frame en dorpel globaal
- ProfileRef heeft alleen `id` en `name`, geen sponning/breedte/diepte info
- ProfileDefinition heeft `width`, `depth`, `sightline`, `glazingRebate`, `crossSection`, `ufValue`

## Plan

### Fase 1: Klikbare onderdelen + member-selectie

**Store:**
- `ui/src/stores/kozijn.js` — toevoegen: `selectedMember` writable store
  - Type: `null | { type: "frame_top"|"frame_bottom"|"frame_left"|"frame_right"|"divider_v_0"|"divider_h_0"|..., index: number }`
  - Bij cel-klik: `selectedMember.set(null)`
  - Bij member-klik: `selectedCellIndex.set(null)`

**KozijnCanvas.svelte:**
- Frame rects klikbaar maken met `on:click` handlers
  - `frameRects[0]` = bovendorpel → `{type: "frame_top"}`
  - `frameRects[1]` = onderdorpel → `{type: "frame_bottom"}`
  - `frameRects[2]` = stijl links → `{type: "frame_left"}`
  - `frameRects[3]` = stijl rechts → `{type: "frame_right"}`
- Divider rects klikbaar maken:
  - `v_dividers[i]` → `{type: "divider_v", index: i}`
  - `h_dividers[i]` → `{type: "divider_h", index: i}`
- Highlight geselecteerd member met amber stroke (zelfde stijl als geselecteerde cel)
- Cursor: pointer op hover

**PropertiesPanel.svelte — nieuw "Onderdeel" sectie:**
- Als `$selectedMember !== null`:
  - Toon naam (bijv. "Stijl links", "Tussenstijl 1", "Onderdorpel")
  - ProfileSelector met filter op `applicableAs` (frame/sill/divider)
  - Invoervelden: breedte (mm), diepte (mm)
  - Sponning-info: breedte, diepte, positie (als beschikbaar uit profiel)
  - Bij profiel-wissel: update het specifieke onderdeel via Tauri command

### Fase 2: Per-member profiel opslag in datamodel

**`ofs-core/src/kozijn.rs` — Frame struct uitbreiden:**
```rust
pub struct Frame {
    // ... bestaande velden ...
    // Per-member profiel overrides (None = gebruik default frame profile)
    #[serde(default)]
    pub top_profile: Option<ProfileRef>,    // bovendorpel
    #[serde(default)]
    pub bottom_profile: Option<ProfileRef>, // onderdorpel
    #[serde(default)]
    pub left_profile: Option<ProfileRef>,   // stijl links
    #[serde(default)]
    pub right_profile: Option<ProfileRef>,  // stijl rechts
}
```
- GridDivision heeft al `divider_profile: Option<ProfileRef>` — dat is goed
- Backward compatible via `#[serde(default)]`

**Tauri commands:**
- `update_member_profile(id, member_type, profile_id, profile_name, width, depth)` — één command voor alle member types
- Intern: match op member_type string, update het juiste veld

### Fase 3: Profielen met sponning-informatie

**`ofs-core/src/profile.rs` — ProfileDefinition uitbreiden:**
```rust
pub struct ProfileDefinition {
    // ... bestaande velden ...
    #[serde(default)]
    pub sponning: Option<SponningInfo>,
}

pub struct SponningInfo {
    pub width: f64,        // sponningbreedte (mm)
    pub depth: f64,        // sponningdiepte (mm)
    pub position: String,  // "binnen" | "buiten" | "midden"
}
```

**Profielbibliotheek updaten:**
- `profiles/wood/*.json` — sponning-info toevoegen aan bestaande profielen
- `profiles/aluminum/*.json` — idem
- `profiles/pvc/*.json` — idem

### Fase 4: Leveranciersimport

**Nieuw: `ui/src/components/panels/ImportDialog.svelte`**
- Modal dialog met 2 tabs: "DXF Import" en "Catalogus Import"
- DXF tab: bestand kiezen → Python parseert DXF → toont preview → voeg toe aan project
- Catalogus tab: JSON/Excel bestand kiezen → parseert → toont lijst → selecteer en importeer

**Nieuw: `python/ofs_import/dxf_profile_parser.py`**
- Leest een DXF-bestand met een profieldoorsnede
- Extraheert de outer contour als `crossSection` polygon
- Detecteert sponningen (rechthoekige inkepingen) automatisch
- Retourneert een `ProfileDefinition` JSON

**Nieuw: `python/ofs_import/catalog_parser.py`**
- Leest JSON of Excel bestand met profielcatalogus
- Verwacht kolommen: naam, breedte, diepte, sightline, glazingRebate, ufValue, materiaal, sponningBreedte, sponningDiepte
- Retourneert een lijst `ProfileDefinition` objecten
- Ondersteunt ook beslag/dorpel/glas imports met hun eigen kolom-schema's

**Nieuw: `python/ofs_import/supplier_templates/`**
- `reynaers.json` — kolom-mapping voor Reynaers Excel export
- `schuco.json` — kolom-mapping voor Schuco data
- `gealan.json` — kolom-mapping voor Gealan data
- Generieke mapping als fallback

**Tauri commands:**
- `import_dxf_profile(file_path)` → roept Python aan, retourneert ProfileDefinition
- `import_catalog(file_path, format)` → roept Python aan, retourneert Vec<ProfileDefinition>
- `import_supplier_parts(file_path, part_type)` → voor beslag, dorpels, glas

**Ribbon:**
- Nieuwe groep "Importeren" in Home of IFC/Export tab
- Knop "Profiel importeren (DXF)"
- Knop "Catalogus importeren"

### Fase 5: Maatvoering en visuele fixes

- `ofs-core/src/geometry.rs` — `dim_offset` van 30 → 15mm
- `KozijnCanvas.svelte` — font-size van 14 → 9, stroke-width van 1 → 0.5
- `TitleBar.svelte` — afsluiten fixen met tauri-plugin-process
- `src-tauri/Cargo.toml` — tauri-plugin-process toevoegen
- `ui/package.json` — @tauri-apps/plugin-process toevoegen

## Volgorde
```
Fase 5 (fixes) — direct doen, zijn al half klaar
  ↓
Fase 1 (klikbare onderdelen) — UI foundation
  ↓
Fase 2 (per-member profiel opslag) — data foundation
  ↓
Fase 3 (sponning-info) — profieldata verrijken
  ↓
Fase 4 (leveranciersimport) — DXF parser + catalogus import
```

## Bestanden

### Nieuw (5):
- `ui/src/components/panels/ImportDialog.svelte`
- `python/ofs_import/__init__.py`
- `python/ofs_import/dxf_profile_parser.py`
- `python/ofs_import/catalog_parser.py`
- `python/ofs_import/supplier_templates/` (mapping bestanden)

### Wijzig (12):
- `ofs-core/src/kozijn.rs` — Frame met per-member profiles
- `ofs-core/src/profile.rs` — SponningInfo
- `ofs-core/src/geometry.rs` — dim_offset fix
- `src-tauri/src/commands/kozijn.rs` — update_member_profile command
- `src-tauri/src/main.rs` — registreer commands + process plugin
- `src-tauri/src/state.rs` — (al aangepast met python_command)
- `src-tauri/Cargo.toml` — tauri-plugin-process
- `ui/src/stores/kozijn.js` — selectedMember store + functies
- `ui/src/components/editor/KozijnCanvas.svelte` — klikbare members + dim fix
- `ui/src/components/panels/PropertiesPanel.svelte` — member-sectie
- `ui/src/components/shell/TitleBar.svelte` — close fix
- `ui/src/components/shell/Ribbon.svelte` — import knoppen
- `profiles/*.json` — sponning-info toevoegen

## Verificatie
- Klik op stijl links → Properties toont "Stijl links" met profiel/maten
- Klik op tussenstijl → Properties toont profiel met sponning-info
- Wissel profiel → tekening update direct met nieuwe maten
- Import DXF → profiel verschijnt in bibliotheek met doorsnede-preview
- Import catalogus → meerdere profielen verschijnen in selector
- Afsluiten werkt
- Maatvoering is proportioneel en leesbaar

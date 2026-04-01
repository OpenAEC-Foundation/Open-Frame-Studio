/**
 * Profielbibliotheek store.
 *
 * Laadt alle profieldefinities uit de /profiles directory en
 * biedt ze aan als een doorzoekbare lijst.
 */
import { writable, derived } from "svelte/store";

// Alle profielen, gegroepeerd per categorie
export const profileCategories = writable([]);

// Platte lijst van alle profielen
export const allProfiles = derived(profileCategories, ($cats) =>
  $cats.flatMap((cat) =>
    cat.profiles.map((p) => ({ ...p, category: cat.id, categoryLabel: cat.label }))
  )
);

// Zoekfilter
export const profileFilter = writable("");

// Gefilterde profielen
export const filteredProfiles = derived(
  [allProfiles, profileFilter],
  ([$all, $filter]) => {
    if (!$filter) return $all;
    const q = $filter.toLowerCase();
    return $all.filter(
      (p) =>
        p.name.toLowerCase().includes(q) ||
        (p.manufacturer || "").toLowerCase().includes(q) ||
        (p.series || "").toLowerCase().includes(q) ||
        p.material.toLowerCase().includes(q)
    );
  }
);

/**
 * Laad de profielbibliotheek.
 * In de browser mock laden we inline data; in Tauri lezen we JSON bestanden.
 */
export async function loadProfiles() {
  try {
    // Embedded profile data for browser preview
    const categories = await fetchProfileData();
    profileCategories.set(categories);
  } catch (e) {
    console.error("Profielen laden mislukt:", e);
  }
}

async function fetchProfileData() {
  // In browser mode, use fetch to load from public assets
  // In Tauri mode, use fs plugin to read from resources
  const isTauri = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;

  if (isTauri) {
    // In Tauri: read from resource directory
    try {
      const { readTextFile } = await import("@tauri-apps/plugin-fs");
      const indexJson = await readTextFile("../profiles/index.json");
      const index = JSON.parse(indexJson);

      const categories = [];
      for (const cat of index.categories) {
        const profiles = [];
        for (const file of cat.files) {
          try {
            const text = await readTextFile(`../profiles/${file}`);
            const data = JSON.parse(text);
            profiles.push(...data);
          } catch (e) {
            console.warn(`Profiel bestand ${file} niet geladen:`, e);
          }
        }
        categories.push({ ...cat, profiles });
      }
      return categories;
    } catch (e) {
      console.warn("Profielen laden via fs mislukt, gebruik fallback:", e);
    }
  }

  // Browser fallback: embedded minimal profile set
  return getEmbeddedProfiles();
}

function getEmbeddedProfiles() {
  return [
    {
      id: "wood",
      label: "Hout",
      profiles: [
        { id: "wood-meranti-67x114", name: "Meranti 67x114mm", manufacturer: "Generiek", series: "Standaard", material: "wood", materialSubtype: "meranti", width: 67, depth: 114, sightline: 54, glazingRebate: 24, ufValue: 1.8, applicableAs: ["frame", "sash", "divider"] },
        { id: "wood-meranti-67x150", name: "Meranti 67x150mm (dorpel)", manufacturer: "Generiek", series: "Standaard", material: "wood", materialSubtype: "meranti", width: 67, depth: 150, sightline: 54, glazingRebate: 24, ufValue: 1.8, applicableAs: ["sill"] },
        { id: "wood-accoya-67x114", name: "Accoya 67x114mm", manufacturer: "Generiek", series: "Accoya", material: "wood", materialSubtype: "accoya", width: 67, depth: 114, sightline: 54, glazingRebate: 24, ufValue: 1.5, applicableAs: ["frame", "sash", "divider"] },
        { id: "hebo-67x114", name: "Hebo Kozijn 67x114mm", manufacturer: "Hebo Kozijnen", series: "Classic", material: "wood", materialSubtype: "meranti", width: 67, depth: 114, sightline: 54, glazingRebate: 24, ufValue: 1.8, applicableAs: ["frame", "sash"] },
        { id: "hebo-67x130", name: "Hebo Kozijn 67x130mm (zwaar)", manufacturer: "Hebo Kozijnen", series: "Classic", material: "wood", materialSubtype: "meranti", width: 67, depth: 130, sightline: 54, glazingRebate: 30, ufValue: 1.7, applicableAs: ["frame", "sash"] },
        { id: "goemaat-67x114", name: "Goemaat Standaard 67x114mm", manufacturer: "Goemaat Kozijnen", series: "Standaard", material: "wood", materialSubtype: "meranti", width: 67, depth: 114, sightline: 54, glazingRebate: 24, ufValue: 1.8, applicableAs: ["frame", "sash"] },
        { id: "goemaat-67x130", name: "Goemaat Zwaar 67x130mm", manufacturer: "Goemaat Kozijnen", series: "Zwaar", material: "wood", materialSubtype: "meranti", width: 67, depth: 130, sightline: 54, glazingRebate: 30, ufValue: 1.7, applicableAs: ["frame", "sash"] },
        { id: "weekamp-67x114", name: "WeekampGroep Standaard 67x114mm", manufacturer: "WeekampGroep", series: "Standaard", material: "wood", materialSubtype: "meranti", width: 67, depth: 114, sightline: 54, glazingRebate: 24, ufValue: 1.8, applicableAs: ["frame", "door_frame"] },
        { id: "raamwerk-67x114", name: "Raamwerk Standaard 67x114mm", manufacturer: "Het Raamwerk", series: "Standaard", material: "wood", materialSubtype: "meranti", width: 67, depth: 114, sightline: 54, glazingRebate: 24, ufValue: 1.8, applicableAs: ["frame", "sash"] },
      ],
    },
    {
      id: "aluminum",
      label: "Aluminium",
      profiles: [
        { id: "reynaers-cs77-frame", name: "Reynaers CS 77", manufacturer: "Reynaers Aluminium", series: "CS 77", material: "aluminum", materialSubtype: "thermisch_onderbroken", width: 77, depth: 77, sightline: 54, glazingRebate: 28, ufValue: 1.6, applicableAs: ["frame", "sash"] },
        { id: "reynaers-cs86-frame", name: "Reynaers CS 86-HI", manufacturer: "Reynaers Aluminium", series: "CS 86-HI", material: "aluminum", materialSubtype: "high_insulation", width: 86, depth: 86, sightline: 60, glazingRebate: 34, ufValue: 1.1, applicableAs: ["frame", "sash"] },
        { id: "schuco-aws70-frame", name: "Schuco AWS 70.HI", manufacturer: "Schuco", series: "AWS 70.HI", material: "aluminum", materialSubtype: "thermisch_onderbroken", width: 70, depth: 70, sightline: 50, glazingRebate: 26, ufValue: 1.6, applicableAs: ["frame", "sash"] },
        { id: "schuco-aws75-frame", name: "Schuco AWS 75.SI+", manufacturer: "Schuco", series: "AWS 75.SI+", material: "aluminum", materialSubtype: "super_insulated", width: 75, depth: 75, sightline: 53, glazingRebate: 32, ufValue: 1.0, applicableAs: ["frame", "sash"] },
      ],
    },
    {
      id: "pvc",
      label: "Kunststof (PVC)",
      profiles: [
        { id: "gealan-s9000-frame", name: "Gealan S 9000", manufacturer: "Gealan", series: "S 9000", material: "pvc", materialSubtype: "6_kamer", width: 83, depth: 83, sightline: 62, glazingRebate: 28, ufValue: 1.0, applicableAs: ["frame", "sash"] },
        { id: "gealan-kubus-frame", name: "Gealan-KUBUS", manufacturer: "Gealan", series: "KUBUS", material: "pvc", materialSubtype: "7_kamer", width: 85, depth: 85, sightline: 0, glazingRebate: 30, ufValue: 0.92, applicableAs: ["frame", "sash"] },
        { id: "veka-softline82-frame", name: "VEKA Softline 82", manufacturer: "VEKA", series: "Softline 82", material: "pvc", materialSubtype: "7_kamer", width: 82, depth: 82, sightline: 61, glazingRebate: 28, ufValue: 0.95, applicableAs: ["frame", "sash"] },
        { id: "veka-softline70-frame", name: "VEKA Softline 70", manufacturer: "VEKA", series: "Softline 70", material: "pvc", materialSubtype: "5_kamer", width: 70, depth: 70, sightline: 53, glazingRebate: 24, ufValue: 1.3, applicableAs: ["frame", "sash"] },
      ],
    },
    {
      id: "wood-aluminum",
      label: "Hout-Aluminium",
      profiles: [
        { id: "hout-alu-67x114", name: "Hout-Aluminium 67x114mm", manufacturer: "Generiek", series: "Combi", material: "wood_aluminum", materialSubtype: "meranti_alu", width: 67, depth: 114, sightline: 54, glazingRebate: 24, ufValue: 1.5, applicableAs: ["frame", "sash"] },
        { id: "hout-alu-67x130", name: "Hout-Aluminium 67x130mm (zwaar)", manufacturer: "Generiek", series: "Combi", material: "wood_aluminum", materialSubtype: "meranti_alu", width: 67, depth: 130, sightline: 54, glazingRebate: 30, ufValue: 1.4, applicableAs: ["frame", "sash"] },
      ],
    },
  ];
}

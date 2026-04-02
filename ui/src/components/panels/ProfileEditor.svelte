<script>
  import { createEventDispatcher } from "svelte";
  import { invoke } from "../../lib/tauri.js";
  import { refreshProject } from "../../stores/project.js";
  import ProfilePreview from "./ProfilePreview.svelte";

  const dispatch = createEventDispatcher();

  export let visible = false;
  export let editProfile = null; // existing profile to edit, or null for new

  // Profile parameters
  let name = "Nieuw profiel";
  let material = "wood";
  let width = 67;
  let depth = 114;
  let sponningWidth = 12;
  let sponningDepth = 17;
  let sponningPos = "buiten";
  let glaslatWidth = 15;
  let noseDepth = 5;
  let ufValue = 1.8;
  let applicableAs = { frame: true, sash: true, divider: true, sill: false };

  // Initialize from existing profile
  $: if (editProfile && visible) {
    name = editProfile.name || "Profiel bewerken";
    material = editProfile.material || "wood";
    width = editProfile.width || 67;
    depth = editProfile.depth || 114;
    ufValue = editProfile.ufValue || 1.8;
    if (editProfile.sponning) {
      sponningWidth = editProfile.sponning.width || 12;
      sponningDepth = editProfile.sponning.depth || 17;
      sponningPos = editProfile.sponning.position || "buiten";
    }
    const aa = editProfile.applicableAs || [];
    applicableAs = {
      frame: aa.includes("frame"),
      sash: aa.includes("sash"),
      divider: aa.includes("divider"),
      sill: aa.includes("sill"),
    };
  }

  // Generate cross-section polygon from parameters
  function generateCrossSection() {
    const w = width, d = depth, sw = sponningWidth, sd = sponningDepth, nd = noseDepth;
    if (sponningPos === "buiten") {
      return [
        [0, 0], [w, 0], [w, d - sd],
        [w - sw, d - sd], [w - sw, d - nd], [w - sw, d],
        [sw, d], [sw, d - nd], [sw, d - sd],
        [0, d - sd],
      ];
    } else if (sponningPos === "binnen") {
      return [
        [sw, 0], [w - sw, 0], [w - sw, sd],
        [w, sd], [w, d], [0, d], [0, sd], [sw, sd],
      ];
    } else {
      const mid = (d - sd) / 2;
      return [
        [0, 0], [w, 0], [w, mid],
        [w - sw, mid], [w - sw, mid + sd],
        [w, mid + sd], [w, d], [0, d],
        [0, mid + sd], [sw, mid + sd], [sw, mid], [0, mid],
      ];
    }
  }

  // Computed values
  $: sightline = width - sponningWidth;
  $: glazingRebate = sponningDepth;

  const MATERIALS = [
    { value: "wood", label: "Hout" },
    { value: "aluminum", label: "Aluminium" },
    { value: "pvc", label: "Kunststof (PVC)" },
    { value: "wood_aluminum", label: "Hout-Aluminium" },
  ];

  async function handleSave() {
    const aa = [];
    if (applicableAs.frame) aa.push("frame");
    if (applicableAs.sash) aa.push("sash");
    if (applicableAs.divider) aa.push("divider");
    if (applicableAs.sill) aa.push("sill");

    const profileId = editProfile?.id || `custom-${name.toLowerCase().replace(/\s+/g, "-")}-${Date.now()}`;
    const crossSection = generateCrossSection();

    const profile = {
      id: profileId,
      name,
      material,
      materialSubtype: null,
      width,
      depth,
      sightline,
      glazingRebate,
      crossSection,
      ufValue,
      applicableAs: aa,
      sponning: { width: sponningWidth, depth: sponningDepth, position: sponningPos },
    };

    try {
      await invoke("add_custom_profile", { profileJson: JSON.stringify(profile) });
      await refreshProject();
      dispatch("saved", profile);
      visible = false;
    } catch (e) {
      alert("Profiel opslaan mislukt: " + e);
    }
  }

  function handleClose() {
    visible = false;
    dispatch("close");
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="overlay" on:click={handleClose}>
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div class="editor-modal" on:click|stopPropagation>
      <div class="editor-header">
        <h2>Profieleditor</h2>
        <button class="close-btn" on:click={handleClose}>
          <svg width="16" height="16" viewBox="0 0 16 16"><path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
        </button>
      </div>

      <div class="editor-body">
        <div class="preview-col">
          <ProfilePreview
            {width} {depth}
            {sponningWidth} {sponningDepth} {sponningPos}
            {noseDepth}
            svgWidth={220} svgHeight={300}
          />
          <div class="computed-values">
            <span>Zichtlijn: {sightline}mm</span>
            <span>Glasfalz: {glazingRebate}mm</span>
          </div>
        </div>

        <div class="params-col">
          <div class="param-section">
            <div class="field">
              <label>Naam</label>
              <input type="text" bind:value={name} />
            </div>
            <div class="field">
              <label>Materiaal</label>
              <select bind:value={material}>
                {#each MATERIALS as mat}
                  <option value={mat.value}>{mat.label}</option>
                {/each}
              </select>
            </div>
          </div>

          <div class="param-section">
            <h4>Afmetingen</h4>
            <div class="field-row">
              <div class="field">
                <label>Breedte (mm)</label>
                <input type="number" bind:value={width} min="30" max="150" step="1" />
              </div>
              <div class="field">
                <label>Diepte (mm)</label>
                <input type="number" bind:value={depth} min="50" max="250" step="1" />
              </div>
            </div>
          </div>

          <div class="param-section">
            <h4>Sponning</h4>
            <div class="field-row">
              <div class="field">
                <label>Breedte (mm)</label>
                <input type="number" bind:value={sponningWidth} min="5" max="40" step="1" />
              </div>
              <div class="field">
                <label>Diepte (mm)</label>
                <input type="number" bind:value={sponningDepth} min="10" max="50" step="1" />
              </div>
            </div>
            <div class="field">
              <label>Positie</label>
              <select bind:value={sponningPos}>
                <option value="buiten">Buiten</option>
                <option value="binnen">Binnen</option>
                <option value="midden">Midden</option>
              </select>
            </div>
          </div>

          <div class="param-section">
            <h4>Details</h4>
            <div class="field-row">
              <div class="field">
                <label>Glaslat (mm)</label>
                <input type="number" bind:value={glaslatWidth} min="8" max="40" step="1" />
              </div>
              <div class="field">
                <label>Neusdiepte (mm)</label>
                <input type="number" bind:value={noseDepth} min="0" max="20" step="1" />
              </div>
            </div>
            <div class="field">
              <label>Uf-waarde (W/m2K)</label>
              <input type="number" bind:value={ufValue} min="0.5" max="5.0" step="0.1" />
            </div>
          </div>

          <div class="param-section">
            <h4>Toepassing</h4>
            <div class="checkbox-row">
              <label class="checkbox"><input type="checkbox" bind:checked={applicableAs.frame} /> Kozijn</label>
              <label class="checkbox"><input type="checkbox" bind:checked={applicableAs.sash} /> Draairaam</label>
              <label class="checkbox"><input type="checkbox" bind:checked={applicableAs.divider} /> Verdeler</label>
              <label class="checkbox"><input type="checkbox" bind:checked={applicableAs.sill} /> Dorpel</label>
            </div>
          </div>

          <div class="actions">
            <button class="btn-primary" on:click={handleSave}>Opslaan</button>
            <button class="btn-secondary" on:click={handleClose}>Annuleren</button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .editor-modal {
    background: var(--bg-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    width: 580px;
    max-height: 90vh;
    overflow-y: auto;
  }

  .editor-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--sp-4) var(--sp-6);
    border-bottom: var(--border-default);
  }

  .editor-header h2 {
    font-family: var(--font-heading);
    font-size: 16px;
    font-weight: 700;
    color: var(--amber);
    margin: 0;
  }

  .close-btn {
    color: var(--text-muted);
    padding: 4px;
    border-radius: var(--radius-sm);
  }
  .close-btn:hover { color: var(--text-primary); background: var(--bg-surface-alt); }

  .editor-body {
    display: flex;
    gap: var(--sp-6);
    padding: var(--sp-6);
  }

  .preview-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--sp-3);
    min-width: 220px;
    padding: var(--sp-3);
    background: var(--bg-surface-alt);
    border-radius: var(--radius-md);
    border: var(--border-default);
  }

  .computed-values {
    display: flex;
    gap: var(--sp-3);
    font-size: 10px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .params-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: var(--sp-4);
  }

  .param-section h4 {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--amber);
    margin-bottom: var(--sp-2);
    padding-bottom: var(--sp-1);
    border-bottom: var(--border-default);
  }

  .field {
    margin-bottom: var(--sp-2);
  }

  .field label {
    display: block;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    margin-bottom: 2px;
  }

  .field input, .field select {
    width: 100%;
    padding: var(--sp-1) var(--sp-2);
    background: var(--bg-surface-alt);
    border: var(--border-default);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: 13px;
  }

  .field input:focus, .field select:focus {
    outline: none;
    border-color: var(--amber);
    box-shadow: 0 0 0 2px rgba(217, 119, 6, 0.2);
  }

  .field-row {
    display: flex;
    gap: var(--sp-2);
  }
  .field-row .field { flex: 1; }

  .checkbox-row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--sp-3);
  }

  .checkbox {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--text-primary);
    cursor: pointer;
    text-transform: none;
    letter-spacing: normal;
    font-weight: 500;
  }

  .checkbox input {
    width: auto;
    accent-color: var(--amber);
  }

  .actions {
    display: flex;
    gap: var(--sp-2);
    margin-top: var(--sp-3);
    padding-top: var(--sp-3);
    border-top: var(--border-default);
  }

  .btn-primary {
    flex: 1;
    padding: var(--sp-2) var(--sp-4);
    background: var(--amber);
    color: white;
    font-weight: 600;
    font-size: 13px;
    border-radius: var(--radius-sm);
    transition: background 0.15s;
  }
  .btn-primary:hover { background: #B45309; }

  .btn-secondary {
    flex: 1;
    padding: var(--sp-2) var(--sp-4);
    background: var(--bg-surface-alt);
    color: var(--text-primary);
    font-weight: 500;
    font-size: 13px;
    border: var(--border-default);
    border-radius: var(--radius-sm);
    transition: background 0.15s;
  }
  .btn-secondary:hover { background: var(--bg-surface); }
</style>

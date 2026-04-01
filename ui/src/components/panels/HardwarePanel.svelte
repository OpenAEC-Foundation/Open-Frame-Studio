<script>
  import {
    currentKozijn,
    selectedCellIndex,
  } from "../../stores/kozijn.js";

  export let visible = true;

  let collapsed = false;

  // Default hardware configuration
  const defaultHardware = {
    scharnieren: { type: "opleg", aantal: 2 },
    greep: { type: "kruk", positie: "rechts", hoogte: 1050 },
    ventilatie: { type: "geen", kleur: "RAL9010" },
    sluitpunten: { aantal: 4 },
  };

  // Local hardware store keyed by kozijn id + cell index
  let hardwareMap = {};

  function getHardwareKey(kozijnId, cellIndex) {
    return `${kozijnId}_${cellIndex}`;
  }

  function getHardware(kozijnId, cellIndex) {
    const key = getHardwareKey(kozijnId, cellIndex);
    if (!hardwareMap[key]) {
      hardwareMap[key] = JSON.parse(JSON.stringify(defaultHardware));
    }
    return hardwareMap[key];
  }

  $: selectedCell =
    $currentKozijn && $selectedCellIndex !== null
      ? $currentKozijn.cells[$selectedCellIndex]
      : null;

  $: hardware =
    $currentKozijn && $selectedCellIndex !== null
      ? getHardware($currentKozijn.id, $selectedCellIndex)
      : null;

  // Auto-calculate locking points based on cell size
  $: if (selectedCell && hardware) {
    const area =
      (selectedCell.bounds?.width || 800) *
      (selectedCell.bounds?.height || 1200);
    const autoSluitpunten = area > 1_500_000 ? 6 : area > 800_000 ? 4 : 2;
    if (!hardwareMap[getHardwareKey($currentKozijn.id, $selectedCellIndex)]?._edited) {
      hardware.sluitpunten.aantal = autoSluitpunten;
    }
  }

  function updateField(section, field, value) {
    if (!$currentKozijn || $selectedCellIndex === null) return;
    const key = getHardwareKey($currentKozijn.id, $selectedCellIndex);
    if (!hardwareMap[key]) {
      hardwareMap[key] = JSON.parse(JSON.stringify(defaultHardware));
    }
    hardwareMap[key][section][field] = value;
    if (section === "sluitpunten") {
      hardwareMap[key]._edited = true;
    }
    hardwareMap = hardwareMap; // trigger reactivity
  }

  function toggleCollapsed() {
    collapsed = !collapsed;
  }
</script>

{#if visible}
  <div class="hardware-panel">
    <button class="collapse-header" on:click={toggleCollapsed}>
      <span class="collapse-icon" class:open={!collapsed}>&#9656;</span>
      <h3>Hang &amp; Sluitwerk</h3>
    </button>

    {#if !collapsed}
      {#if selectedCell && hardware}
        <!-- Scharnieren -->
        <div class="section">
          <h3>Scharnieren</h3>
          <div class="field">
            <label>Type</label>
            <select
              value={hardware.scharnieren.type}
              on:change={(e) => updateField("scharnieren", "type", e.target.value)}
            >
              <option value="opleg">Opleg</option>
              <option value="inboor">Inboor</option>
              <option value="verborgen">Verborgen</option>
            </select>
          </div>
          <div class="field">
            <label>Aantal</label>
            <input
              type="number"
              value={hardware.scharnieren.aantal}
              on:change={(e) => updateField("scharnieren", "aantal", parseInt(e.target.value))}
              min="2"
              max="4"
            />
          </div>
        </div>

        <!-- Greep -->
        <div class="section">
          <h3>Greep</h3>
          <div class="field">
            <label>Type</label>
            <select
              value={hardware.greep.type}
              on:change={(e) => updateField("greep", "type", e.target.value)}
            >
              <option value="kruk">Kruk</option>
              <option value="knop">Knop</option>
              <option value="stangenslot">Stangenslot</option>
              <option value="t-greep">T-greep</option>
            </select>
          </div>
          <div class="field-row">
            <div class="field">
              <label>Positie</label>
              <select
                value={hardware.greep.positie}
                on:change={(e) => updateField("greep", "positie", e.target.value)}
              >
                <option value="links">Links</option>
                <option value="rechts">Rechts</option>
              </select>
            </div>
            <div class="field">
              <label>Hoogte (mm)</label>
              <input
                type="number"
                value={hardware.greep.hoogte}
                on:change={(e) => updateField("greep", "hoogte", parseInt(e.target.value))}
                min="500"
                max="1500"
                step="10"
              />
            </div>
          </div>
        </div>

        <!-- Ventilatie -->
        <div class="section">
          <h3>Ventilatie</h3>
          <div class="field">
            <label>Type</label>
            <select
              value={hardware.ventilatie.type}
              on:change={(e) => updateField("ventilatie", "type", e.target.value)}
            >
              <option value="geen">Geen</option>
              <option value="rooster_boven">Rooster boven</option>
              <option value="rooster_onder">Rooster onder</option>
              <option value="suskasten">Suskasten</option>
            </select>
          </div>
          <div class="field">
            <label>Kleur</label>
            <input
              type="text"
              value={hardware.ventilatie.kleur}
              on:change={(e) => updateField("ventilatie", "kleur", e.target.value)}
            />
          </div>
        </div>

        <!-- Sluitpunten -->
        <div class="section">
          <h3>Sluitpunten</h3>
          <div class="field">
            <label>Aantal</label>
            <input
              type="number"
              value={hardware.sluitpunten.aantal}
              on:change={(e) => updateField("sluitpunten", "aantal", parseInt(e.target.value))}
              min="2"
              max="10"
            />
          </div>
        </div>
      {:else}
        <div class="hint">
          <p>Selecteer een cel om hang &amp; sluitwerk te configureren</p>
        </div>
      {/if}
    {/if}
  </div>
{/if}

<style>
  .hardware-panel {
    margin-bottom: var(--sp-4);
  }

  .collapse-header {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    width: 100%;
    background: none;
    border: none;
    cursor: pointer;
    padding: var(--sp-2) 0;
    margin-bottom: var(--sp-2);
    border-bottom: var(--border-default);
  }

  .collapse-header h3 {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--amber);
    margin: 0;
  }

  .collapse-icon {
    display: inline-block;
    font-size: 10px;
    color: var(--text-muted);
    transition: transform 0.15s ease;
  }

  .collapse-icon.open {
    transform: rotate(90deg);
  }

  .section {
    margin-bottom: var(--sp-4);
  }

  .section h3 {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--amber);
    margin-bottom: var(--sp-3);
    padding-bottom: var(--sp-2);
    border-bottom: var(--border-default);
  }

  .field {
    margin-bottom: var(--sp-3);
  }

  .field label {
    display: block;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    margin-bottom: var(--sp-1);
  }

  .field input,
  .field select {
    width: 100%;
    padding: var(--sp-2) var(--sp-3);
    background: var(--bg-surface-alt);
    border: var(--border-default);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: 13px;
  }

  .field input:focus,
  .field select:focus {
    outline: none;
    border-color: var(--amber);
    box-shadow: 0 0 0 2px rgba(217, 119, 6, 0.2);
  }

  .field-row {
    display: flex;
    gap: var(--sp-2);
  }

  .field-row .field {
    flex: 1;
  }

  .hint {
    color: var(--text-muted);
    font-size: 13px;
    font-style: italic;
    padding: var(--sp-2) 0;
  }
</style>

<script>
  import {
    currentKozijn,
    selectedCellIndex,
    selectedMember,
    updateDimensions,
    updateCellType,
    updateFrameProfile,
    updateSillProfile,
    updateDividerProfile,
    updateFrameShape,
    updateGridSizes,
    updateFrameColors,
    calculateThermal,
  } from "../../stores/kozijn.js";
  import { allProfiles } from "../../stores/profiles.js";
  import HardwarePanel from "./HardwarePanel.svelte";
  import GlazingPanel from "./GlazingPanel.svelte";
  import ProfileSelector from "./ProfileSelector.svelte";
  import ProfileCrossSection from "./ProfileCrossSection.svelte";
  import { RAL_COLORS, ralToHex } from "../../lib/ral-colors.js";

  let editWidth = 0;
  let editHeight = 0;
  let thermalResult = null;

  $: if ($currentKozijn) {
    editWidth = $currentKozijn.frame.outerWidth;
    editHeight = $currentKozijn.frame.outerHeight;
    calculateThermal().then(r => thermalResult = r);
  }

  $: selectedCell =
    $currentKozijn && $selectedCellIndex !== null
      ? $currentKozijn.cells[$selectedCellIndex]
      : null;

  function handleDimensionChange() {
    if (editWidth > 0 && editHeight > 0) {
      updateDimensions(editWidth, editHeight);
    }
  }

  const panelTypes = [
    { value: "fixed_glass", label: "Vast glas" },
    { value: "turn_tilt", label: "Draaikiepraam" },
    { value: "turn", label: "Draairaam" },
    { value: "tilt", label: "Kiepraam" },
    { value: "sliding", label: "Schuifraam" },
    { value: "door", label: "Deur" },
    { value: "panel", label: "Paneel" },
    { value: "ventilation", label: "Ventilatie" },
  ];

  function handlePanelTypeChange(e) {
    if ($selectedCellIndex === null) return;
    updateCellType($selectedCellIndex, e.target.value, null);
  }

  const MEMBER_LABELS = {
    frame_top: "Bovendorpel",
    frame_bottom: "Onderdorpel",
    frame_left: "Stijl links",
    frame_right: "Stijl rechts",
    divider_v: "Tussenstijl",
    divider_h: "Tussendorpel",
  };

  function getMemberLabel(member) {
    if (!member) return "";
    const base = MEMBER_LABELS[member.type] || member.type;
    if (member.type === "divider_v" || member.type === "divider_h") {
      return `${base} ${member.index + 1}`;
    }
    return base;
  }

  function getMemberProfile(member) {
    if (!member || !$currentKozijn) return null;
    const frame = $currentKozijn.frame;
    if (member.type === "frame_top") return frame.topProfile || frame.profile;
    if (member.type === "frame_bottom") return frame.bottomProfile || frame.sillProfile || frame.profile;
    if (member.type === "frame_left") return frame.leftProfile || frame.profile;
    if (member.type === "frame_right") return frame.rightProfile || frame.profile;
    if (member.type === "divider_v") {
      const col = $currentKozijn.grid.columns[member.index];
      return col?.dividerProfile || frame.profile;
    }
    if (member.type === "divider_h") {
      const row = $currentKozijn.grid.rows[member.index];
      return row?.dividerProfile || frame.profile;
    }
    return frame.profile;
  }

  function getMemberProfileFilter(member) {
    if (!member) return "frame";
    if (member.type === "frame_bottom") return "sill";
    if (member.type === "divider_v" || member.type === "divider_h") return "divider";
    return "frame";
  }

  function getMemberProfileDefinition(member) {
    const ref = getMemberProfile(member);
    if (!ref) return null;
    return ($allProfiles || []).find(p => p.id === ref.id) || null;
  }

  function handleMemberProfileChange(detail) {
    const member = $selectedMember;
    if (!member) return;
    if (member.type.startsWith("frame_")) {
      // For now, frame members share the main frame profile
      updateFrameProfile(detail.id, detail.name, detail.width, detail.depth);
    } else if (member.type === "divider_v") {
      updateDividerProfile(member.index, true, detail.id, detail.name);
    } else if (member.type === "divider_h") {
      updateDividerProfile(member.index, false, detail.id, detail.name);
    }
  }
</script>

<div class="panel">
  {#if $currentKozijn}
    <div class="section">
      <h3>Kozijn</h3>
      <div class="field">
        <label>Naam</label>
        <input type="text" value={$currentKozijn.name} disabled />
      </div>
      <div class="field">
        <label>Merk</label>
        <input type="text" value={$currentKozijn.mark} disabled />
      </div>
      <div class="field-row">
        <div class="field">
          <label>Breedte (mm)</label>
          <input
            type="number"
            bind:value={editWidth}
            on:change={handleDimensionChange}
            step="10"
            min="200"
            max="6000"
          />
        </div>
        <div class="field">
          <label>Hoogte (mm)</label>
          <input
            type="number"
            bind:value={editHeight}
            on:change={handleDimensionChange}
            step="10"
            min="200"
            max="4000"
          />
        </div>
      </div>
      <div class="field">
        <label>Materiaal</label>
        <div class="value">{$currentKozijn.frame.material?.wood || "Hout"}</div>
      </div>
    </div>

    <div class="section">
      <h3>Kleuren</h3>
      <div class="field">
        <label>Binnenzijde</label>
        <div class="color-row">
          <span class="color-swatch" style="background: {ralToHex($currentKozijn.frame.colorInside)}"></span>
          <select
            value={$currentKozijn.frame.colorInside}
            on:change={(e) => updateFrameColors(e.target.value, $currentKozijn.frame.colorOutside)}
          >
            {#each RAL_COLORS as ral}
              <option value={ral.code}>{ral.code} — {ral.name}</option>
            {/each}
          </select>
        </div>
      </div>
      <div class="field">
        <label>Buitenzijde</label>
        <div class="color-row">
          <span class="color-swatch" style="background: {ralToHex($currentKozijn.frame.colorOutside)}"></span>
          <select
            value={$currentKozijn.frame.colorOutside}
            on:change={(e) => updateFrameColors($currentKozijn.frame.colorInside, e.target.value)}
          >
            {#each RAL_COLORS as ral}
              <option value={ral.code}>{ral.code} — {ral.name}</option>
            {/each}
          </select>
        </div>
      </div>
    </div>

    <div class="section">
      <h3>Profielen</h3>
      <ProfileSelector
        label="Kozijnprofiel"
        filter="frame"
        value={$currentKozijn.frame.profile}
        on:change={(e) => updateFrameProfile(e.detail.id, e.detail.name, e.detail.width, e.detail.depth)}
      />
      <ProfileSelector
        label="Dorpelprofiel"
        filter="sill"
        value={$currentKozijn.frame.sillProfile}
        on:change={(e) => updateSillProfile(e.detail.id, e.detail.name)}
      />
    </div>

    <div class="section">
      <h3>Vorm</h3>
      <div class="field">
        <label>Kozijnvorm</label>
        <select
          value={$currentKozijn.frame.shape?.shapeType || "rectangular"}
          on:change={(e) => updateFrameShape(e.target.value, e.target.value === "arched" ? $currentKozijn.frame.outerWidth / 6 : null)}
        >
          <option value="rectangular">Rechthoekig</option>
          <option value="arched">Getoogd (segmentboog)</option>
          <option value="round">Rond</option>
        </select>
      </div>
      {#if $currentKozijn.frame.shape?.shapeType === "arched"}
        <div class="field">
          <label>Booghoogte (mm)</label>
          <input
            type="number"
            value={$currentKozijn.frame.shape.archHeight || Math.round($currentKozijn.frame.outerWidth / 6)}
            on:change={(e) => updateFrameShape("arched", parseFloat(e.target.value))}
            min="50"
            max={Math.round($currentKozijn.frame.outerHeight / 2)}
            step="10"
          />
        </div>
      {/if}
    </div>

    <div class="section">
      <h3>Vakmaten</h3>
      <div class="field">
        <label>Kolommen (mm)</label>
        {#each $currentKozijn.grid.columns as col, i}
          <div class="field-row" style="margin-bottom: 4px;">
            <span class="col-label">{i + 1}</span>
            <input
              type="number"
              value={Math.round(col.size)}
              on:change={(e) => {
                const sizes = $currentKozijn.grid.columns.map(c => c.size);
                sizes[i] = parseFloat(e.target.value) || sizes[i];
                updateGridSizes(sizes, $currentKozijn.grid.rows.map(r => r.size));
              }}
              min="100"
              step="10"
            />
          </div>
        {/each}
      </div>
      <div class="field">
        <label>Rijen (mm)</label>
        {#each $currentKozijn.grid.rows as row, i}
          <div class="field-row" style="margin-bottom: 4px;">
            <span class="col-label">{i + 1}</span>
            <input
              type="number"
              value={Math.round(row.size)}
              on:change={(e) => {
                const sizes = $currentKozijn.grid.rows.map(r => r.size);
                sizes[i] = parseFloat(e.target.value) || sizes[i];
                updateGridSizes($currentKozijn.grid.columns.map(c => c.size), sizes);
              }}
              min="100"
              step="10"
            />
          </div>
        {/each}
      </div>
    </div>

    {#if $selectedMember}
      <div class="section">
        <h3>Onderdeel</h3>
        <div class="field">
          <label>Type</label>
          <div class="value">{getMemberLabel($selectedMember)}</div>
        </div>
        <ProfileSelector
          label="Profiel"
          filter={getMemberProfileFilter($selectedMember)}
          value={getMemberProfile($selectedMember)}
          on:change={(e) => handleMemberProfileChange(e.detail)}
        />
        {#if getMemberProfile($selectedMember)}
          {@const profileDef = getMemberProfileDefinition($selectedMember)}
          <div class="field-row">
            <div class="field">
              <label>Breedte (mm)</label>
              <div class="value">{getMemberProfile($selectedMember)?.width || profileDef?.width || "—"}</div>
            </div>
            <div class="field">
              <label>Diepte (mm)</label>
              <div class="value">{getMemberProfile($selectedMember)?.depth || profileDef?.depth || "—"}</div>
            </div>
          </div>
          {#if profileDef?.sponning}
            <div class="field">
              <label>Sponning</label>
              <div class="value">{profileDef.sponning.width}x{profileDef.sponning.depth}mm ({profileDef.sponning.position})</div>
            </div>
          {/if}
          {#if profileDef?.ufValue}
            <div class="field">
              <label>Uf-waarde</label>
              <div class="value">{profileDef.ufValue} W/m²K</div>
            </div>
          {/if}
          {#if profileDef?.crossSection?.length > 2}
            <div class="field">
              <label>Dwarsdoorsnede</label>
              <ProfileCrossSection
                crossSection={profileDef.crossSection}
                sponning={profileDef.sponning}
              />
            </div>
          {/if}
        {/if}
      </div>
    {:else if selectedCell}
      <div class="section">
        <h3>Cel {$selectedCellIndex + 1}</h3>
        <div class="field">
          <label>Paneel type</label>
          <select value={selectedCell.panelType} on:change={handlePanelTypeChange}>
            {#each panelTypes as pt}
              <option value={pt.value}>{pt.label}</option>
            {/each}
          </select>
        </div>

        {#if ["turn_tilt", "turn", "tilt"].includes(selectedCell.panelType)}
          <div class="field">
            <label>Openingsrichting</label>
            <select value={selectedCell.openingDirection || "left"}
              on:change={(e) => updateCellType($selectedCellIndex, selectedCell.panelType, e.target.value)}>
              <option value="left">Links</option>
              <option value="right">Rechts</option>
            </select>
          </div>
          <ProfileSelector
            label="Raamhout profiel"
            filter="sash"
            value={selectedCell.sashProfile}
            on:change={(e) => {/* TODO: update sash profile */}}
          />
          {#if selectedCell.sashProfile}
            <div class="field-row">
              <div class="field">
                <label>Raambreedte</label>
                <div class="value">{selectedCell.sashWidth || 54}mm</div>
              </div>
              <div class="field">
                <label>Raamdiepte</label>
                <div class="value">{selectedCell.sashDepth || 67}mm</div>
              </div>
            </div>
          {/if}
        {/if}

        {#if selectedCell.panelType === "door"}
          <div class="field">
            <label>Openingsrichting</label>
            <select value={selectedCell.openingDirection || "inward"}
              on:change={(e) => updateCellType($selectedCellIndex, "door", e.target.value)}>
              <option value="inward">Naar binnen</option>
              <option value="outward">Naar buiten</option>
              <option value="left">Links</option>
              <option value="right">Rechts</option>
            </select>
          </div>
          <ProfileSelector
            label="Deurhout profiel"
            filter="sash"
            value={selectedCell.sashProfile}
            on:change={(e) => {/* TODO: update sash profile */}}
          />
        {/if}
      </div>
      <GlazingPanel />
      <HardwarePanel visible={true} />
    {:else}
      <div class="section hint">
        <p>Klik op een cel of onderdeel in het kozijn om de eigenschappen te bewerken</p>
      </div>
    {/if}

    {#if thermalResult}
      <div class="section">
        <h3>Thermisch</h3>
        <div class="field">
          <label>Uw-waarde (kozijn)</label>
          <div class="value thermal-badge" class:thermal-a={thermalResult.rating === "A"} class:thermal-b={thermalResult.rating === "B"} class:thermal-c={thermalResult.rating === "C"} class:thermal-d={thermalResult.rating === "D"}>
            {thermalResult.uwValue} W/m²K
            <span class="rating">{thermalResult.rating}</span>
          </div>
        </div>
        <div class="field-row">
          <div class="field">
            <label>Uf (kozijn)</label>
            <div class="value">{thermalResult.ufValue}</div>
          </div>
          <div class="field">
            <label>Ug (glas)</label>
            <div class="value">{thermalResult.ugValue}</div>
          </div>
          <div class="field">
            <label>Ψg</label>
            <div class="value">{thermalResult.psiValue}</div>
          </div>
        </div>
        <div class="field-row">
          <div class="field">
            <label>Glas %</label>
            <div class="value">{thermalResult.areaTotalM2 > 0 ? Math.round(thermalResult.areaGlassM2 / thermalResult.areaTotalM2 * 100) : 0}%</div>
          </div>
          <div class="field">
            <label>Opp. (m²)</label>
            <div class="value">{thermalResult.areaTotalM2}</div>
          </div>
        </div>
      </div>
    {/if}

    <div class="section">
      <h3>Grid</h3>
      <div class="field-row">
        <div class="field">
          <label>Kolommen</label>
          <div class="value">{$currentKozijn.grid.columns.length}</div>
        </div>
        <div class="field">
          <label>Rijen</label>
          <div class="value">{$currentKozijn.grid.rows.length}</div>
        </div>
      </div>
    </div>
  {:else}
    <div class="empty">
      <p>Selecteer of maak een kozijn</p>
    </div>
  {/if}
</div>

<style>
  .panel {
    width: 280px;
    flex-shrink: 0;
    background: var(--bg-surface);
    border-left: var(--border-default);
    overflow-y: auto;
    padding: var(--sp-4);
  }

  .section {
    margin-bottom: var(--sp-6);
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

  .field input, .field select {
    width: 100%;
    padding: var(--sp-2) var(--sp-3);
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
    align-items: center;
  }

  .field-row .field {
    flex: 1;
  }

  .col-label {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-muted);
    min-width: 16px;
  }

  .value {
    font-size: 13px;
    color: var(--text-primary);
    padding: var(--sp-2) 0;
  }

  .hint {
    color: var(--text-muted);
    font-size: 13px;
    font-style: italic;
  }

  .empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 200px;
    color: var(--text-muted);
    text-align: center;
  }

  .color-row {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
  }

  .color-row select {
    flex: 1;
    padding: var(--sp-2) var(--sp-3);
    background: var(--bg-surface-alt);
    border: var(--border-default);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: 11px;
  }

  .color-row select:focus {
    outline: none;
    border-color: var(--amber);
  }

  .color-swatch {
    display: inline-block;
    width: 20px;
    height: 20px;
    border-radius: 3px;
    border: 1px solid rgba(0, 0, 0, 0.2);
    flex-shrink: 0;
  }

  .thermal-badge {
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: var(--sp-2);
  }

  .rating {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    font-size: 11px;
    font-weight: 700;
    color: white;
  }

  .thermal-a .rating { background: #16A34A; }
  .thermal-b .rating { background: #84CC16; }
  .thermal-c .rating { background: #F59E0B; }
  .thermal-d .rating { background: #DC2626; }
</style>

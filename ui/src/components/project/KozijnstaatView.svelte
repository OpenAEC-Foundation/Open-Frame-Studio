<script>
  import { kozijnen } from "../../stores/project.js";
  import { currentKozijn, selectKozijn } from "../../stores/kozijn.js";

  const PANEL_LABELS = {
    fixed_glass: "VG",
    turn_tilt: "DK",
    turn: "D",
    tilt: "K",
    sliding: "S",
    door: "DR",
    panel: "P",
    ventilation: "V",
  };

  function materialLabel(material) {
    if (!material) return "-";
    if (typeof material === "string") return material;
    // Rust enum serializes as { "wood": "meranti" } or "aluminum" etc.
    if (material.wood) {
      const woods = { meranti: "Meranti", accoya: "Accoya", vuren: "Vuren", eiken: "Eiken" };
      return woods[material.wood] || material.wood;
    }
    if (material === "aluminum") return "Aluminium";
    if (material === "pvc") return "PVC";
    if (material === "wood_aluminum") return "Hout-Alu";
    // Fallback for object keys
    const key = Object.keys(material)[0];
    if (key === "wood") return materialLabel({ wood: material[key] });
    const labels = { aluminum: "Aluminium", pvc: "PVC", woodAluminum: "Hout-Alu" };
    return labels[key] || key || "-";
  }

  function cellSummary(kozijn) {
    if (!kozijn.cells || kozijn.cells.length === 0) return "-";
    const counts = {};
    for (const cell of kozijn.cells) {
      const label = PANEL_LABELS[cell.panelType] || "?";
      counts[label] = (counts[label] || 0) + 1;
    }
    return Object.entries(counts)
      .map(([k, v]) => `${v}x ${k}`)
      .join(", ");
  }

  function glazingSummary(kozijn) {
    if (!kozijn.cells || kozijn.cells.length === 0) return "-";
    const types = new Set();
    for (const cell of kozijn.cells) {
      if (cell.glazing && cell.glazing.glassType) {
        types.add(cell.glazing.glassType);
      }
    }
    return types.size > 0 ? [...types].join(", ") : "-";
  }

  function ugSummary(kozijn) {
    if (!kozijn.cells || kozijn.cells.length === 0) return "-";
    const values = new Set();
    for (const cell of kozijn.cells) {
      if (cell.glazing && cell.glazing.ugValue != null) {
        values.add(cell.glazing.ugValue);
      }
    }
    if (values.size === 0) return "-";
    const arr = [...values];
    if (arr.length === 1) return arr[0].toFixed(1);
    return arr.map((v) => v.toFixed(1)).join(" / ");
  }

  $: totalCells = $kozijnen.reduce((sum, k) => sum + (k.cells ? k.cells.length : 0), 0);
</script>

<div class="kozijnstaat">
  {#if $kozijnen.length === 0}
    <div class="empty-state">
      <p>Geen kozijnen in project</p>
    </div>
  {:else}
    <div class="table-wrapper">
      <table>
        <thead>
          <tr>
            <th>Mark</th>
            <th>Naam</th>
            <th class="num">Breedte (mm)</th>
            <th class="num">Hoogte (mm)</th>
            <th>Materiaal</th>
            <th class="num">Kolommen</th>
            <th class="num">Rijen</th>
            <th class="num">Cellen</th>
            <th>Paneel types</th>
            <th>Beglazing</th>
            <th class="num">Ug-waarde</th>
          </tr>
        </thead>
        <tbody>
          {#each $kozijnen as kozijn, i}
            <tr
              class:active={$currentKozijn?.id === kozijn.id}
              class:alt={i % 2 === 1}
              on:click={() => selectKozijn(kozijn.id)}
            >
              <td class="mark-cell">{kozijn.mark}</td>
              <td>{kozijn.name}</td>
              <td class="num">{kozijn.frame.outerWidth}</td>
              <td class="num">{kozijn.frame.outerHeight}</td>
              <td>{materialLabel(kozijn.frame.material)}</td>
              <td class="num">{kozijn.grid.columns.length}</td>
              <td class="num">{kozijn.grid.rows.length}</td>
              <td class="num">{kozijn.cells.length}</td>
              <td class="panel-types">{cellSummary(kozijn)}</td>
              <td>{glazingSummary(kozijn)}</td>
              <td class="num">{ugSummary(kozijn)}</td>
            </tr>
          {/each}
        </tbody>
        <tfoot>
          <tr>
            <td colspan="2" class="footer-label">Totaal: {$kozijnen.length} kozijnen</td>
            <td class="num">-</td>
            <td class="num">-</td>
            <td>-</td>
            <td class="num">-</td>
            <td class="num">-</td>
            <td class="num">{totalCells}</td>
            <td>-</td>
            <td>-</td>
            <td class="num">-</td>
          </tr>
        </tfoot>
      </table>
    </div>
  {/if}
</div>

<style>
  .kozijnstaat {
    width: 100%;
    height: 100%;
    overflow: auto;
    background: var(--bg-surface);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 200px;
  }

  .empty-state p {
    color: var(--text-muted);
    font-size: 14px;
  }

  .table-wrapper {
    width: 100%;
    min-width: fit-content;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
    color: var(--text-primary);
  }

  thead {
    position: sticky;
    top: 0;
    z-index: 1;
  }

  thead tr {
    background: var(--deep-forge);
    color: var(--text-on-dark);
  }

  th {
    padding: 8px 12px;
    text-align: left;
    font-weight: 600;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    white-space: nowrap;
    border-bottom: 2px solid var(--amber);
  }

  th.num {
    text-align: right;
  }

  tbody tr {
    background: var(--bg-surface);
    cursor: pointer;
    transition: background 0.1s;
  }

  tbody tr.alt {
    background: var(--bg-surface-alt);
  }

  tbody tr:hover {
    background: rgba(217, 119, 6, 0.08);
  }

  tbody tr.active {
    background: rgba(217, 119, 6, 0.15);
    outline: 2px solid var(--amber);
    outline-offset: -2px;
  }

  td {
    padding: 6px 12px;
    white-space: nowrap;
    border-bottom: 1px solid var(--border-subtle, rgba(255, 255, 255, 0.06));
  }

  td.num {
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  td.mark-cell {
    font-weight: 700;
    color: var(--amber);
  }

  td.panel-types {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .footer-label {
    font-weight: 700;
    color: var(--text-primary);
  }

  tfoot tr {
    background: var(--bg-surface-alt);
    border-top: 2px solid var(--amber);
  }

  tfoot td {
    padding: 8px 12px;
    font-weight: 600;
    font-size: 12px;
    color: var(--text-muted);
  }
</style>

<script>
  import { kozijnen } from "../../stores/project.js";
  import { selectKozijn, removeKozijn, currentKozijn } from "../../stores/kozijn.js";

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

  function cellSummary(kozijn) {
    const counts = {};
    for (const cell of kozijn.cells) {
      const label = PANEL_LABELS[cell.panelType] || "?";
      counts[label] = (counts[label] || 0) + 1;
    }
    return Object.entries(counts)
      .map(([k, v]) => `${v}x ${k}`)
      .join(", ");
  }
</script>

<div class="overview">
  <h3>Kozijnen in project</h3>
  {#if $kozijnen.length === 0}
    <p class="empty">Nog geen kozijnen. Maak er een via de ribbon toolbar.</p>
  {:else}
    <div class="list">
      {#each $kozijnen as kozijn}
        <button
          class="card"
          class:active={$currentKozijn?.id === kozijn.id}
          on:click={() => selectKozijn(kozijn.id)}
        >
          <div class="card-header">
            <span class="mark">{kozijn.mark}</span>
            <span class="name">{kozijn.name}</span>
            <button
              class="delete-btn"
              on:click|stopPropagation={() => removeKozijn(kozijn.id)}
              title="Verwijderen"
            >x</button>
          </div>
          <div class="card-info">
            <span>{kozijn.frame.outerWidth} x {kozijn.frame.outerHeight} mm</span>
            <span class="cells">{cellSummary(kozijn)}</span>
          </div>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .overview {
    width: 240px;
    flex-shrink: 0;
    background: var(--bg-surface-alt);
    border-right: var(--border-default);
    padding: var(--sp-3);
    overflow-y: auto;
  }

  h3 {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    margin-bottom: var(--sp-3);
  }

  .empty {
    color: var(--text-muted);
    font-size: 13px;
    text-align: center;
    padding: var(--sp-6) 0;
  }

  .list {
    display: flex;
    flex-direction: column;
    gap: var(--sp-2);
  }

  .card {
    text-align: left;
    padding: var(--sp-3);
    background: var(--bg-surface);
    border: var(--border-default);
    border-radius: var(--radius-md);
    transition: border-color 0.15s, box-shadow 0.15s;
    width: 100%;
  }

  .card:hover {
    border-color: var(--amber);
  }

  .card.active {
    border-color: var(--amber);
    box-shadow: 0 0 0 2px rgba(217, 119, 6, 0.2);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    margin-bottom: var(--sp-1);
  }

  .mark {
    background: var(--amber);
    color: var(--night-build);
    padding: 0 var(--sp-2);
    border-radius: var(--radius-sm);
    font-size: 11px;
    font-weight: 700;
  }

  .name {
    flex: 1;
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .delete-btn {
    width: 20px;
    height: 20px;
    font-size: 12px;
    color: var(--text-muted);
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .delete-btn:hover {
    background: var(--error);
    color: white;
  }

  .card-info {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: var(--text-muted);
  }

  .cells {
    color: var(--text-secondary);
  }
</style>

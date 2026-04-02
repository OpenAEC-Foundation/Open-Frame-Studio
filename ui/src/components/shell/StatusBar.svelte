<script>
  import { currentKozijn, calculateThermal } from "../../stores/kozijn.js";
  import { zoom } from "../../stores/ui.js";
  import { kozijnen } from "../../stores/project.js";

  let thermalResult = null;

  $: if ($currentKozijn) {
    calculateThermal().then(r => thermalResult = r);
  } else {
    thermalResult = null;
  }

  // Count cells missing hardware
  $: missingHardware = $currentKozijn
    ? $currentKozijn.cells.filter(c => {
        const operable = ["turn_tilt", "turn", "tilt", "sliding", "door"].includes(c.panelType);
        return operable && !c.hardwareSet;
      }).length
    : 0;

  $: material = $currentKozijn?.frame?.material?.wood
    || ($currentKozijn?.frame?.material === "aluminum" ? "Aluminium"
    : $currentKozijn?.frame?.material === "pvc" ? "PVC"
    : $currentKozijn?.frame?.material === "wood_aluminum" ? "Hout-Alu"
    : "Hout");
</script>

<div class="statusbar">
  <div class="left">
    {#if $currentKozijn}
      <span class="badge">{$currentKozijn.mark}</span>
      <span>{$currentKozijn.name}</span>
      <span class="dim">
        {$currentKozijn.frame.outerWidth} x {$currentKozijn.frame.outerHeight} mm
      </span>
      <span class="sep">|</span>
      <span class="dim">{material}</span>
      <span class="dim">{$currentKozijn.cells.length} cel(len)</span>
    {:else}
      <span class="dim">Geen kozijn geselecteerd</span>
    {/if}
  </div>

  <div class="center">
    {#if missingHardware > 0}
      <span class="warning">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 9v4m0 4h.01"/>
          <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z"/>
        </svg>
        {missingHardware} cel(len) zonder beslag
      </span>
    {/if}
  </div>

  <div class="right">
    {#if thermalResult}
      <span class="thermal" class:thermal-good={thermalResult.rating === "A"} class:thermal-ok={thermalResult.rating === "B"} class:thermal-warn={thermalResult.rating === "C" || thermalResult.rating === "D"}>
        Uw {thermalResult.uwValue}
      </span>
    {/if}
    <span class="dim">{$kozijnen.length} kozijn(en)</span>
    <span class="sep">|</span>
    <span class="dim">Zoom: {Math.round($zoom * 100)}%</span>
  </div>
</div>

<style>
  .statusbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 28px;
    padding: 0 var(--sp-3);
    background: var(--bg-statusbar);
    color: var(--text-on-dark);
    font-size: 12px;
  }

  .left, .right, .center {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
  }

  .center {
    flex: 1;
    justify-content: center;
  }

  .dim {
    color: var(--scaffold-gray);
  }

  .sep {
    color: rgba(255, 255, 255, 0.15);
    font-size: 10px;
  }

  .badge {
    background: var(--amber);
    color: var(--night-build);
    padding: 0 var(--sp-2);
    border-radius: var(--radius-sm);
    font-weight: 700;
    font-size: 11px;
  }

  .warning {
    display: flex;
    align-items: center;
    gap: 4px;
    color: #F59E0B;
    font-size: 11px;
    font-weight: 500;
  }

  .warning svg {
    stroke: #F59E0B;
  }

  .thermal {
    font-size: 11px;
    font-weight: 600;
    padding: 0 var(--sp-2);
    border-radius: var(--radius-sm);
  }

  .thermal-good { color: #16A34A; }
  .thermal-ok { color: #84CC16; }
  .thermal-warn { color: #F59E0B; }
</style>

<script>
  import { currentKozijn } from "../../stores/kozijn.js";
  import { zoom } from "../../stores/ui.js";
  import { kozijnen } from "../../stores/project.js";
</script>

<div class="statusbar">
  <div class="left">
    {#if $currentKozijn}
      <span class="badge">{$currentKozijn.mark}</span>
      <span>{$currentKozijn.name}</span>
      <span class="dim">
        {$currentKozijn.frame.outerWidth} x {$currentKozijn.frame.outerHeight} mm
      </span>
    {:else}
      <span class="dim">Geen kozijn geselecteerd</span>
    {/if}
  </div>
  <div class="right">
    <span class="dim">{$kozijnen.length} kozijn(en)</span>
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

  .left, .right {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
  }

  .dim {
    color: var(--scaffold-gray);
  }

  .badge {
    background: var(--amber);
    color: var(--night-build);
    padding: 0 var(--sp-2);
    border-radius: var(--radius-sm);
    font-weight: 700;
    font-size: 11px;
  }
</style>

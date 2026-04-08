<script>
  import { _ } from "svelte-i18n";
  import { currentKozijn, updateCornerJoints } from "../../stores/kozijn.js";

  let { visible = true } = $props();

  const CORNER_LABELS = ["Linksboven", "Rechtsboven", "Linksonder", "Rechtsonder"];
  const CORNER_IDS = ["top_left", "top_right", "bottom_left", "bottom_right"];

  const JOINT_TYPES = [
    { value: "pen_slis", label: "Pen/slis" },
    { value: "verstek", label: "Verstek (45°)" },
    { value: "contramal", label: "Contramal" },
    { value: "stomp", label: "Stomp" },
  ];

  const THROUGH_TYPES = [
    { value: "stijl", label: "Stijl (verticaal)" },
    { value: "dorpel", label: "Dorpel (horizontaal)" },
  ];

  function getJoints() {
    const k = $currentKozijn;
    if (!k || !k.frame.cornerJoints || k.frame.cornerJoints.length < 4) {
      // Default: 4 pen/slis joints
      return Array(4).fill(null).map(() => ({
        jointType: "pen_slis",
        throughMember: "stijl",
        angle: 90,
        penLength: 20,
      }));
    }
    return k.frame.cornerJoints;
  }

  async function updateJoint(index, field, value) {
    const joints = [...getJoints()];
    joints[index] = { ...joints[index], [field]: value };

    // Auto-set angle based on joint type
    if (field === "jointType") {
      joints[index].angle = value === "verstek" ? 45 : 90;
      joints[index].penLength = value === "pen_slis" ? 20 : 0;
    }

    await updateCornerJoints(joints);
  }

  /** Apply same joint type to all 4 corners at once */
  async function applyToAll(jointType) {
    const joints = getJoints().map(j => ({
      ...j,
      jointType,
      angle: jointType === "verstek" ? 45 : 90,
      penLength: jointType === "pen_slis" ? 20 : 0,
    }));
    await updateCornerJoints(joints);
  }
</script>

{#if visible && $currentKozijn}
  <div class="joint-panel">
    <h3>Hoekverbindingen</h3>

    <!-- Quick apply buttons -->
    <div class="quick-apply">
      <span class="label">Alle hoeken:</span>
      <button class="chip" onclick={() => applyToAll("pen_slis")}>Pen/slis</button>
      <button class="chip" onclick={() => applyToAll("verstek")}>Verstek</button>
      <button class="chip" onclick={() => applyToAll("stomp")}>Stomp</button>
    </div>

    <!-- Per-corner configuration -->
    {#each getJoints() as joint, i}
      <div class="corner-config">
        <div class="corner-header">
          <span class="corner-indicator" style="background: {joint.jointType === 'pen_slis' ? '#22c55e' : joint.jointType === 'verstek' ? '#3b82f6' : joint.jointType === 'contramal' ? '#f59e0b' : '#ef4444'}"></span>
          <span class="corner-name">{CORNER_LABELS[i]}</span>
        </div>
        <div class="corner-fields">
          <select
            value={joint.jointType}
            onchange={(e) => updateJoint(i, "jointType", e.target.value)}
          >
            {#each JOINT_TYPES as jt}
              <option value={jt.value}>{jt.label}</option>
            {/each}
          </select>
          <select
            value={joint.throughMember}
            onchange={(e) => updateJoint(i, "throughMember", e.target.value)}
          >
            {#each THROUGH_TYPES as tt}
              <option value={tt.value}>{tt.label}</option>
            {/each}
          </select>
          {#if joint.jointType === "pen_slis"}
            <div class="pen-length">
              <label>Pen:</label>
              <input
                type="number"
                value={joint.penLength}
                onchange={(e) => updateJoint(i, "penLength", parseFloat(e.target.value))}
                min="10" max="50" step="5"
              />
              <span class="unit">mm</span>
            </div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
{/if}

<style>
  .joint-panel {
    padding: var(--sp-3);
  }

  h3 {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0 0 var(--sp-3);
  }

  .quick-apply {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    margin-bottom: var(--sp-3);
    flex-wrap: wrap;
  }

  .quick-apply .label {
    font-size: 11px;
    color: var(--text-muted);
  }

  .chip {
    padding: 2px 8px;
    font-size: 10px;
    font-weight: 600;
    border: 1px solid var(--border-color, #444);
    border-radius: 10px;
    background: transparent;
    color: var(--text-secondary);
    cursor: default;
    transition: all 0.15s;
  }

  .chip:hover {
    background: var(--amber);
    color: var(--bg-surface);
    border-color: var(--amber);
  }

  .corner-config {
    margin-bottom: var(--sp-3);
    padding: var(--sp-2);
    background: var(--bg-surface-alt);
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-color, #333);
  }

  .corner-header {
    display: flex;
    align-items: center;
    gap: var(--sp-2);
    margin-bottom: var(--sp-2);
  }

  .corner-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .corner-name {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .corner-fields {
    display: flex;
    flex-direction: column;
    gap: var(--sp-1);
  }

  .corner-fields select {
    width: 100%;
    padding: 3px 6px;
    font-size: 11px;
    background: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-color, #444);
    border-radius: var(--radius-sm);
  }

  .pen-length {
    display: flex;
    align-items: center;
    gap: var(--sp-1);
    font-size: 11px;
    color: var(--text-secondary);
  }

  .pen-length input {
    width: 50px;
    padding: 2px 4px;
    font-size: 11px;
    background: var(--bg-surface);
    color: var(--text-primary);
    border: 1px solid var(--border-color, #444);
    border-radius: var(--radius-sm);
    text-align: center;
  }

  .unit {
    color: var(--text-muted);
    font-size: 10px;
  }
</style>

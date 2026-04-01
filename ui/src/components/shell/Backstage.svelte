<script>
  import { showBackstage } from "../../stores/ui.js";
  import { newProject, openProject, saveProject, projectPath } from "../../stores/project.js";
  import { isTauri } from "../../lib/tauri.js";
  import { get } from "svelte/store";

  async function getDialogs() {
    if (isTauri) {
      return await import("@tauri-apps/plugin-dialog");
    }
    // Browser fallback
    return {
      open: async () => prompt("Bestandspad om te openen:"),
      save: async () => prompt("Bestandspad om op te slaan:", "project.ofs"),
    };
  }

  async function handleNew() {
    await newProject("Nieuw project", "");
    showBackstage.set(false);
  }

  async function handleOpen() {
    const { open } = await getDialogs();
    const path = await open({
      filters: [{ name: "Open Frame Studio", extensions: ["ofs"] }],
      multiple: false,
    });
    if (path) {
      await openProject(path);
      showBackstage.set(false);
    }
  }

  async function handleSave() {
    let path = get(projectPath);
    if (!path) {
      const { save } = await getDialogs();
      path = await save({
        filters: [{ name: "Open Frame Studio", extensions: ["ofs"] }],
        defaultPath: "project.ofs",
      });
    }
    if (path) {
      await saveProject(path);
      showBackstage.set(false);
    }
  }

  async function handleSaveAs() {
    const { save } = await getDialogs();
    const path = await save({
      filters: [{ name: "Open Frame Studio", extensions: ["ofs"] }],
      defaultPath: "project.ofs",
    });
    if (path) {
      await saveProject(path);
      showBackstage.set(false);
    }
  }

  function handleKeydown(e) {
    if (e.key === "Escape") showBackstage.set(false);
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if $showBackstage}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="overlay" on:click={() => showBackstage.set(false)}>
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div class="backstage" on:click|stopPropagation>
      <div class="sidebar">
        <div class="brand">
          <span class="brand-name">Open Frame Studio</span>
          <span class="brand-org">OpenAEC Foundation</span>
        </div>
        <nav>
          <button class="nav-btn" on:click={handleNew}>Nieuw project</button>
          <button class="nav-btn" on:click={handleOpen}>Openen</button>
          <button class="nav-btn" on:click={handleSave}>Opslaan</button>
          <button class="nav-btn" on:click={handleSaveAs}>Opslaan als...</button>
        </nav>
        <button class="nav-btn close-btn" on:click={() => showBackstage.set(false)}>
          Terug
        </button>
      </div>
      <div class="content">
        <h2>Welkom bij Open Frame Studio</h2>
        <p>Open-source kozijntekensoftware voor architecten, timmerfabrieken en BIM-modelleurs.</p>
        <div class="recent">
          <h3>Snelstart</h3>
          <p>Klik op "Nieuw project" om te beginnen, of open een bestaand .ofs bestand.</p>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 50;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
  }

  .backstage {
    display: flex;
    width: 100%;
    height: 100%;
    background: var(--bg-surface);
  }

  .sidebar {
    width: 260px;
    background: var(--deep-forge);
    color: var(--text-on-dark);
    display: flex;
    flex-direction: column;
    padding: var(--sp-4) 0;
  }

  .brand {
    padding: var(--sp-4) var(--sp-6);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    margin-bottom: var(--sp-4);
  }

  .brand-name {
    display: block;
    font-family: var(--font-heading);
    font-weight: 700;
    font-size: 16px;
    color: var(--amber);
  }

  .brand-org {
    font-size: 11px;
    color: var(--scaffold-gray);
  }

  nav {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .nav-btn {
    padding: var(--sp-3) var(--sp-6);
    text-align: left;
    font-size: 14px;
    color: var(--text-on-dark);
    transition: background 0.15s;
  }

  .nav-btn:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .close-btn {
    margin-top: auto;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    color: var(--scaffold-gray);
  }

  .content {
    flex: 1;
    padding: var(--sp-12) var(--sp-16);
  }

  .content h2 {
    font-size: 1.5rem;
    margin-bottom: var(--sp-4);
    color: var(--text-primary);
  }

  .content p {
    color: var(--text-secondary);
    max-width: 500px;
    line-height: 1.6;
  }

  .recent {
    margin-top: var(--sp-8);
  }

  .recent h3 {
    font-size: 1rem;
    margin-bottom: var(--sp-3);
    color: var(--text-primary);
  }
</style>

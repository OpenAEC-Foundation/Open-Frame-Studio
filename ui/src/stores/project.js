import { writable, derived } from "svelte/store";
import { invoke } from "../lib/tauri.js";

export const project = writable(null);
export const projectPath = writable(null);

export const kozijnen = derived(project, ($project) =>
  $project ? $project.kozijnen : []
);

export async function loadProject() {
  try {
    const p = await invoke("get_project");
    project.set(p);
    return p;
  } catch (e) {
    console.error("Laden project mislukt:", e);
    return null;
  }
}

export async function newProject(name, number) {
  const p = await invoke("new_project", { name, number });
  project.set(p);
  projectPath.set(null);
  return p;
}

export async function openProject(filePath) {
  const p = await invoke("open_project", { filePath });
  project.set(p);
  projectPath.set(filePath);
  return p;
}

export async function saveProject(filePath) {
  await invoke("save_project", { filePath });
  projectPath.set(filePath);
}

export async function refreshProject() {
  return loadProject();
}

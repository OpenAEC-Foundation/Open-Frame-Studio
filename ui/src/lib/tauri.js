/**
 * Safe Tauri invoke wrapper — returns mock data when running in browser.
 */
export const isTauri = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;

export async function invoke(cmd, args) {
  if (isTauri) {
    const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
    return tauriInvoke(cmd, args);
  }
  // Browser mock fallback
  return mockCommand(cmd, args);
}

// In-memory mock state for browser preview
const mockState = {
  project: {
    formatVersion: "1.0",
    projectInfo: {
      name: "Demo Project",
      number: "2026-001",
      client: "Preview Mode",
      address: "",
    },
    kozijnen: [],
  },
  nextMark: 1,
};

function mockCommand(cmd, args) {
  switch (cmd) {
    case "get_project":
      return structuredClone(mockState.project);

    case "new_project":
      mockState.project = {
        formatVersion: "1.0",
        projectInfo: { name: args.name, number: args.number, client: "", address: "" },
        kozijnen: [],
      };
      mockState.nextMark = 1;
      return structuredClone(mockState.project);

    case "create_kozijn": {
      const k = createMockKozijn(args.name, args.mark, args.width, args.height);
      mockState.project.kozijnen.push(k);
      return structuredClone(k);
    }

    case "create_kozijn_from_template": {
      const k = createMockKozijn(
        args.template.replace(/_/g, " "),
        `K${String(mockState.nextMark++).padStart(2, "0")}`,
        args.width,
        args.height
      );
      // Add cells based on template
      if (args.template === "double_turn_tilt") {
        const fw = 67;
        const half = (args.width - 2 * fw - fw) / 2;
        k.grid.columns = [
          { size: half, dividerProfile: null },
          { size: half, dividerProfile: { id: "div", name: "Divider" } },
        ];
        k.cells = [
          { ...defaultCell(), panelType: "turn_tilt", openingDirection: "left" },
          { ...defaultCell(), panelType: "turn_tilt", openingDirection: "right" },
        ];
      } else if (args.template === "sliding_door") {
        const fw = 67;
        const half = (args.width - 2 * fw - fw) / 2;
        k.grid.columns = [
          { size: half, dividerProfile: null },
          { size: half, dividerProfile: { id: "div", name: "Divider" } },
        ];
        k.cells = [
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "sliding" },
        ];
      } else if (args.template === "front_door") {
        const fw = 67;
        const topLight = 400;
        const doorH = args.height - 2 * fw - topLight - fw;
        k.grid.rows = [
          { size: topLight, dividerProfile: null },
          { size: doorH, dividerProfile: { id: "div", name: "Divider" } },
        ];
        k.cells = [
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "door", openingDirection: "inward" },
        ];
      } else if (args.template === "single_turn_tilt") {
        k.cells = [{ ...defaultCell(), panelType: "turn_tilt", openingDirection: "left",
          sashProfile: { id: "raam-meranti-54x67", name: "Raamhout 54x67mm" }, sashWidth: 54, sashDepth: 67 }];
      } else if (args.template === "melkmeisje") {
        const fw = 67;
        const innerW = args.width - 2 * fw;
        const innerH = args.height - 2 * fw;
        const sideW = Math.min(400, (innerW - 2 * fw) / 4);
        const doorW = innerW - 2 * sideW - 2 * fw;
        const borstH = 600;
        const topH = innerH - borstH - fw;
        k.grid.columns = [
          { size: sideW, dividerProfile: null },
          { size: doorW, dividerProfile: { id: "div", name: "Divider" } },
          { size: sideW, dividerProfile: { id: "div", name: "Divider" } },
        ];
        k.grid.rows = [
          { size: topH, dividerProfile: null },
          { size: borstH, dividerProfile: { id: "div", name: "Divider" } },
        ];
        k.cells = [
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "panel" },
          { ...defaultCell(), panelType: "door", openingDirection: "inward" },
          { ...defaultCell(), panelType: "panel" },
        ];
      } else if (args.template === "melkmeisje_bovenlicht") {
        const fw = 67;
        const innerW = args.width - 2 * fw;
        const innerH = args.height - 2 * fw;
        const sideW = Math.min(400, (innerW - 2 * fw) / 4);
        const doorW = innerW - 2 * sideW - 2 * fw;
        const bovenlichtH = 350;
        const borstH = 600;
        const midH = innerH - bovenlichtH - borstH - 2 * fw;
        k.grid.columns = [
          { size: sideW, dividerProfile: null },
          { size: doorW, dividerProfile: { id: "div", name: "Divider" } },
          { size: sideW, dividerProfile: { id: "div", name: "Divider" } },
        ];
        k.grid.rows = [
          { size: bovenlichtH, dividerProfile: null },
          { size: midH, dividerProfile: { id: "div", name: "Divider" } },
          { size: borstH, dividerProfile: { id: "div", name: "Divider" } },
        ];
        k.cells = [
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "door", openingDirection: "inward" },
          { ...defaultCell(), panelType: "fixed_glass" },
          { ...defaultCell(), panelType: "panel" },
          { ...defaultCell(), panelType: "panel" },
          { ...defaultCell(), panelType: "panel" },
        ];
      }
      mockState.project.kozijnen.push(k);
      return structuredClone(k);
    }

    case "get_kozijn": {
      const found = mockState.project.kozijnen.find((k) => k.id === args.id);
      return found ? structuredClone(found) : Promise.reject("Kozijn niet gevonden");
    }

    case "get_all_kozijnen":
      return structuredClone(mockState.project.kozijnen);

    case "update_kozijn_dimensions": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      const scaleW = (args.width - 2 * k.frame.frameWidth) / (k.frame.outerWidth - 2 * k.frame.frameWidth);
      const scaleH = (args.height - 2 * k.frame.frameWidth) / (k.frame.outerHeight - 2 * k.frame.frameWidth);
      k.frame.outerWidth = args.width;
      k.frame.outerHeight = args.height;
      k.grid.columns.forEach((c) => (c.size *= scaleW));
      k.grid.rows.forEach((r) => (r.size *= scaleH));
      return structuredClone(k);
    }

    case "update_cell_type": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      const cell = k.cells[args.cellIndex];
      cell.panelType = args.panelType;
      cell.openingDirection = args.openingDirection;
      // Auto-assign sash profile (professional workflow)
      if (["turn_tilt", "turn", "tilt", "sliding"].includes(args.panelType)) {
        cell.sashProfile = cell.sashProfile || { id: "raam-meranti-54x67", name: "Raamhout 54x67mm" };
        cell.sashWidth = cell.sashWidth || 54;
        cell.sashDepth = cell.sashDepth || 67;
      } else if (args.panelType === "door") {
        cell.sashProfile = cell.sashProfile || { id: "deur-meranti-67x114", name: "Deurhout 67x114mm" };
        cell.sashWidth = cell.sashWidth || 67;
        cell.sashDepth = cell.sashDepth || 114;
      } else {
        cell.sashProfile = null;
        cell.sashWidth = null;
        cell.sashDepth = null;
      }
      return structuredClone(k);
    }

    case "get_sjablonen":
      return [
        { id: "standaard-67-meranti", name: "Standaard 67mm Meranti", profileSeries: "67", frameWidth: 67, frameDepth: 114, sashWidth: 54, sashDepth: 67 },
        { id: "standaard-67-accoya", name: "Standaard 67mm Accoya", profileSeries: "67", frameWidth: 67, frameDepth: 114, sashWidth: 54, sashDepth: 67 },
        { id: "zwaar-78-meranti", name: "Zwaar 78mm Meranti", profileSeries: "78", frameWidth: 78, frameDepth: 114, sashWidth: 54, sashDepth: 78 },
        { id: "passief-90-meranti", name: "Passief 90mm Meranti", profileSeries: "90", frameWidth: 90, frameDepth: 114, sashWidth: 54, sashDepth: 90 },
      ];

    case "add_column": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      // Simple: split the first column that contains the position
      let acc = 0;
      for (let i = 0; i < k.grid.columns.length; i++) {
        const end = acc + k.grid.columns[i].size;
        if (args.position > acc && args.position < end) {
          const left = args.position - acc;
          const right = end - args.position;
          k.grid.columns[i].size = left;
          k.grid.columns.splice(i + 1, 0, { size: right, dividerProfile: { id: "div", name: "Divider" } });
          break;
        }
        acc = end;
      }
      // Rebuild cells
      const numCells = k.grid.columns.length * k.grid.rows.length;
      while (k.cells.length < numCells) k.cells.push(defaultCell());
      return structuredClone(k);
    }

    case "add_row": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      let acc = 0;
      for (let i = 0; i < k.grid.rows.length; i++) {
        const end = acc + k.grid.rows[i].size;
        if (args.position > acc && args.position < end) {
          const top = args.position - acc;
          const bottom = end - args.position;
          k.grid.rows[i].size = top;
          k.grid.rows.splice(i + 1, 0, { size: bottom, dividerProfile: { id: "div", name: "Divider" } });
          break;
        }
        acc = end;
      }
      const numCells = k.grid.columns.length * k.grid.rows.length;
      while (k.cells.length < numCells) k.cells.push(defaultCell());
      return structuredClone(k);
    }

    case "remove_kozijn":
      mockState.project.kozijnen = mockState.project.kozijnen.filter((k) => k.id !== args.id);
      return;

    case "get_kozijn_geometry": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      return computeGeometry(k);
    }

    case "save_project":
    case "open_project":
      return structuredClone(mockState.project);

    case "export_ifc":
      return "mock-export.ifc";

    case "export_dxf":
      return "mock-export.dxf";

    case "export_kozijnstaat":
      return "mock-kozijnstaat." + (args?.format || "pdf");

    case "export_workshop_drawing":
      return "mock-werkplaats.pdf";

    case "send_to_blender":
      return "OK (browser mock)";

    case "check_blender_connection":
      return false;

    case "update_cell_glazing": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      const glazing = JSON.parse(args.glazingJson);
      k.cells[args.cellIndex].glazing = glazing;
      return structuredClone(k);
    }

    case "update_frame_colors": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      k.frame.colorInside = args.colorInside;
      k.frame.colorOutside = args.colorOutside;
      return structuredClone(k);
    }

    case "duplicate_kozijn": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      const dup = structuredClone(k);
      dup.id = crypto.randomUUID();
      dup.mark = args.newMark;
      dup.name = k.name + " (kopie)";
      mockState.project.kozijnen.push(dup);
      return structuredClone(dup);
    }

    case "calculate_thermal": {
      const k = mockState.project.kozijnen.find((k) => k.id === args.id);
      if (!k) return Promise.reject("Kozijn niet gevonden");
      const fw = k.frame.frameWidth || 67;
      const ow = k.frame.outerWidth;
      const oh = k.frame.outerHeight;
      const areaTotal = (ow * oh) / 1e6;
      const iw = ow - 2 * fw;
      const ih = oh - 2 * fw;
      const areaInner = (iw * ih) / 1e6;
      const areaFrame = areaTotal - areaInner;
      const areaGlass = areaInner;
      const ufValue = 1.8;
      const ugValue = k.cells.length > 0 ? k.cells.reduce((s, c) => s + c.glazing.ugValue, 0) / k.cells.length : 1.0;
      const psiValue = 0.04;
      const glassPeri = k.cells.length * 2 * (iw / k.grid.columns.length + ih / k.grid.rows.length) / 1000;
      const uwValue = areaTotal > 0 ? Math.round((areaFrame * ufValue + areaGlass * ugValue + glassPeri * psiValue) / areaTotal * 100) / 100 : 0;
      const rating = uwValue < 1.3 ? "A" : uwValue < 1.8 ? "B" : uwValue < 2.5 ? "C" : "D";
      return { uwValue, ufValue, ugValue, psiValue, areaFrameM2: Math.round(areaFrame * 1000) / 1000, areaGlassM2: Math.round(areaGlass * 1000) / 1000, areaTotalM2: Math.round(areaTotal * 1000) / 1000, glassPerimeterM: Math.round(glassPeri * 100) / 100, rating };
    }

    case "import_dxf_profile":
      return JSON.stringify({ id: "imported-test", name: "Test profiel", width: 67, depth: 114 });

    case "import_catalog":
      return JSON.stringify([]);

    default:
      console.warn(`Mock: unknown command "${cmd}"`);
      return null;
  }
}

function createMockKozijn(name, mark, width, height) {
  const fw = 67;
  return {
    id: crypto.randomUUID(),
    name,
    mark,
    frame: {
      outerWidth: width,
      outerHeight: height,
      material: { wood: "meranti" },
      profile: { id: "wood-meranti-67x114", name: "Meranti 67x114mm" },
      sillProfile: { id: "wood-meranti-67x150", name: "Meranti 67x150mm" },
      colorInside: "RAL9010",
      colorOutside: "RAL9010",
      frameWidth: fw,
      frameDepth: 114,
    },
    grid: {
      columns: [{ size: width - 2 * fw, dividerProfile: null }],
      rows: [{ size: height - 2 * fw, dividerProfile: null }],
    },
    cells: [defaultCell()],
    placement: { wallId: null, x: 0, y: 0, z: 0, orientation: 0 },
  };
}

function defaultCell() {
  return {
    panelType: "fixed_glass",
    openingDirection: null,
    glazing: { glassType: "HR++", thicknessMm: 24, ugValue: 1.0 },
    hardware: [],
  };
}

function computeGeometry(kozijn) {
  const fw = kozijn.frame.frameWidth;
  const ow = kozijn.frame.outerWidth;
  const oh = kozijn.frame.outerHeight;
  const dw = fw;

  const outerRect = { x: 0, y: 0, width: ow, height: oh };
  const innerRect = { x: fw, y: fw, width: ow - 2 * fw, height: oh - 2 * fw };

  const frameRects = [
    { x: 0, y: 0, width: ow, height: fw },
    { x: 0, y: oh - fw, width: ow, height: fw },
    { x: 0, y: fw, width: fw, height: oh - 2 * fw },
    { x: ow - fw, y: fw, width: fw, height: oh - 2 * fw },
  ];

  const colPositions = [];
  let cx = fw;
  for (let i = 0; i < kozijn.grid.columns.length; i++) {
    colPositions.push(cx);
    cx += kozijn.grid.columns[i].size;
    if (i < kozijn.grid.columns.length - 1) cx += dw;
  }

  const rowPositions = [];
  let ry = fw;
  for (let i = 0; i < kozijn.grid.rows.length; i++) {
    rowPositions.push(ry);
    ry += kozijn.grid.rows[i].size;
    if (i < kozijn.grid.rows.length - 1) ry += dw;
  }

  const vDividers = [];
  let vx = fw;
  for (let i = 0; i < kozijn.grid.columns.length; i++) {
    vx += kozijn.grid.columns[i].size;
    if (i < kozijn.grid.columns.length - 1) {
      vDividers.push({ x: vx, y: fw, width: dw, height: oh - 2 * fw });
      vx += dw;
    }
  }

  const hDividers = [];
  let hy = fw;
  for (let i = 0; i < kozijn.grid.rows.length; i++) {
    hy += kozijn.grid.rows[i].size;
    if (i < kozijn.grid.rows.length - 1) {
      hDividers.push({ x: fw, y: hy, width: ow - 2 * fw, height: dw });
      hy += dw;
    }
  }

  const numCols = kozijn.grid.columns.length;
  const cellRects = [];
  for (let ri = 0; ri < kozijn.grid.rows.length; ri++) {
    for (let ci = 0; ci < kozijn.grid.columns.length; ci++) {
      cellRects.push({
        rect: {
          x: colPositions[ci],
          y: rowPositions[ri],
          width: kozijn.grid.columns[ci].size,
          height: kozijn.grid.rows[ri].size,
        },
        col: ci,
        row: ri,
        cellIndex: ri * numCols + ci,
      });
    }
  }

  const dimOffset = 30;
  const dimensions = [
    { x1: 0, y1: oh + dimOffset, x2: ow, y2: oh + dimOffset, label: String(Math.round(ow)), side: "bottom" },
    { x1: ow + dimOffset, y1: 0, x2: ow + dimOffset, y2: oh, label: String(Math.round(oh)), side: "right" },
  ];

  kozijn.grid.columns.forEach((col, i) => {
    dimensions.push({
      x1: colPositions[i], y1: oh + dimOffset * 2,
      x2: colPositions[i] + col.size, y2: oh + dimOffset * 2,
      label: String(Math.round(col.size)), side: "bottom",
    });
  });

  kozijn.grid.rows.forEach((row, i) => {
    dimensions.push({
      x1: ow + dimOffset * 2, y1: rowPositions[i],
      x2: ow + dimOffset * 2, y2: rowPositions[i] + row.size,
      label: String(Math.round(row.size)), side: "right",
    });
  });

  return { outerRect, innerRect, frameRects, hDividers, vDividers, cellRects, dimensions };
}

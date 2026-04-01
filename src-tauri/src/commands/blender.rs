use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn check_blender_connection() -> Result<bool, String> {
    match crate::blender::ping().await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub async fn send_to_blender(
    state: State<'_, AppState>,
    id: String,
) -> Result<String, String> {
    // Extract data from mutex before any await points
    let kozijn_json = {
        let project = state.project.lock().map_err(|e| e.to_string())?;
        let id: uuid::Uuid = id.parse().map_err(|e: uuid::Error| e.to_string())?;
        let kozijn = project
            .kozijnen
            .iter()
            .find(|k| k.id == id)
            .ok_or("Kozijn niet gevonden")?;
        serde_json::to_string(kozijn).map_err(|e| e.to_string())?
    }; // MutexGuard dropped here

    // Generate Python code that creates the kozijn in Blender/Bonsai
    // Uses Bonsai's IFC API for proper BIM kozijn creation
    let python_code = format!(
        r#"
import bpy
import json
import ifcopenshell
import ifcopenshell.api

kozijn = json.loads('''{json}''')

# Zorg dat er een IFC project is
ifc_file = None
if hasattr(bpy.context.scene, 'BIMProperties') and bpy.context.scene.BIMProperties.ifc_file:
    import blenderbim.tool as tool
    ifc_file = tool.Ifc.get()
else:
    bpy.ops.bim.create_project()
    import blenderbim.tool as tool
    ifc_file = tool.Ifc.get()

if not ifc_file:
    raise Exception("Kan geen IFC bestand laden of aanmaken")

# Kozijn gegevens
w = kozijn["frame"]["outerWidth"] / 1000.0
h = kozijn["frame"]["outerHeight"] / 1000.0
d = kozijn["frame"]["frameDepth"] / 1000.0
fw = kozijn["frame"]["frameWidth"] / 1000.0
name = kozijn["name"]
mark = kozijn["mark"]

# Bepaal of het een deur of raam is
has_door = any(c.get("panelType") == "door" for c in kozijn.get("cells", []))
ifc_class = "IfcDoor" if has_door else "IfcWindow"

# Maak het element aan
element = ifcopenshell.api.run("root.create_entity", ifc_file,
    ifc_class=ifc_class, name=name)
element.Tag = mark
element.OverallWidth = w
element.OverallHeight = h

# Geometrie context
body = None
for ctx in ifc_file.by_type("IfcGeometricRepresentationSubContext"):
    if ctx.ContextIdentifier == "Body":
        body = ctx
        break

if body:
    # Maak frame geometrie (buitenprofiel - binnenprofiel)
    outer = ifc_file.createIfcPolyline([
        ifc_file.createIfcCartesianPoint(p) for p in
        [(0.,0.), (w,0.), (w,h), (0.,h), (0.,0.)]
    ])
    inner = ifc_file.createIfcPolyline([
        ifc_file.createIfcCartesianPoint(p) for p in
        [(fw,fw), (w-fw,fw), (w-fw,h-fw), (fw,h-fw), (fw,fw)]
    ])
    profile = ifc_file.createIfcArbitraryProfileDefWithVoids(
        "AREA", "KozijnProfiel", outer, [inner])

    direction = ifc_file.createIfcDirection((0.,0.,1.))
    placement = ifc_file.createIfcAxis2Placement3D(
        ifc_file.createIfcCartesianPoint((0.,0.,0.)),
        ifc_file.createIfcDirection((0.,0.,1.)),
        ifc_file.createIfcDirection((1.,0.,0.)))

    extrusion = ifc_file.createIfcExtrudedAreaSolid(profile, placement, direction, d)

    shape = ifc_file.createIfcShapeRepresentation(body, "Body", "SweptSolid", [extrusion])
    product_shape = ifc_file.createIfcProductDefinitionShape(None, None, [shape])
    element.Representation = product_shape

# Plaats in de actieve verdieping
storey = None
for s in ifc_file.by_type("IfcBuildingStorey"):
    storey = s
    break

if storey:
    ifcopenshell.api.run("spatial.assign_container", ifc_file,
        relating_structure=storey, products=[element])

# Maak plaatsing
local_placement = ifc_file.createIfcLocalPlacement(None,
    ifc_file.createIfcAxis2Placement3D(
        ifc_file.createIfcCartesianPoint((0.,0.,0.)),
        ifc_file.createIfcDirection((0.,0.,1.)),
        ifc_file.createIfcDirection((1.,0.,0.))))
element.ObjectPlacement = local_placement

# Property sets
pset_name = "Pset_DoorCommon" if has_door else "Pset_WindowCommon"
pset = ifcopenshell.api.run("pset.add_pset", ifc_file,
    product=element, name=pset_name)
ifcopenshell.api.run("pset.edit_pset", ifc_file, pset=pset,
    properties={{"Reference": mark, "IsExternal": True}})

# Voeg OFS property set toe
ofs_pset = ifcopenshell.api.run("pset.add_pset", ifc_file,
    product=element, name="Pset_OFS_Kozijn")
ifcopenshell.api.run("pset.edit_pset", ifc_file, pset=ofs_pset,
    properties={{
        "Materiaal": str(kozijn["frame"].get("material", "")),
        "KleurBinnen": kozijn["frame"].get("colorInside", "RAL9010"),
        "KleurBuiten": kozijn["frame"].get("colorOutside", "RAL9010"),
        "AantalCellen": len(kozijn.get("cells", [])),
    }})

# Ververs de viewport
import blenderbim.tool as tool
tool.Ifc.set(ifc_file)
bpy.ops.bim.load_project_elements()

print(f"Kozijn '{{name}}' ({{mark}}) succesvol aangemaakt in Blender/Bonsai")
"#,
        json = kozijn_json.replace('\'', "\\'")
    );

    crate::blender::execute_code(&python_code).await
}

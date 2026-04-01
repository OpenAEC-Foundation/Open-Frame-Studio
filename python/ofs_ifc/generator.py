"""IFC generation for kozijnen (window frames).

Generates IFC4 files with IfcWindow/IfcDoor entities including proper
geometry (IfcExtrudedAreaSolid) and property sets.
"""

import ifcopenshell
import ifcopenshell.api
import ifcopenshell.util.placement

from .ils_properties import add_ils_property_sets


def generate_ifc(kozijn_data: dict, output_path: str):
    """Generate an IFC4 file from a kozijn definition."""
    model = ifcopenshell.file(schema="IFC4")

    # Project setup
    project = ifcopenshell.api.run("root.create_entity", model, ifc_class="IfcProject", name="Open Frame Studio Export")
    ifcopenshell.api.run("unit.assign_unit", model)

    # Context for 3D geometry
    context = ifcopenshell.api.run("context.add_context", model, context_type="Model")
    body = ifcopenshell.api.run(
        "context.add_context", model,
        context_type="Model",
        context_identifier="Body",
        target_view="MODEL_VIEW",
        parent=context,
    )

    # Spatial structure
    site = ifcopenshell.api.run("root.create_entity", model, ifc_class="IfcSite", name="Bouwlocatie")
    building = ifcopenshell.api.run("root.create_entity", model, ifc_class="IfcBuilding", name="Gebouw")
    storey = ifcopenshell.api.run("root.create_entity", model, ifc_class="IfcBuildingStorey", name="Begane grond")

    ifcopenshell.api.run("aggregate.assign_object", model, relating_object=project, products=[site])
    ifcopenshell.api.run("aggregate.assign_object", model, relating_object=site, products=[building])
    ifcopenshell.api.run("aggregate.assign_object", model, relating_object=building, products=[storey])

    # Determine if this is a door or window
    has_door = any(
        c.get("panelType") == "door" for c in kozijn_data.get("cells", [])
    )

    ifc_class = "IfcDoor" if has_door else "IfcWindow"
    frame = kozijn_data.get("frame", {})
    name = kozijn_data.get("name", "Kozijn")
    mark = kozijn_data.get("mark", "K01")

    # Dimensions in meters (IFC standard)
    width_m = frame.get("outerWidth", 1200) / 1000.0
    height_m = frame.get("outerHeight", 1500) / 1000.0
    depth_m = frame.get("frameDepth", 114) / 1000.0
    frame_width_m = frame.get("frameWidth", 67) / 1000.0

    # Create the window/door element
    element = ifcopenshell.api.run(
        "root.create_entity", model,
        ifc_class=ifc_class,
        name=name,
    )
    element.Tag = mark
    element.OverallWidth = width_m
    element.OverallHeight = height_m

    # Create geometry — simplified as an extruded rectangle for the overall frame
    outer_points = [
        (0.0, 0.0),
        (width_m, 0.0),
        (width_m, height_m),
        (0.0, height_m),
    ]
    inner_points = [
        (frame_width_m, frame_width_m),
        (width_m - frame_width_m, frame_width_m),
        (width_m - frame_width_m, height_m - frame_width_m),
        (frame_width_m, height_m - frame_width_m),
    ]

    # Outer profile
    outer_polyline = model.createIfcPolyline([
        model.createIfcCartesianPoint(p) for p in outer_points + [outer_points[0]]
    ])
    inner_polyline = model.createIfcPolyline([
        model.createIfcCartesianPoint(p) for p in inner_points + [inner_points[0]]
    ])

    profile = model.createIfcArbitraryProfileDefWithVoids(
        "AREA",
        "FrameProfile",
        outer_polyline,
        [inner_polyline],
    )

    # Extrude the frame profile
    direction = model.createIfcDirection((0.0, 0.0, 1.0))
    extrusion = model.createIfcExtrudedAreaSolid(
        profile,
        model.createIfcAxis2Placement3D(
            model.createIfcCartesianPoint((0.0, 0.0, 0.0)),
            model.createIfcDirection((0.0, 0.0, 1.0)),
            model.createIfcDirection((1.0, 0.0, 0.0)),
        ),
        direction,
        depth_m,
    )

    # Glass panel (simplified as a thin slab inside the frame)
    glass_points = [
        (frame_width_m, frame_width_m),
        (width_m - frame_width_m, frame_width_m),
        (width_m - frame_width_m, height_m - frame_width_m),
        (frame_width_m, height_m - frame_width_m),
    ]
    glass_polyline = model.createIfcPolyline([
        model.createIfcCartesianPoint(p) for p in glass_points + [glass_points[0]]
    ])
    glass_profile = model.createIfcArbitraryClosedProfileDef("AREA", "GlassProfile", glass_polyline)
    glass_thickness = 0.024  # 24mm HR++ glass
    glass_offset = (depth_m - glass_thickness) / 2.0

    glass_extrusion = model.createIfcExtrudedAreaSolid(
        glass_profile,
        model.createIfcAxis2Placement3D(
            model.createIfcCartesianPoint((0.0, 0.0, glass_offset)),
            model.createIfcDirection((0.0, 0.0, 1.0)),
            model.createIfcDirection((1.0, 0.0, 0.0)),
        ),
        direction,
        glass_thickness,
    )

    # Combine into shape representation
    shape = model.createIfcShapeRepresentation(
        body,
        "Body",
        "SweptSolid",
        [extrusion, glass_extrusion],
    )
    product_shape = model.createIfcProductDefinitionShape(None, None, [shape])
    element.Representation = product_shape

    # Place the element
    placement = model.createIfcLocalPlacement(
        None,
        model.createIfcAxis2Placement3D(
            model.createIfcCartesianPoint((0.0, 0.0, 0.0)),
            model.createIfcDirection((0.0, 0.0, 1.0)),
            model.createIfcDirection((1.0, 0.0, 0.0)),
        ),
    )
    element.ObjectPlacement = placement

    # Assign to storey
    ifcopenshell.api.run(
        "spatial.assign_container", model,
        relating_structure=storey,
        products=[element],
    )

    # Property sets
    _add_property_sets(model, element, kozijn_data)

    # ILS Houten Kozijnen v2.0 property sets
    add_ils_property_sets(model, element, kozijn_data)

    # Write file
    model.write(output_path)


def _add_property_sets(model, element, kozijn_data):
    """Add IFC property sets for the kozijn."""
    frame = kozijn_data.get("frame", {})
    cells = kozijn_data.get("cells", [])

    # Pset_WindowCommon / Pset_DoorCommon
    is_door = any(c.get("panelType") == "door" for c in cells)
    pset_name = "Pset_DoorCommon" if is_door else "Pset_WindowCommon"

    properties = {
        "Reference": kozijn_data.get("mark", "K01"),
        "IsExternal": True,
    }

    # Calculate glazing area fraction
    total_area = (frame.get("outerWidth", 1200) / 1000.0) * (frame.get("outerHeight", 1500) / 1000.0)
    fw = frame.get("frameWidth", 67) / 1000.0
    inner_w = frame.get("outerWidth", 1200) / 1000.0 - 2 * fw
    inner_h = frame.get("outerHeight", 1500) / 1000.0 - 2 * fw
    glass_area = inner_w * inner_h
    if total_area > 0:
        properties["GlazingAreaFraction"] = glass_area / total_area

    # Get Ug value from first cell with glazing
    for cell in cells:
        glazing = cell.get("glazing", {})
        ug = glazing.get("ugValue")
        if ug:
            properties["ThermalTransmittance"] = ug
            break

    # Create property set using ifcopenshell api
    pset = ifcopenshell.api.run("pset.add_pset", model, product=element, name=pset_name)
    ifcopenshell.api.run("pset.edit_pset", model, pset=pset, properties=properties)

    # Custom OFS property set
    ofs_props = {
        "Material": str(frame.get("material", "wood")),
        "ColorInside": frame.get("colorInside", "RAL9010"),
        "ColorOutside": frame.get("colorOutside", "RAL9010"),
        "FrameWidth_mm": frame.get("frameWidth", 67.0),
        "FrameDepth_mm": frame.get("frameDepth", 114.0),
        "CellCount": len(cells),
    }
    ofs_pset = ifcopenshell.api.run("pset.add_pset", model, product=element, name="Pset_OFS_Kozijn")
    ifcopenshell.api.run("pset.edit_pset", model, pset=ofs_pset, properties=ofs_props)

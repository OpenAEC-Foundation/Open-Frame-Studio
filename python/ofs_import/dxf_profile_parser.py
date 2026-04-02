"""Parse a DXF file containing a profile cross-section and extract a ProfileDefinition."""

import json
import math


def parse_dxf_profile(filepath):
    """Parse a DXF file and extract the profile cross-section.

    Reads a DXF with a 2D cross-section drawing and extracts:
    - Outer contour polygon (crossSection)
    - Bounding box for width/depth
    - Sponning (rabbet) detection from rectangular notches

    Args:
        filepath: Path to the DXF file.

    Returns:
        dict: A ProfileDefinition-compatible dict.
    """
    try:
        import ezdxf
    except ImportError:
        raise ImportError(
            "ezdxf is vereist voor DXF import. Installeer met: pip install ezdxf"
        )

    doc = ezdxf.readfile(filepath)
    msp = doc.modelspace()

    # Collect all line/polyline points
    points = []
    for entity in msp:
        if entity.dxftype() == "LINE":
            points.append((entity.dxf.start.x, entity.dxf.start.y))
            points.append((entity.dxf.end.x, entity.dxf.end.y))
        elif entity.dxftype() == "LWPOLYLINE":
            for pt in entity.get_points(format="xy"):
                points.append(pt)
        elif entity.dxftype() == "POLYLINE":
            for vertex in entity.vertices:
                points.append((vertex.dxf.location.x, vertex.dxf.location.y))

    if not points:
        raise ValueError("Geen geometrie gevonden in DXF bestand")

    # Build outer contour from convex hull or ordered points
    contour = _extract_contour(points)

    # Calculate bounding box
    xs = [p[0] for p in contour]
    ys = [p[1] for p in contour]
    min_x, max_x = min(xs), max(xs)
    min_y, max_y = min(ys), max(ys)
    width = round(max_x - min_x, 1)
    depth = round(max_y - min_y, 1)

    # Normalize contour to origin
    cross_section = [[round(p[0] - min_x, 2), round(p[1] - min_y, 2)] for p in contour]

    # Detect sponning (rabbet) — look for rectangular notches
    sponning = _detect_sponning(contour, width, depth)

    # Estimate sightline (visible face width after glazing)
    sightline = round(width * 0.8, 1)
    glazing_rebate = round(width * 0.36, 1)

    # Build profile name from filename
    import os
    name = os.path.splitext(os.path.basename(filepath))[0]
    profile_id = f"imported-{name.lower().replace(' ', '-')}"

    result = {
        "id": profile_id,
        "name": name,
        "material": "unknown",
        "materialSubtype": None,
        "width": width,
        "depth": depth,
        "sightline": sightline,
        "glazingRebate": glazing_rebate,
        "crossSection": cross_section,
        "ufValue": 2.0,  # default, user should adjust
        "applicableAs": ["frame", "sash", "divider"],
    }

    if sponning:
        result["sponning"] = sponning

    return result


def _extract_contour(points):
    """Extract an ordered contour from a set of points using convex hull."""
    if len(points) < 3:
        return points

    # Simple convex hull (Graham scan)
    points = list(set(points))  # Remove duplicates

    # Find lowest-leftmost point
    start = min(points, key=lambda p: (p[1], p[0]))
    points.remove(start)

    def polar_angle(p):
        dx = p[0] - start[0]
        dy = p[1] - start[1]
        return math.atan2(dy, dx)

    points.sort(key=polar_angle)

    hull = [start]
    for p in points:
        while len(hull) > 1 and _cross(hull[-2], hull[-1], p) <= 0:
            hull.pop()
        hull.append(p)

    return hull


def _cross(o, a, b):
    """Cross product of vectors OA and OB."""
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


def _detect_sponning(contour, width, depth):
    """Detect rectangular notches (sponningen) in the contour.

    Looks for step patterns that indicate a rabbet/rebate.
    Returns sponning info dict or None.
    """
    if len(contour) < 6:
        return None

    # Look for concave notches by finding right-angle steps
    for i in range(len(contour)):
        p0 = contour[i]
        p1 = contour[(i + 1) % len(contour)]
        p2 = contour[(i + 2) % len(contour)]

        # Check for L-shaped step (horizontal then vertical or vice versa)
        dx1 = abs(p1[0] - p0[0])
        dy1 = abs(p1[1] - p0[1])
        dx2 = abs(p2[0] - p1[0])
        dy2 = abs(p2[1] - p1[1])

        # Horizontal then vertical step = potential sponning
        if dx1 > 2 and dy1 < 1 and dx2 < 1 and dy2 > 2:
            sp_width = round(dx1, 1)
            sp_depth = round(dy2, 1)
            if 5 <= sp_width <= 30 and 5 <= sp_depth <= 40:
                # Determine position based on location
                mid_x = (p0[0] + p1[0]) / 2
                position = "buiten" if mid_x > width / 2 else "binnen"
                return {
                    "width": sp_width,
                    "depth": sp_depth,
                    "position": position,
                }

        # Vertical then horizontal step
        if dy1 > 2 and dx1 < 1 and dy2 < 1 and dx2 > 2:
            sp_width = round(dx2, 1)
            sp_depth = round(dy1, 1)
            if 5 <= sp_width <= 30 and 5 <= sp_depth <= 40:
                mid_y = (p0[1] + p1[1]) / 2
                position = "buiten" if mid_y > depth / 2 else "binnen"
                return {
                    "width": sp_width,
                    "depth": sp_depth,
                    "position": position,
                }

    return None

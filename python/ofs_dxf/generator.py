"""DXF workshop drawing generation for kozijnen."""

import ezdxf
from ezdxf.enums import TextEntityAlignment


def generate_dxf(kozijn_data: dict, output_path: str):
    """Generate a DXF workshop drawing from a kozijn definition."""
    doc = ezdxf.new("R2013")
    msp = doc.modelspace()

    # Setup layers
    doc.layers.add("FRAME", color=7)       # White (frame members)
    doc.layers.add("GLASS", color=5)       # Blue (glazing)
    doc.layers.add("DIMENSIONS", color=3)  # Green (dimension lines)
    doc.layers.add("TEXT", color=7)         # White (labels)
    doc.layers.add("SYMBOLS", color=1)     # Red (opening symbols)

    frame = kozijn_data.get("frame", {})
    grid = kozijn_data.get("grid", {})
    cells = kozijn_data.get("cells", [])

    ow = frame.get("outerWidth", 1200)
    oh = frame.get("outerHeight", 1500)
    fw = frame.get("frameWidth", 67)

    # Draw outer frame
    _draw_rect(msp, 0, 0, ow, oh, "FRAME")

    # Draw inner opening
    _draw_rect(msp, fw, fw, ow - fw, oh - fw, "FRAME")

    # Draw vertical dividers
    columns = grid.get("columns", [])
    x = fw
    for i, col in enumerate(columns):
        x += col.get("size", 0)
        if i < len(columns) - 1:
            _draw_rect(msp, x, fw, x + fw, oh - fw, "FRAME")
            x += fw

    # Draw horizontal dividers
    rows = grid.get("rows", [])
    y = fw
    for i, row in enumerate(rows):
        y += row.get("size", 0)
        if i < len(rows) - 1:
            _draw_rect(msp, fw, y, ow - fw, y + fw, "FRAME")
            y += fw

    # Draw cells (glass fill patterns)
    col_positions = []
    cx = fw
    for i, col in enumerate(columns):
        col_positions.append(cx)
        cx += col.get("size", 0)
        if i < len(columns) - 1:
            cx += fw

    row_positions = []
    ry = fw
    for i, row in enumerate(rows):
        row_positions.append(ry)
        ry += row.get("size", 0)
        if i < len(rows) - 1:
            ry += fw

    num_cols = len(columns)
    for row_idx, row in enumerate(rows):
        for col_idx, col in enumerate(columns):
            cell_idx = row_idx * num_cols + col_idx
            if cell_idx < len(cells):
                cell = cells[cell_idx]
                x1 = col_positions[col_idx]
                y1 = row_positions[row_idx]
                x2 = x1 + col.get("size", 0)
                y2 = y1 + row.get("size", 0)

                # Draw glass diagonals for glass panels
                panel_type = cell.get("panelType", "fixed_glass")
                if panel_type in ("fixed_glass", "turn_tilt", "turn", "tilt", "sliding"):
                    msp.add_line((x1, y1), (x2, y2), dxfattribs={"layer": "GLASS"})
                    msp.add_line((x1, y2), (x2, y1), dxfattribs={"layer": "GLASS"})

                # Draw opening symbols
                _draw_opening_symbol(msp, cell, x1, y1, x2, y2)

                # Cell label
                label = _panel_label(panel_type)
                mid_x = (x1 + x2) / 2
                mid_y = (y1 + y2) / 2
                msp.add_text(
                    label,
                    height=min((x2 - x1), (y2 - y1)) * 0.12,
                    dxfattribs={"layer": "TEXT"},
                ).set_placement((mid_x, mid_y), align=TextEntityAlignment.MIDDLE_CENTER)

    # Overall dimensions
    dim_offset = 80

    # Width dimension (bottom)
    msp.add_linear_dim(
        base=(ow / 2, -dim_offset),
        p1=(0, 0),
        p2=(ow, 0),
        dimstyle="EZDXF",
        override={"dimtxt": 30},
    ).render()

    # Height dimension (right)
    msp.add_linear_dim(
        base=(ow + dim_offset, oh / 2),
        p1=(ow, 0),
        p2=(ow, oh),
        angle=90,
        dimstyle="EZDXF",
        override={"dimtxt": 30},
    ).render()

    # Title block
    mark = kozijn_data.get("mark", "K01")
    name = kozijn_data.get("name", "Kozijn")
    msp.add_text(
        f"{mark} - {name}",
        height=40,
        dxfattribs={"layer": "TEXT"},
    ).set_placement((0, -dim_offset * 3), align=TextEntityAlignment.LEFT)

    msp.add_text(
        f"{ow} x {oh} mm",
        height=25,
        dxfattribs={"layer": "TEXT"},
    ).set_placement((0, -dim_offset * 3 - 50), align=TextEntityAlignment.LEFT)

    doc.saveas(output_path)


def _draw_rect(msp, x1, y1, x2, y2, layer):
    """Draw a rectangle as four lines."""
    msp.add_line((x1, y1), (x2, y1), dxfattribs={"layer": layer})
    msp.add_line((x2, y1), (x2, y2), dxfattribs={"layer": layer})
    msp.add_line((x2, y2), (x1, y2), dxfattribs={"layer": layer})
    msp.add_line((x1, y2), (x1, y1), dxfattribs={"layer": layer})


def _draw_opening_symbol(msp, cell, x1, y1, x2, y2):
    """Draw opening direction symbols for operable panels."""
    panel_type = cell.get("panelType", "fixed_glass")
    direction = cell.get("openingDirection")

    if panel_type in ("turn_tilt", "turn"):
        mid_y = (y1 + y2) / 2
        if direction == "left" or direction is None:
            msp.add_line((x1, y1), (x2, mid_y), dxfattribs={"layer": "SYMBOLS", "linetype": "DASHED"})
            msp.add_line((x1, y2), (x2, mid_y), dxfattribs={"layer": "SYMBOLS", "linetype": "DASHED"})
        else:
            msp.add_line((x2, y1), (x1, mid_y), dxfattribs={"layer": "SYMBOLS", "linetype": "DASHED"})
            msp.add_line((x2, y2), (x1, mid_y), dxfattribs={"layer": "SYMBOLS", "linetype": "DASHED"})

    elif panel_type == "tilt":
        mid_x = (x1 + x2) / 2
        msp.add_line((x1, y2), (mid_x, y1), dxfattribs={"layer": "SYMBOLS", "linetype": "DASHED"})
        msp.add_line((x2, y2), (mid_x, y1), dxfattribs={"layer": "SYMBOLS", "linetype": "DASHED"})

    elif panel_type == "door":
        mid_x = (x1 + x2) / 2
        # Door swing arc (simplified as triangle)
        if direction == "left" or direction == "inward" or direction is None:
            msp.add_line((x1, y1), (x2, y2), dxfattribs={"layer": "SYMBOLS"})
        else:
            msp.add_line((x2, y1), (x1, y2), dxfattribs={"layer": "SYMBOLS"})


def _panel_label(panel_type):
    """Return abbreviated label for panel type."""
    labels = {
        "fixed_glass": "VG",
        "turn_tilt": "DK",
        "turn": "D",
        "tilt": "K",
        "sliding": "S",
        "door": "DR",
        "panel": "P",
        "ventilation": "V",
    }
    return labels.get(panel_type, "?")

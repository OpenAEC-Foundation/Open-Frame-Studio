"""Workshop drawing (werkplaatstekening) generation for individual kozijnen.

Generates a PDF with:
- Front elevation view with dimensions
- Cross-section details
- Cell labels and opening symbols
- Title block with project info
"""

from reportlab.lib.pagesizes import A3, landscape
from reportlab.lib import colors
from reportlab.lib.units import mm
from reportlab.pdfgen import canvas


# OpenAEC brand colors
AMBER = colors.HexColor("#D97706")
DEEP_FORGE = colors.HexColor("#36363E")
LIGHT_GRAY = colors.HexColor("#E7E5E4")

PANEL_LABELS = {
    "fixed_glass": "VG",
    "turn_tilt": "DK",
    "turn": "D",
    "tilt": "K",
    "sliding": "S",
    "door": "DR",
    "panel": "P",
    "ventilation": "V",
}


def generate_workshop_drawing(kozijn_data: dict, project_data: dict, output_path: str):
    """Generate a workshop drawing PDF for a single kozijn."""
    c = canvas.Canvas(output_path, pagesize=landscape(A3))
    page_w, page_h = landscape(A3)

    # Margins
    margin = 15 * mm
    title_block_h = 40 * mm
    drawing_area_w = page_w - 2 * margin
    drawing_area_h = page_h - 2 * margin - title_block_h

    # Draw border
    c.setStrokeColor(DEEP_FORGE)
    c.setLineWidth(1.5)
    c.rect(margin, margin, drawing_area_w, page_h - 2 * margin)

    # Title block
    _draw_title_block(c, kozijn_data, project_data, margin, margin, drawing_area_w, title_block_h)

    # Calculate scale to fit kozijn in drawing area
    frame = kozijn_data.get("frame", {})
    ow = frame.get("outerWidth", 1200)
    oh = frame.get("outerHeight", 1500)

    # Leave room for dimensions (60mm on each side)
    dim_margin = 60 * mm
    available_w = drawing_area_w - 2 * dim_margin
    available_h = drawing_area_h - 2 * dim_margin

    scale_x = available_w / ow
    scale_y = available_h / oh
    scale = min(scale_x, scale_y) * 0.85  # 85% to leave breathing room

    # Center the drawing
    kozijn_w_scaled = ow * scale
    kozijn_h_scaled = oh * scale
    origin_x = margin + dim_margin + (available_w - kozijn_w_scaled) / 2
    origin_y = margin + title_block_h + dim_margin + (available_h - kozijn_h_scaled) / 2

    # Draw the kozijn
    _draw_kozijn_elevation(c, kozijn_data, origin_x, origin_y, scale)

    # Draw dimensions
    _draw_dimensions(c, kozijn_data, origin_x, origin_y, scale)

    # Draw scale indicator
    scale_text = f"Schaal 1:{int(1/scale)}" if scale < 1 else f"Schaal {scale:.1f}:1"
    c.setFont("Helvetica", 8)
    c.setFillColor(colors.gray)
    c.drawString(margin + 5 * mm, margin + title_block_h + 5 * mm, scale_text)

    c.save()


def _draw_title_block(c, kozijn_data, project_data, x, y, width, height):
    """Draw the title block at the bottom of the drawing."""
    frame = kozijn_data.get("frame", {})
    project_info = project_data.get("projectInfo", {})

    # Background
    c.setFillColor(DEEP_FORGE)
    c.rect(x, y, width, height, fill=1, stroke=0)

    # Amber accent line at top
    c.setStrokeColor(AMBER)
    c.setLineWidth(3)
    c.line(x, y + height, x + width, y + height)

    # Logo area (left)
    c.setFillColor(colors.white)
    c.setFont("Helvetica-Bold", 14)
    c.drawString(x + 8 * mm, y + height - 14 * mm, "Open Frame Studio")
    c.setFont("Helvetica", 8)
    c.setFillColor(AMBER)
    c.drawString(x + 8 * mm, y + height - 20 * mm, "OpenAEC Foundation")

    # Divider line
    div_x = x + 80 * mm
    c.setStrokeColor(colors.HexColor("#555555"))
    c.setLineWidth(0.5)
    c.line(div_x, y + 5 * mm, div_x, y + height - 5 * mm)

    # Project info (middle-left)
    info_x = div_x + 8 * mm
    c.setFillColor(colors.HexColor("#999999"))
    c.setFont("Helvetica", 7)
    c.drawString(info_x, y + height - 10 * mm, "PROJECT")
    c.setFillColor(colors.white)
    c.setFont("Helvetica-Bold", 10)
    c.drawString(info_x, y + height - 18 * mm, project_info.get("name", "-"))
    c.setFont("Helvetica", 8)
    c.setFillColor(colors.HexColor("#CCCCCC"))
    c.drawString(info_x, y + height - 25 * mm, f"Nr: {project_info.get('number', '-')}")
    c.drawString(info_x, y + height - 32 * mm, f"Opdrachtgever: {project_info.get('client', '-')}")

    # Divider
    div_x2 = x + 180 * mm
    c.setStrokeColor(colors.HexColor("#555555"))
    c.line(div_x2, y + 5 * mm, div_x2, y + height - 5 * mm)

    # Kozijn info (middle-right)
    koz_x = div_x2 + 8 * mm
    c.setFillColor(colors.HexColor("#999999"))
    c.setFont("Helvetica", 7)
    c.drawString(koz_x, y + height - 10 * mm, "KOZIJN")
    c.setFillColor(AMBER)
    c.setFont("Helvetica-Bold", 16)
    c.drawString(koz_x, y + height - 20 * mm, kozijn_data.get("mark", "K01"))
    c.setFillColor(colors.white)
    c.setFont("Helvetica", 9)
    c.drawString(koz_x + 30 * mm, y + height - 18 * mm, kozijn_data.get("name", "Kozijn"))

    c.setFillColor(colors.HexColor("#CCCCCC"))
    c.setFont("Helvetica", 8)
    c.drawString(koz_x, y + height - 28 * mm,
                 f"{int(frame.get('outerWidth', 0))} x {int(frame.get('outerHeight', 0))} mm")

    material = frame.get("material", {})
    mat_str = _material_label(material)
    c.drawString(koz_x, y + height - 35 * mm, f"Materiaal: {mat_str}")

    # Divider
    div_x3 = x + 300 * mm
    c.setStrokeColor(colors.HexColor("#555555"))
    c.line(div_x3, y + 5 * mm, div_x3, y + height - 5 * mm)

    # Details (right)
    det_x = div_x3 + 8 * mm
    c.setFillColor(colors.HexColor("#999999"))
    c.setFont("Helvetica", 7)
    c.drawString(det_x, y + height - 10 * mm, "DETAILS")

    c.setFillColor(colors.HexColor("#CCCCCC"))
    c.setFont("Helvetica", 8)
    cells = kozijn_data.get("cells", [])
    c.drawString(det_x, y + height - 18 * mm, f"Cellen: {len(cells)}")
    c.drawString(det_x, y + height - 25 * mm, f"Profiel: {frame.get('frameWidth', 67)}x{frame.get('frameDepth', 114)} mm")
    c.drawString(det_x, y + height - 32 * mm, f"Kleur: {frame.get('colorInside', 'RAL9010')} / {frame.get('colorOutside', 'RAL9010')}")

    # Date (far right)
    import datetime
    c.setFillColor(colors.HexColor("#999999"))
    c.setFont("Helvetica", 7)
    c.drawRightString(x + width - 8 * mm, y + height - 10 * mm, "DATUM")
    c.setFillColor(colors.HexColor("#CCCCCC"))
    c.setFont("Helvetica", 8)
    c.drawRightString(x + width - 8 * mm, y + height - 18 * mm,
                      datetime.date.today().strftime("%d-%m-%Y"))
    c.drawRightString(x + width - 8 * mm, y + height - 25 * mm, "WERKPLAATS TEKENING")


def _draw_kozijn_elevation(c, kozijn_data, origin_x, origin_y, scale):
    """Draw the front elevation of the kozijn."""
    frame = kozijn_data.get("frame", {})
    grid = kozijn_data.get("grid", {})
    cells = kozijn_data.get("cells", [])

    ow = frame.get("outerWidth", 1200)
    oh = frame.get("outerHeight", 1500)
    fw = frame.get("frameWidth", 67)

    def sx(v):
        return origin_x + v * scale

    def sy(v):
        return origin_y + v * scale

    # Outer frame rectangle
    c.setStrokeColor(DEEP_FORGE)
    c.setLineWidth(2)
    c.rect(sx(0), sy(0), ow * scale, oh * scale)

    # Frame members (hatched fill)
    c.setFillColor(LIGHT_GRAY)
    c.setStrokeColor(DEEP_FORGE)
    c.setLineWidth(1)

    # Top frame
    c.rect(sx(0), sy(oh - fw), ow * scale, fw * scale, fill=1)
    # Bottom frame (sill)
    c.rect(sx(0), sy(0), ow * scale, fw * scale, fill=1)
    # Left frame
    c.rect(sx(0), sy(fw), fw * scale, (oh - 2 * fw) * scale, fill=1)
    # Right frame
    c.rect(sx(ow - fw), sy(fw), fw * scale, (oh - 2 * fw) * scale, fill=1)

    # Calculate cell positions
    columns = grid.get("columns", [{"size": ow - 2 * fw}])
    rows = grid.get("rows", [{"size": oh - 2 * fw}])
    dw = fw  # divider width

    col_positions = []
    cx = fw
    for i, col in enumerate(columns):
        col_positions.append(cx)
        cx += col.get("size", 500)
        if i < len(columns) - 1:
            cx += dw

    row_positions = []
    ry = fw
    for i, row in enumerate(rows):
        row_positions.append(ry)
        ry += row.get("size", 500)
        if i < len(rows) - 1:
            ry += dw

    # Draw vertical dividers
    vx = fw
    for i, col in enumerate(columns):
        vx += col.get("size", 500)
        if i < len(columns) - 1:
            c.setFillColor(LIGHT_GRAY)
            c.rect(sx(vx), sy(fw), dw * scale, (oh - 2 * fw) * scale, fill=1)
            vx += dw

    # Draw horizontal dividers
    hy = fw
    for i, row in enumerate(rows):
        hy += row.get("size", 500)
        if i < len(rows) - 1:
            c.setFillColor(LIGHT_GRAY)
            c.rect(sx(fw), sy(hy), (ow - 2 * fw) * scale, dw * scale, fill=1)
            hy += dw

    # Draw cell contents
    num_cols = len(columns)
    for ri, row in enumerate(rows):
        for ci, col in enumerate(columns):
            cell_idx = ri * num_cols + ci
            if cell_idx >= len(cells):
                continue

            cell = cells[cell_idx]
            cell_x = col_positions[ci]
            cell_y = row_positions[ri]
            cell_w = col.get("size", 500)
            cell_h = row.get("size", 500)

            panel_type = cell.get("panelType", "fixed_glass")

            # Glass cross for fixed glass
            if panel_type == "fixed_glass":
                c.setStrokeColor(colors.HexColor("#93C5FD"))
                c.setLineWidth(0.5)
                c.line(sx(cell_x), sy(cell_y), sx(cell_x + cell_w), sy(cell_y + cell_h))
                c.line(sx(cell_x + cell_w), sy(cell_y), sx(cell_x), sy(cell_y + cell_h))

            # Opening triangle for turn/turn_tilt
            elif panel_type in ("turn_tilt", "turn"):
                c.setStrokeColor(colors.HexColor("#3B82F6"))
                c.setLineWidth(0.8)
                c.setDash(3, 2)
                cx_mid = cell_x + cell_w / 2
                cy_mid = cell_y + cell_h / 2
                direction = cell.get("openingDirection", "left")
                if direction == "left":
                    p = c.beginPath()
                    p.moveTo(sx(cell_x), sy(cell_y))
                    p.lineTo(sx(cell_x + cell_w), sy(cy_mid))
                    p.lineTo(sx(cell_x), sy(cell_y + cell_h))
                    p.close()
                    c.drawPath(p, stroke=1, fill=0)
                else:
                    p = c.beginPath()
                    p.moveTo(sx(cell_x + cell_w), sy(cell_y))
                    p.lineTo(sx(cell_x), sy(cy_mid))
                    p.lineTo(sx(cell_x + cell_w), sy(cell_y + cell_h))
                    p.close()
                    c.drawPath(p, stroke=1, fill=0)
                c.setDash()

            # Tilt triangle
            elif panel_type == "tilt":
                c.setStrokeColor(colors.HexColor("#818CF8"))
                c.setLineWidth(0.8)
                c.setDash(3, 2)
                cx_mid = cell_x + cell_w / 2
                p = c.beginPath()
                p.moveTo(sx(cell_x), sy(cell_y))
                p.lineTo(sx(cx_mid), sy(cell_y + cell_h))
                p.lineTo(sx(cell_x + cell_w), sy(cell_y))
                p.close()
                c.drawPath(p, stroke=1, fill=0)
                c.setDash()

            # Door symbol
            elif panel_type == "door":
                c.setStrokeColor(colors.HexColor("#F97316"))
                c.setLineWidth(1)
                c.circle(sx(cell_x + cell_w * 0.8), sy(cell_y + cell_h * 0.5), 3 * scale, fill=0)

            # Sliding arrow
            elif panel_type == "sliding":
                c.setStrokeColor(colors.HexColor("#34D399"))
                c.setLineWidth(1)
                mid_y = cell_y + cell_h / 2
                c.line(sx(cell_x + cell_w * 0.3), sy(mid_y),
                       sx(cell_x + cell_w * 0.7), sy(mid_y))
                # Arrow head
                c.line(sx(cell_x + cell_w * 0.6), sy(mid_y - cell_h * 0.05),
                       sx(cell_x + cell_w * 0.7), sy(mid_y))
                c.line(sx(cell_x + cell_w * 0.6), sy(mid_y + cell_h * 0.05),
                       sx(cell_x + cell_w * 0.7), sy(mid_y))

            # Panel fill
            elif panel_type == "panel":
                c.setFillColor(colors.HexColor("#E7E5E4"))
                c.rect(sx(cell_x + 2), sy(cell_y + 2),
                       (cell_w - 4) * scale, (cell_h - 4) * scale, fill=1, stroke=0)

            # Cell label
            label = PANEL_LABELS.get(panel_type, "")
            if label:
                c.setFillColor(DEEP_FORGE)
                c.setFont("Helvetica-Bold", max(8, min(14, int(cell_w * scale / 6))))
                c.drawCentredString(
                    sx(cell_x + cell_w / 2),
                    sy(cell_y + cell_h / 2) - 3,
                    label,
                )


def _draw_dimensions(c, kozijn_data, origin_x, origin_y, scale):
    """Draw dimension lines around the kozijn."""
    frame = kozijn_data.get("frame", {})
    grid = kozijn_data.get("grid", {})

    ow = frame.get("outerWidth", 1200)
    oh = frame.get("outerHeight", 1500)
    fw = frame.get("frameWidth", 67)

    def sx(v):
        return origin_x + v * scale

    def sy(v):
        return origin_y + v * scale

    dim_offset = 15 * mm
    tick_len = 3 * mm

    c.setStrokeColor(DEEP_FORGE)
    c.setLineWidth(0.5)
    c.setFillColor(DEEP_FORGE)
    c.setFont("Helvetica", 7)

    # Bottom dimension - overall width
    y_dim = sy(0) - dim_offset
    c.line(sx(0), y_dim, sx(ow), y_dim)
    c.line(sx(0), y_dim - tick_len, sx(0), y_dim + tick_len)
    c.line(sx(ow), y_dim - tick_len, sx(ow), y_dim + tick_len)
    # Extension lines
    c.setDash(1, 2)
    c.line(sx(0), sy(0), sx(0), y_dim - tick_len)
    c.line(sx(ow), sy(0), sx(ow), y_dim - tick_len)
    c.setDash()
    c.drawCentredString(sx(ow / 2), y_dim - 5 * mm, f"{int(ow)}")

    # Bottom dimension - column widths
    columns = grid.get("columns", [])
    if len(columns) > 1:
        y_dim2 = y_dim - 12 * mm
        col_x = fw
        for i, col in enumerate(columns):
            col_w = col.get("size", 500)
            c.line(sx(col_x), y_dim2, sx(col_x + col_w), y_dim2)
            c.line(sx(col_x), y_dim2 - tick_len, sx(col_x), y_dim2 + tick_len)
            c.line(sx(col_x + col_w), y_dim2 - tick_len, sx(col_x + col_w), y_dim2 + tick_len)
            c.drawCentredString(sx(col_x + col_w / 2), y_dim2 - 4 * mm, f"{int(col_w)}")
            col_x += col_w
            if i < len(columns) - 1:
                col_x += fw

    # Right dimension - overall height
    x_dim = sx(ow) + dim_offset
    c.line(x_dim, sy(0), x_dim, sy(oh))
    c.line(x_dim - tick_len, sy(0), x_dim + tick_len, sy(0))
    c.line(x_dim - tick_len, sy(oh), x_dim + tick_len, sy(oh))
    c.setDash(1, 2)
    c.line(sx(ow), sy(0), x_dim - tick_len, sy(0))
    c.line(sx(ow), sy(oh), x_dim - tick_len, sy(oh))
    c.setDash()
    c.saveState()
    c.translate(x_dim + 5 * mm, sy(oh / 2))
    c.rotate(90)
    c.drawCentredString(0, 0, f"{int(oh)}")
    c.restoreState()

    # Right dimension - row heights
    rows = grid.get("rows", [])
    if len(rows) > 1:
        x_dim2 = x_dim + 12 * mm
        row_y = fw
        for i, row in enumerate(rows):
            row_h = row.get("size", 500)
            c.line(x_dim2, sy(row_y), x_dim2, sy(row_y + row_h))
            c.line(x_dim2 - tick_len, sy(row_y), x_dim2 + tick_len, sy(row_y))
            c.line(x_dim2 - tick_len, sy(row_y + row_h), x_dim2 + tick_len, sy(row_y + row_h))
            c.saveState()
            c.translate(x_dim2 + 4 * mm, sy(row_y + row_h / 2))
            c.rotate(90)
            c.drawCentredString(0, 0, f"{int(row_h)}")
            c.restoreState()
            row_y += row_h
            if i < len(rows) - 1:
                row_y += fw


def _material_label(material):
    """Convert material dict/string to readable label."""
    if isinstance(material, dict):
        if "wood" in material:
            return f"Hout ({material['wood']})"
        elif "aluminum" in material:
            return "Aluminium"
        elif "pvc" in material:
            return "Kunststof"
        elif "woodAluminum" in material:
            return "Hout-Aluminium"
        return str(list(material.values())[0]) if material else "Onbekend"
    return str(material)

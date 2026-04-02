"""Parse profile catalogs from JSON or Excel files."""

import json
import os


# Default column mappings for known suppliers
SUPPLIER_MAPPINGS = {
    "reynaers": {
        "name": "Description",
        "width": "Width (mm)",
        "depth": "Depth (mm)",
        "sightline": "Sightline (mm)",
        "glazingRebate": "Glazing Rebate (mm)",
        "ufValue": "Uf (W/m²K)",
        "material": "Material",
        "sponningWidth": "Rebate Width (mm)",
        "sponningDepth": "Rebate Depth (mm)",
    },
    "schuco": {
        "name": "Bezeichnung",
        "width": "Ansichtsbreite",
        "depth": "Bautiefe",
        "sightline": "Ansichtsbreite Fl.",
        "glazingRebate": "Falzmaß",
        "ufValue": "Uf",
        "material": "Werkstoff",
        "sponningWidth": "Falzbreite",
        "sponningDepth": "Falztiefe",
    },
    "gealan": {
        "name": "Profilname",
        "width": "Breite",
        "depth": "Tiefe",
        "sightline": "Ansichtsbreite",
        "glazingRebate": "Glasfalz",
        "ufValue": "Uf-Wert",
        "material": "Material",
        "sponningWidth": "Falzbreite",
        "sponningDepth": "Falztiefe",
    },
    "generic": {
        "name": "name",
        "width": "width",
        "depth": "depth",
        "sightline": "sightline",
        "glazingRebate": "glazingRebate",
        "ufValue": "ufValue",
        "material": "material",
        "sponningWidth": "sponningWidth",
        "sponningDepth": "sponningDepth",
    },
}


def parse_catalog(filepath, supplier=None):
    """Parse a profile catalog file (JSON or Excel).

    Args:
        filepath: Path to the catalog file.
        supplier: Optional supplier key for column mapping (reynaers, schuco, gealan).
                  If None, auto-detect from file content.

    Returns:
        list[dict]: List of ProfileDefinition-compatible dicts.
    """
    ext = os.path.splitext(filepath)[1].lower()

    if ext == ".json":
        return _parse_json_catalog(filepath, supplier)
    elif ext in (".xlsx", ".xls"):
        return _parse_excel_catalog(filepath, supplier)
    elif ext == ".csv":
        return _parse_csv_catalog(filepath, supplier)
    else:
        raise ValueError(f"Onbekend bestandsformaat: {ext}. Ondersteund: .json, .xlsx, .xls, .csv")


def _parse_json_catalog(filepath, supplier):
    """Parse a JSON catalog file."""
    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)

    # Handle both array and object with profiles key
    if isinstance(data, dict):
        profiles = data.get("profiles", data.get("items", [data]))
    elif isinstance(data, list):
        profiles = data
    else:
        raise ValueError("Onverwacht JSON formaat")

    mapping = _get_mapping(supplier or "generic")
    return [_map_profile(p, mapping, i) for i, p in enumerate(profiles)]


def _parse_excel_catalog(filepath, supplier):
    """Parse an Excel catalog file."""
    try:
        import openpyxl
    except ImportError:
        raise ImportError(
            "openpyxl is vereist voor Excel import. Installeer met: pip install openpyxl"
        )

    wb = openpyxl.load_workbook(filepath, read_only=True, data_only=True)
    ws = wb.active

    rows = list(ws.iter_rows(values_only=True))
    if len(rows) < 2:
        raise ValueError("Excel bestand bevat geen data (minstens header + 1 rij nodig)")

    headers = [str(h).strip() if h else "" for h in rows[0]]

    # Auto-detect supplier from headers if not specified
    if not supplier:
        supplier = _detect_supplier(headers)

    mapping = _get_mapping(supplier)

    profiles = []
    for i, row in enumerate(rows[1:]):
        row_dict = {headers[j]: row[j] for j in range(len(headers)) if j < len(row)}
        try:
            profile = _map_profile(row_dict, mapping, i)
            profiles.append(profile)
        except (ValueError, KeyError):
            continue  # Skip invalid rows

    wb.close()
    return profiles


def _parse_csv_catalog(filepath, supplier):
    """Parse a CSV catalog file."""
    import csv

    with open(filepath, "r", encoding="utf-8-sig") as f:
        reader = csv.DictReader(f, delimiter=";")
        rows = list(reader)

    if not rows:
        raise ValueError("CSV bestand is leeg")

    # Try comma delimiter if semicolon gave only one column
    if len(rows[0]) <= 1:
        with open(filepath, "r", encoding="utf-8-sig") as f:
            reader = csv.DictReader(f, delimiter=",")
            rows = list(reader)

    headers = list(rows[0].keys())
    if not supplier:
        supplier = _detect_supplier(headers)

    mapping = _get_mapping(supplier)
    return [_map_profile(row, mapping, i) for i, row in enumerate(rows)]


def _get_mapping(supplier):
    """Get the column mapping for a supplier."""
    return SUPPLIER_MAPPINGS.get(supplier, SUPPLIER_MAPPINGS["generic"])


def _detect_supplier(headers):
    """Auto-detect supplier from column headers."""
    header_set = set(h.lower() for h in headers)

    if any("bezeichnung" in h for h in header_set):
        return "schuco"
    if any("profilname" in h for h in header_set):
        return "gealan"
    if any("description" in h for h in header_set):
        return "reynaers"
    return "generic"


def _map_profile(data, mapping, index):
    """Map a raw data row to a ProfileDefinition dict."""
    def get_val(key, default=None):
        mapped_key = mapping.get(key, key)
        val = data.get(mapped_key)
        if val is None:
            # Try case-insensitive match
            for k, v in data.items():
                if k.lower() == mapped_key.lower():
                    return v
            return default
        return val

    def get_float(key, default=0.0):
        val = get_val(key, default)
        if val is None:
            return default
        try:
            return float(val)
        except (ValueError, TypeError):
            return default

    name = str(get_val("name", f"Profiel {index + 1}"))
    width = get_float("width")
    depth = get_float("depth")

    if width <= 0 or depth <= 0:
        raise ValueError(f"Ongeldige afmetingen: {width}x{depth}")

    profile_id = f"imported-{name.lower().replace(' ', '-').replace('/', '-')[:40]}-{index}"

    result = {
        "id": profile_id,
        "name": name,
        "material": str(get_val("material", "unknown")).lower(),
        "materialSubtype": None,
        "width": width,
        "depth": depth,
        "sightline": get_float("sightline", round(width * 0.8, 1)),
        "glazingRebate": get_float("glazingRebate", round(width * 0.36, 1)),
        "crossSection": [],
        "ufValue": get_float("ufValue", 2.0),
        "applicableAs": ["frame", "sash", "divider"],
    }

    # Add sponning if available
    sp_width = get_float("sponningWidth")
    sp_depth = get_float("sponningDepth")
    if sp_width > 0 and sp_depth > 0:
        result["sponning"] = {
            "width": sp_width,
            "depth": sp_depth,
            "position": "buiten",
        }

    return result

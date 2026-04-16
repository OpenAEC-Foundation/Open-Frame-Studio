use printpdf::*;
use crate::kozijn::Project;
use crate::pricing::QuotationPrice;

pub struct CompanyInfo {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub kvk: String,
    pub btw_id: String,
}

fn font() -> PdfFontHandle { PdfFontHandle::Builtin(BuiltinFont::Helvetica) }
fn font_bold() -> PdfFontHandle { PdfFontHandle::Builtin(BuiltinFont::HelveticaBold) }

fn black() -> Color {
    Color::Rgb(Rgb { r: 0.0, g: 0.0, b: 0.0, icc_profile: None })
}

fn text(s: &str, size: f32, x: f32, y: f32, f: PdfFontHandle) -> Vec<Op> {
    vec![
        Op::SetFillColor { col: black() },
        Op::StartTextSection,
        Op::SetFont { font: f, size: Pt(size) },
        Op::SetTextCursor { pos: Point { x: Mm(x).into(), y: Mm(y).into() } },
        Op::ShowText { items: vec![TextItem::Text(s.to_string())] },
        Op::EndTextSection,
    ]
}

pub fn generate_quotation_pdf(
    project: &Project,
    company: &CompanyInfo,
    prices: &[(String, f64)],  // (kozijn_mark, subtotal)
    quotation_price: &QuotationPrice,
    terms: &str,
) -> Result<Vec<u8>, String> {
    let mut ops: Vec<Op> = Vec::new();

    // Company header
    ops.extend(text(&company.name, 16.0, 20.0, 275.0, font_bold()));
    ops.extend(text(&company.address, 9.0, 20.0, 268.0, font()));
    ops.extend(text(&format!("Tel: {} | E-mail: {}", company.phone, company.email), 8.0, 20.0, 263.0, font()));
    ops.extend(text(&format!("KvK: {} | BTW: {}", company.kvk, company.btw_id), 8.0, 20.0, 258.0, font()));

    // Title
    ops.extend(text("OFFERTE", 14.0, 20.0, 245.0, font_bold()));
    ops.extend(text(&format!("Project: {} ({})", project.project_info.name, project.project_info.number), 10.0, 20.0, 238.0, font()));
    ops.extend(text(&format!("Klant: {}", project.project_info.client), 10.0, 20.0, 232.0, font()));

    // Line items
    let mut y: f32 = 215.0;
    ops.extend(text("Merk", 9.0, 20.0, y, font_bold()));
    ops.extend(text("Bedrag", 9.0, 160.0, y, font_bold()));
    y -= 6.0;

    for (mark, total) in prices {
        ops.extend(text(mark, 9.0, 20.0, y, font()));
        ops.extend(text(&format!("\u{20AC} {:.2}", total), 9.0, 160.0, y, font()));
        y -= 5.0;
    }

    // Totals
    y -= 5.0;
    ops.extend(text(&format!("Subtotaal: \u{20AC} {:.2}", quotation_price.subtotal), 9.0, 130.0, y, font()));
    y -= 5.0;
    if quotation_price.discount_amount > 0.0 {
        ops.extend(text(&format!("Korting: -\u{20AC} {:.2}", quotation_price.discount_amount), 9.0, 130.0, y, font()));
        y -= 5.0;
    }
    if quotation_price.transport > 0.0 {
        ops.extend(text(&format!("Transport: \u{20AC} {:.2}", quotation_price.transport), 9.0, 130.0, y, font()));
        y -= 5.0;
    }
    if quotation_price.montage > 0.0 {
        ops.extend(text(&format!("Montage: \u{20AC} {:.2}", quotation_price.montage), 9.0, 130.0, y, font()));
        y -= 5.0;
    }
    ops.extend(text(&format!("BTW: \u{20AC} {:.2}", quotation_price.btw_amount), 9.0, 130.0, y, font()));
    y -= 6.0;
    ops.extend(text(&format!("TOTAAL INCL. BTW: \u{20AC} {:.2}", quotation_price.total_incl_btw), 11.0, 130.0, y, font_bold()));

    // Terms
    y -= 15.0;
    ops.extend(text("Voorwaarden:", 9.0, 20.0, y, font_bold()));
    y -= 5.0;
    for line in terms.lines().take(5) {
        ops.extend(text(line, 8.0, 20.0, y, font()));
        y -= 4.0;
    }

    // Build document
    let title = format!("Offerte - {}", project.project_info.name);
    let mut doc = PdfDocument::new(&title);
    doc.pages.push(PdfPage::new(Mm(210.0), Mm(297.0), ops));

    let mut warnings = Vec::new();
    let bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);
    Ok(bytes)
}

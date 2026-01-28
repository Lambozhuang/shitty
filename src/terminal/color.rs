use eframe::egui;

pub(crate) const DEFAULT_FG: egui::Color32 = egui::Color32::WHITE;
pub(crate) const DEFAULT_BG: egui::Color32 = egui::Color32::BLACK;

#[derive(Clone, Copy)]
pub(crate) enum ColorKind {
    Default,
    Ansi(u8),
    Xterm(u8),
    Rgb(egui::Color32),
}

pub(crate) fn ansi_16_color(index: u8) -> egui::Color32 {
    match index {
        0 => egui::Color32::from_rgb(0, 0, 0),
        1 => egui::Color32::from_rgb(128, 0, 0),
        2 => egui::Color32::from_rgb(0, 128, 0),
        3 => egui::Color32::from_rgb(128, 128, 0),
        4 => egui::Color32::from_rgb(0, 0, 128),
        5 => egui::Color32::from_rgb(128, 0, 128),
        6 => egui::Color32::from_rgb(0, 128, 128),
        7 => egui::Color32::from_rgb(192, 192, 192),
        8 => egui::Color32::from_rgb(128, 128, 128),
        9 => egui::Color32::from_rgb(255, 0, 0),
        10 => egui::Color32::from_rgb(0, 255, 0),
        11 => egui::Color32::from_rgb(255, 255, 0),
        12 => egui::Color32::from_rgb(0, 0, 255),
        13 => egui::Color32::from_rgb(255, 0, 255),
        14 => egui::Color32::from_rgb(0, 255, 255),
        _ => egui::Color32::from_rgb(255, 255, 255),
    }
}

pub(crate) fn xterm_256_color(index: u8) -> egui::Color32 {
    if index < 16 {
        return ansi_16_color(index);
    }
    if index < 232 {
        let idx = index - 16;
        let r = idx / 36;
        let g = (idx / 6) % 6;
        let b = idx % 6;
        let levels = [0u8, 95, 135, 175, 215, 255];
        return egui::Color32::from_rgb(levels[r as usize], levels[g as usize], levels[b as usize]);
    }
    let gray = 8u8.saturating_add((index - 232).saturating_mul(10));
    egui::Color32::from_rgb(gray, gray, gray)
}

pub(crate) fn parse_color_spec(spec: &str) -> Option<egui::Color32> {
    let spec = spec.trim();
    if spec.eq_ignore_ascii_case("none") {
        return None;
    }
    if let Some(rest) = spec.strip_prefix("rgb:") {
        let comps: Vec<&str> = rest.split('/').collect();
        if comps.len() < 3 {
            return None;
        }
        let r = parse_hex_component(comps[0])?;
        let g = parse_hex_component(comps[1])?;
        let b = parse_hex_component(comps[2])?;
        return Some(egui::Color32::from_rgb(r, g, b));
    }
    if let Some(hex) = spec.strip_prefix('#') {
        if hex.len() % 3 != 0 {
            return None;
        }
        let step = hex.len() / 3;
        if step == 0 || step > 4 {
            return None;
        }
        let r = parse_hex_component(&hex[0..step])?;
        let g = parse_hex_component(&hex[step..step * 2])?;
        let b = parse_hex_component(&hex[step * 2..step * 3])?;
        return Some(egui::Color32::from_rgb(r, g, b));
    }
    None
}

fn parse_hex_component(comp: &str) -> Option<u8> {
    if comp.is_empty() || comp.len() > 4 {
        return None;
    }
    let value = u32::from_str_radix(comp, 16).ok()?;
    let max = (1u32 << (comp.len() * 4)) - 1;
    let scaled = (value.saturating_mul(255) + max / 2) / max;
    Some(scaled as u8)
}

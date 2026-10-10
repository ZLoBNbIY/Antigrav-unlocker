//! Black & Red Studio theme for Antigravity Unlocker.
//!
//! Clean, high-contrast dark theme matching the `adb_ui` project:
//! - Deep solid obsidian background (#0D1117)
//! - Clean dark surface cards (#161B22) with crisp borders (#30363D)
//! - Studio Red accents (#DC2626 / #EF4444) for primary actions, active switches, and highlights
//! - Crisp typography (#F0F6FC) and muted metadata (#8B949E)

use eframe::egui::{self, Color32, CornerRadius, Stroke};

/// Base background - deep dark obsidian (#0D1117).
pub const BG: Color32 = Color32::from_rgb(0x0D, 0x11, 0x17);

/// Surface / Card background (#161B22).
pub const CARD: Color32 = Color32::from_rgb(0x16, 0x1B, 0x22);

/// Inset / sunken control surface (#0D1117).
pub const SUNKEN: Color32 = Color32::from_rgb(0x0D, 0x11, 0x17);

/// Secondary button background (#21262D).
pub const BUTTON: Color32 = Color32::from_rgb(0x21, 0x26, 0x2D);

/// Secondary button hover (#30363D).
pub const BUTTON_HOVER: Color32 = Color32::from_rgb(0x30, 0x36, 0x3D);

/// Secondary button pressed (#161B22).
pub const BUTTON_PRESSED: Color32 = Color32::from_rgb(0x16, 0x1B, 0x22);

/// Card & control border stroke (#30363D).
pub const LINE: Color32 = Color32::from_rgb(0x30, 0x36, 0x3D);

/// Lighter border highlight on hover (#3C444D).
pub const LINE_LIGHT: Color32 = Color32::from_rgb(0x3C, 0x44, 0x4D);

/// Primary high-contrast text (#F0F6FC).
pub const TEXT: Color32 = Color32::from_rgb(0xF0, 0xF6, 0xFC);

/// Softened secondary text (#C9D1D9).
pub const SOFT_TEXT: Color32 = Color32::from_rgb(0xC9, 0xD1, 0xD9);

/// Muted metadata & hint text (#8B949E).
pub const MUTED: Color32 = Color32::from_rgb(0x8B, 0x94, 0x9E);

/// Primary accent - vibrant studio red (#DC2626).
pub const ACCENT: Color32 = Color32::from_rgb(0xDC, 0x26, 0x26);

/// Accent on hover - glowing crimson red (#EF4444).
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(0xEF, 0x44, 0x44);

/// Accent pressed - deep dark red (#991B1B).
#[allow(dead_code)]
pub const ACCENT_PRESSED: Color32 = Color32::from_rgb(0x99, 0x1B, 0x1B);

/// Accent border line (#EF4444).
pub const ACCENT_LINE: Color32 = Color32::from_rgb(0xEF, 0x44, 0x44);

/// Status "OK / succeeded" - emerald green (#4ADE80).
pub const OK: Color32 = Color32::from_rgb(0x4A, 0xDE, 0x80);

/// Status "Needs attention / fixing" - warm amber (#F59E0B).
pub const WARN: Color32 = Color32::from_rgb(0xF5, 0x9E, 0x0B);

/// Status "Off / failed / error" - red danger (#EF4444).
pub const BAD: Color32 = Color32::from_rgb(0xEF, 0x44, 0x44);

/// Corner radius for main cards.
pub const RADIUS: u8 = 8;

/// Corner radius for buttons, switches and insets.
pub const RADIUS_SMALL: u8 = 6;

/// Installs Segoe UI Semibold as the UI face, when the machine has it.
fn install_font(ctx: &egui::Context) {
    let dir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
    let candidates = ["seguisb.ttf", "segoeui.ttf"];
    let Some((name, bytes)) = candidates.iter().find_map(|f| {
        let path = std::path::Path::new(&dir).join("Fonts").join(f);
        std::fs::read(&path).ok().map(|b| (*f, b))
    }) else {
        return;
    };

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        name.to_string(),
        std::sync::Arc::new(egui::FontData::from_owned(bytes)),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, name.to_string());
    ctx.set_fonts(fonts);
}

/// Applies the theme to an egui context. Called once at startup.
pub fn apply(ctx: &egui::Context) {
    install_font(ctx);

    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = BG;
    visuals.window_fill = BG;
    visuals.extreme_bg_color = SUNKEN;
    visuals.faint_bg_color = CARD;
    visuals.override_text_color = Some(TEXT);
    visuals.hyperlink_color = ACCENT_HOVER;

    let r = CornerRadius::same(RADIUS_SMALL);
    visuals.widgets.noninteractive.bg_fill = CARD;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, LINE);
    visuals.widgets.noninteractive.corner_radius = r;

    visuals.widgets.inactive.bg_fill = BUTTON;
    visuals.widgets.inactive.weak_bg_fill = BUTTON;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, LINE);
    visuals.widgets.inactive.corner_radius = r;

    visuals.widgets.hovered.bg_fill = BUTTON_HOVER;
    visuals.widgets.hovered.weak_bg_fill = BUTTON_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, LINE_LIGHT);
    visuals.widgets.hovered.corner_radius = r;

    visuals.widgets.active.bg_fill = BUTTON_PRESSED;
    visuals.widgets.active.weak_bg_fill = BUTTON_PRESSED;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT_LINE);
    visuals.widgets.active.corner_radius = r;

    visuals.widgets.open.bg_fill = CARD;
    visuals.widgets.open.weak_bg_fill = CARD;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, LINE);
    visuals.widgets.open.corner_radius = r;

    visuals.selection.bg_fill = Color32::from_rgba_premultiplied(220, 38, 38, 70);
    visuals.selection.stroke = Stroke::new(1.0, ACCENT_LINE);

    ctx.set_theme(egui::ThemePreference::Dark);
    ctx.set_visuals_of(egui::Theme::Dark, visuals);

    ctx.all_styles_mut(|style| {
        use egui::{FontFamily::Proportional, FontId, TextStyle};
        style.text_styles = [
            (TextStyle::Heading, FontId::new(21.0, Proportional)),
            (TextStyle::Body, FontId::new(14.5, Proportional)),
            (TextStyle::Button, FontId::new(14.0, Proportional)),
            (TextStyle::Small, FontId::new(12.5, Proportional)),
            (
                TextStyle::Monospace,
                FontId::new(13.0, egui::FontFamily::Monospace),
            ),
        ]
        .into();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(12.0, 7.0);
        style.spacing.interact_size.y = 26.0;
        style.spacing.scroll.floating = false;
        style.spacing.scroll.bar_width = 8.0;
        style.spacing.scroll.bar_inner_margin = 2.0;
        style.spacing.scroll.bar_outer_margin = 0.0;
    });
}

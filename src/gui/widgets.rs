//! The small set of controls every screen is built from.
//!
//! Restyled in clean Black & Red Studio aesthetic matching adb_ui.

use eframe::egui::{self, CornerRadius, Sense, Stroke};

use super::theme;

/// A clean black & red toggle switch.
pub fn switch(ui: &mut egui::Ui, on: &mut bool, enabled: bool) -> egui::Response {
    let size = egui::vec2(44.0, 24.0);
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, mut resp) = ui.allocate_exact_size(size, sense);

    if enabled && resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let how = ui.ctx().animate_bool_with_time(resp.id, *on, 0.14);
        let painter = ui.painter();
        let r_cr = CornerRadius::same(12);

        if *on {
            // Vibrant studio red active track
            let track_color = if enabled {
                theme::ACCENT
            } else {
                egui::Color32::from_rgba_premultiplied(185, 28, 28, 120)
            };
            painter.rect_filled(rect, r_cr, track_color);
            painter.rect_stroke(
                rect,
                r_cr,
                Stroke::new(1.0, theme::ACCENT_HOVER),
                egui::StrokeKind::Inside,
            );
        } else {
            // Dark obsidian inactive track
            painter.rect_filled(rect, r_cr, theme::BUTTON);
            painter.rect_stroke(
                rect,
                r_cr,
                Stroke::new(1.0, theme::LINE),
                egui::StrokeKind::Inside,
            );
        }

        // Knob
        let r = rect.height() / 2.0 - 3.0;
        let cx = egui::lerp((rect.left() + r + 3.0)..=(rect.right() - r - 3.0), how);
        let knob_center = egui::pos2(cx, rect.center().y);

        if *on && enabled {
            // Soft red glow halo around the knob
            painter.circle_filled(
                knob_center,
                r + 2.5,
                egui::Color32::from_rgba_premultiplied(239, 68, 68, 45),
            );
        }

        let knob_color = if enabled {
            if *on {
                egui::Color32::WHITE
            } else {
                theme::SOFT_TEXT
            }
        } else {
            theme::MUTED.gamma_multiply(0.6)
        };
        painter.circle_filled(knob_center, r, knob_color);
    }

    if enabled {
        resp.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        resp
    }
}

/// Room reserved on the right of a row for its switch: the 44 px control plus
/// the gap that keeps it off the card's edge.
pub const SWITCH_COLUMN: f32 = 64.0;

/// A row of text with a switch pinned to its right.
pub fn switch_row(
    ui: &mut egui::Ui,
    on: &mut bool,
    enabled: bool,
    text: impl FnOnce(&mut egui::Ui),
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let text_w = (ui.available_width() - SWITCH_COLUMN).max(120.0);
        ui.allocate_ui_with_layout(
            egui::vec2(text_w, 0.0),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.set_max_width(text_w);
                text(ui);
            },
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            changed = switch(ui, on, enabled).changed();
        });
    });
    changed
}

/// A titled dark surface card with subtle border.
pub fn card<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let frame = egui::Frame::new()
        .fill(theme::CARD)
        .corner_radius(CornerRadius::same(theme::RADIUS))
        .inner_margin(egui::Margin::same(15))
        .stroke(Stroke::new(1.0, theme::LINE))
        .shadow(egui::Shadow {
            offset: [0, 2],
            blur: 8,
            spread: 0,
            color: egui::Color32::from_black_alpha(100),
        });

    let resp = frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        add(ui)
    });
    resp.inner
}

/// Primary studio red action button.
pub fn primary(ui: &mut egui::Ui, text: &str, enabled: bool) -> egui::Response {
    let (fill, stroke, text_color) = if enabled {
        (
            theme::ACCENT,
            Stroke::new(1.0, theme::ACCENT_LINE),
            egui::Color32::WHITE,
        )
    } else {
        (theme::BUTTON, Stroke::new(1.0, theme::LINE), theme::MUTED)
    };
    let btn = egui::Button::new(
        egui::RichText::new(text)
            .color(text_color)
            .strong()
            .size(14.5),
    )
    .fill(fill)
    .stroke(stroke)
    .corner_radius(CornerRadius::same(theme::RADIUS_SMALL))
    .min_size(egui::vec2(0.0, 34.0));

    ui.add_enabled(enabled, btn)
}

/// A clean dark secondary button with border.
pub fn ghost(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(
        egui::RichText::new(text)
            .color(theme::TEXT)
            .size(13.5),
    )
    .fill(theme::BUTTON)
    .stroke(Stroke::new(1.0, theme::LINE))
    .corner_radius(CornerRadius::same(theme::RADIUS_SMALL))
    .min_size(egui::vec2(0.0, 30.0));
    ui.add(btn)
}

/// Small muted explanatory text under a control.
pub fn hint(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).color(theme::MUTED).size(12.5));
}

/// A status dot with subtle halo.
pub fn dot(ui: &mut egui::Ui, color: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let center = rect.center();
        // Glow halo
        p.circle_filled(center, 5.0, color.gamma_multiply(0.25));
        // Core dot
        p.circle_filled(center, 3.5, color);
    }
}

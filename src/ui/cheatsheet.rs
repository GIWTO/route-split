// 咒语区组件 - FlClash fake-ip-filter 配置

use super::theme::{self, SUCCESS_COLOR};
use arboard::Clipboard;
use eframe::egui::{self, RichText, Sense, Ui};
use std::time::Instant;

/// 咒语区状态
pub struct CheatsheetState {
    pub last_copy_time: Option<Instant>,
}

impl Default for CheatsheetState {
    fn default() -> Self {
        Self {
            last_copy_time: None,
        }
    }
}

impl CheatsheetState {
    pub fn is_highlight_active(&self) -> bool {
        if let Some(time) = self.last_copy_time {
            time.elapsed().as_millis() < 800
        } else {
            false
        }
    }

    pub fn mark_copied(&mut self) {
        self.last_copy_time = Some(Instant::now());
    }
}

fn generate_config(target_dest: &str) -> String {
    let domains = vec![
        "*.sinopec.com",
        "sinopec.com",
        "+.sinopec.com",
        "*.internal.company.com",
    ];

    let mut config = String::from("fake-ip-filter:\n");
    for domain in domains {
        config.push_str(&format!("  - \"{}\"\n", domain));
    }

    if target_dest.starts_with("10.") {
        config.push_str("  - \"10.0.0.0/8\"\n");
    } else if target_dest.is_empty() {
        config.push_str("  - \"10.0.0.0/8\"\n");
    } else {
        config.push_str(&format!("  - \"{}/8\"\n", target_dest));
    }

    config.push_str("  - \"192.168.*\"\n");
    config.push_str("  - \"+.local\"\n");
    config.push_str("  - \"+.lan\"\n");

    config
}

/// 渲染咒语区
pub fn render_cheatsheet(ui: &mut Ui, state: &mut CheatsheetState, target_dest: &str) {
    let is_highlighted = state.is_highlight_active();
    let dark_mode = ui.visuals().dark_mode;
    let text_color = theme::get_text_color(dark_mode);
    let secondary_color = theme::get_secondary_text_color(dark_mode);
    let card_bg = theme::get_card_bg_color(dark_mode);
    let border_color = theme::get_border_color(dark_mode);

    let current_config = generate_config(target_dest);

    ui.horizontal(|ui| {
        ui.label(
            RichText::new("📖 排除指南 (建议配置到 FlClash)")
                .color(text_color)
                .size(16.0)
                .strong(),
        );
        if is_highlighted {
            ui.label(RichText::new(" ✨ 已复制").color(SUCCESS_COLOR).size(14.0));
        }
    });
    ui.add_space(8.0);

    // 代码块容器 - 关键：设置宽度
    egui::Frame::group(ui.style())
        .fill(card_bg)
        .stroke(egui::Stroke::new(
            1.0,
            if is_highlighted {
                SUCCESS_COLOR
            } else {
                border_color
            },
        ))
        .rounding(8.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width()); // 关键配置

            let response = ui.add(
                egui::Label::new(
                    RichText::new(&current_config)
                        .color(if is_highlighted {
                            text_color
                        } else {
                            secondary_color
                        })
                        .size(13.0)
                        .family(egui::FontFamily::Monospace),
                )
                .sense(Sense::click()),
            );

            if response.clicked() {
                if let Ok(mut clipboard) = Clipboard::new() {
                    if clipboard.set_text(&current_config).is_ok() {
                        state.mark_copied();
                    }
                }
            }

            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        });

    ui.add_space(4.0);
    ui.label(
        RichText::new("💡 点击上方代码块可直接复制")
            .color(secondary_color)
            .size(11.0),
    );
}

// status_panel.rs - 状态监测区组件

use super::theme;
use crate::network::AdapterInfo;
use eframe::egui::{self, Color32, Layout, RichText, Ui};

/// 渲染状态监测区
pub fn render_status_panel(
    ui: &mut Ui,
    route_exists: bool,
    proxy_enabled: bool,
    internal_adapter: &Option<AdapterInfo>,
    external_adapter: &Option<AdapterInfo>,
    internal_ping: Option<u32>,
    external_ping: Option<u32>,
    active_routes: &[crate::network::RouteEntry],
    dark_mode: bool,
) {
    let text_color = theme::get_text_color(dark_mode);
    let secondary_color = theme::get_secondary_text_color(dark_mode);

    // 半透明胶囊徽章的底色计算
    let get_badge_bg = |base_color: Color32| -> Color32 {
        if dark_mode {
            Color32::from_rgba_unmultiplied(base_color.r(), base_color.g(), base_color.b(), 30)
        } else {
            Color32::from_rgba_unmultiplied(base_color.r(), base_color.g(), base_color.b(), 20)
        }
    };

    let gray_color = theme::get_secondary_text_color(dark_mode);
    let offline_bg = get_badge_bg(gray_color);

    // --- 1. 外网出口卡片 ---
    egui::Frame::none()
        .fill(theme::get_card_bg_color(dark_mode))
        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
        .rounding(theme::CARD_ROUNDING)
        .inner_margin(16.0) // 充足内边距
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🌎 外网出口").color(text_color).strong().size(theme::FONT_SIZE_SUBTITLE));
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if external_adapter.is_some() && !proxy_enabled {
                            let green_bg = get_badge_bg(theme::SUCCESS_COLOR);
                            if let Some(ping) = external_ping {
                                theme::draw_badge(ui, &format!("在线 ({}ms)", ping), theme::SUCCESS_COLOR, green_bg);
                            } else {
                                theme::draw_badge(ui, "在线", theme::SUCCESS_COLOR, green_bg);
                            }
                        } else {
                            theme::draw_badge(ui, "离线", gray_color, offline_bg);
                        }
                    });
                });
                ui.add_space(8.0);
                
                if let Some(ext) = external_adapter {
                    // 使用 CARD_SOFT 浅灰背景包裹只读卡片内容，突显层次
                    egui::Frame::none()
                        .fill(theme::get_card_soft_color(dark_mode))
                        .rounding(theme::BUTTON_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&ext.name).color(text_color).size(theme::FONT_SIZE_BODY).strong());
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("IP:").color(secondary_color).size(theme::FONT_SIZE_SMALL));
                                    ui.label(RichText::new(&ext.ip).color(text_color).size(theme::FONT_SIZE_SMALL).monospace());
                                    ui.add_space(16.0);
                                    ui.label(RichText::new("网关:").color(secondary_color).size(theme::FONT_SIZE_SMALL));
                                    ui.label(RichText::new(&ext.gateway).color(text_color).size(theme::FONT_SIZE_SMALL).monospace());
                                });
                            });
                        });
                } else {
                    ui.label(RichText::new("未选择网卡").color(secondary_color).size(theme::FONT_SIZE_BODY).italics());
                }
            });
        });

    ui.add_space(14.0);

    // --- 2. 内网分流卡片 ---
    egui::Frame::none()
        .fill(theme::get_card_bg_color(dark_mode))
        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
        .rounding(theme::CARD_ROUNDING)
        .inner_margin(16.0) // 充足内边距
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🏢 内网分流").color(text_color).strong().size(theme::FONT_SIZE_SUBTITLE));
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if route_exists {
                            let green_bg = get_badge_bg(theme::SUCCESS_COLOR);
                            if let Some(ping) = internal_ping {
                                theme::draw_badge(ui, &format!("已生效 ({}ms)", ping), theme::SUCCESS_COLOR, green_bg);
                            } else {
                                theme::draw_badge(ui, "已生效", theme::SUCCESS_COLOR, green_bg);
                            }
                        } else {
                            theme::draw_badge(ui, "未生效", gray_color, offline_bg);
                        }
                    });
                });
                ui.add_space(8.0);
                
                if let Some(int) = internal_adapter {
                    // 使用 CARD_SOFT 浅灰背景包裹
                    egui::Frame::none()
                        .fill(theme::get_card_soft_color(dark_mode))
                        .rounding(theme::BUTTON_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&int.name).color(text_color).size(theme::FONT_SIZE_BODY).strong());
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("IP:").color(secondary_color).size(theme::FONT_SIZE_SMALL));
                                    ui.label(RichText::new(&int.ip).color(text_color).size(theme::FONT_SIZE_SMALL).monospace());
                                    ui.add_space(16.0);
                                    ui.label(RichText::new("网关:").color(secondary_color).size(theme::FONT_SIZE_SMALL));
                                    ui.label(RichText::new(&int.gateway).color(text_color).size(theme::FONT_SIZE_SMALL).monospace());
                                });
                            });
                        });
                } else {
                    ui.label(RichText::new("未选择网卡").color(secondary_color).size(theme::FONT_SIZE_BODY).italics());
                }

                if !active_routes.is_empty() {
                    ui.add_space(8.0);
                    // 活跃分流路由信息，也使用 CARD_SOFT 区分
                    egui::Frame::none()
                        .fill(theme::get_card_soft_color(dark_mode))
                        .rounding(theme::BUTTON_ROUNDING)
                        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.label(RichText::new("📍 活跃路由分流条目:").color(text_color).size(theme::FONT_SIZE_SMALL).strong());
                                ui.add_space(4.0);
                                for r in active_routes.iter().take(3) {
                                    ui.label(
                                        RichText::new(format!("• {} ➔ 网关 {}", r.destination, r.gateway))
                                            .size(theme::FONT_SIZE_SMALL)
                                            .color(secondary_color)
                                            .monospace(),
                                    );
                                }
                                if active_routes.len() > 3 {
                                    ui.label(
                                        RichText::new(format!("... 及其他 {} 条路由规则", active_routes.len() - 3))
                                            .size(theme::FONT_SIZE_TINY)
                                            .color(secondary_color),
                                    );
                                }
                            });
                        });
                }
            });
        });

    ui.add_space(14.0);

    // --- 3. 系统代理状态卡片 ---
    egui::Frame::none()
        .fill(theme::get_card_bg_color(dark_mode))
        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
        .rounding(theme::CARD_ROUNDING)
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new("🛡 系统代理状态").color(text_color).strong().size(theme::FONT_SIZE_SUBTITLE));
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if !proxy_enabled {
                        let green_bg = get_badge_bg(theme::SUCCESS_COLOR);
                        theme::draw_badge(ui, "已绕过", theme::SUCCESS_COLOR, green_bg);
                    } else {
                        let orange_bg = get_badge_bg(theme::WARNING_COLOR);
                        theme::draw_badge(ui, "启用中 (可能干扰分流)", theme::WARNING_COLOR, orange_bg);
                    }
                });
            });
        });
}

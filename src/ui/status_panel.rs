// 状态监测区组件

use super::theme::{self, SUCCESS_COLOR};
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

    // 外网出口卡片
    egui::Frame::none()
        .fill(theme::get_card_bg_color(dark_mode))
        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
        .rounding(10.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🌎 外网出口").color(text_color).strong().size(14.0));
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if external_adapter.is_some() && !proxy_enabled {
                            if let Some(ping) = external_ping {
                                ui.label(RichText::new(format!("● {}ms", ping)).color(SUCCESS_COLOR).strong());
                            } else if external_ping.is_none() && external_adapter.is_some() {
                                ui.label(RichText::new("● 在线").color(SUCCESS_COLOR).strong());
                            }
                        } else {
                            ui.label(RichText::new("○ 离线").color(secondary_color));
                        }
                    });
                });
                ui.add_space(6.0);
                if let Some(ext) = external_adapter {
                    ui.label(RichText::new(&ext.name).color(text_color).size(13.0));
                    ui.add_space(2.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("IP:").color(secondary_color).size(11.0));
                        ui.label(RichText::new(&ext.ip).color(text_color).size(11.0));
                        ui.add_space(8.0);
                        ui.label(RichText::new("Gate:").color(secondary_color).size(11.0));
                        ui.label(RichText::new(&ext.gateway).color(text_color).size(11.0));
                    });
                } else {
                    ui.label(RichText::new("未选择网卡").color(secondary_color).size(11.0).italics());
                }
            });
        });

    ui.add_space(8.0);

    // 内网分流卡片
    egui::Frame::none()
        .fill(theme::get_card_bg_color(dark_mode))
        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
        .rounding(10.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🏢 内网分流").color(text_color).strong().size(14.0));
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if route_exists {
                            if let Some(ping) = internal_ping {
                                ui.label(RichText::new(format!("● {}ms", ping)).color(SUCCESS_COLOR).strong());
                            } else {
                                ui.label(RichText::new("● 已生效").color(SUCCESS_COLOR).strong());
                            }
                        } else {
                            ui.label(RichText::new("○ 未生效").color(secondary_color));
                        }
                    });
                });
                ui.add_space(6.0);
                if let Some(int) = internal_adapter {
                    ui.label(RichText::new(&int.name).color(text_color).size(13.0));
                    ui.add_space(2.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("IP:").color(secondary_color).size(11.0));
                        ui.label(RichText::new(&int.ip).color(text_color).size(11.0));
                        ui.add_space(8.0);
                        ui.label(RichText::new("Gate:").color(secondary_color).size(11.0));
                        ui.label(RichText::new(&int.gateway).color(text_color).size(11.0));
                    });
                } else {
                    ui.label(RichText::new("未选择网卡").color(secondary_color).size(11.0).italics());
                }

                if !active_routes.is_empty() {
                    ui.add_space(8.0);
                    egui::Frame::none()
                        .fill(Color32::from_black_alpha(20))
                        .rounding(4.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                for r in active_routes.iter().take(3) {
                                    ui.label(RichText::new(format!("📍 {} -> {}", r.destination, r.gateway)).size(10.0).color(secondary_color));
                                }
                                if active_routes.len() > 3 {
                                    ui.label(RichText::new(format!("... 及其他 {} 条路由", active_routes.len() - 3)).size(9.0).color(secondary_color));
                                }
                            });
                        });
                }
            });
        });

    ui.add_space(12.0);

    ui.add_space(10.0);
    // 系统代理状态
    egui::Frame::none()
        .fill(theme::get_card_bg_color(dark_mode))
        .rounding(10.0)
        .inner_margin(egui::Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("🛡 系统代理:").color(secondary_color).size(12.0));
                if !proxy_enabled {
                    ui.label(RichText::new("已绕过").color(SUCCESS_COLOR).size(12.0).strong());
                } else {
                    ui.label(RichText::new("开启中 (可能干扰分流)").color(theme::WARNING_COLOR).size(12.0).strong());
                }
            });
        });
}

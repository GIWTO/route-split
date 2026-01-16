// 状态监测区组件

use super::theme::{self, SUCCESS_COLOR, WARNING_COLOR};
use crate::network::AdapterInfo;
use eframe::egui::{self, Layout, RichText, Ui};

/// 渲染状态监测区
pub fn render_status_panel(
    ui: &mut Ui,
    route_exists: bool,
    proxy_enabled: bool,
    internal_adapter: &Option<AdapterInfo>,
    external_adapter: &Option<AdapterInfo>,
    dark_mode: bool,
) {
    let text_color = theme::get_text_color(dark_mode);
    let secondary_color = theme::get_secondary_text_color(dark_mode);

    ui.label(
        RichText::new("📊 实时流量预览")
            .color(text_color)
            .size(16.0)
            .strong(),
    );
    ui.add_space(8.0);

    // 外网出口卡片 - 关键：在 Frame 内部使用 ui.available_width()
    egui::Frame::group(ui.style())
        .fill(ui.visuals().extreme_bg_color)
        .rounding(8.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width()); // 关键：使用当前上下文的可用宽度

            ui.horizontal(|ui| {
                ui.label(RichText::new("🌎 外网出口").color(text_color).strong());
            });
            ui.add_space(4.0);
            if let Some(ext) = external_adapter {
                ui.label(RichText::new(&ext.name).color(text_color).size(13.0));
                ui.label(
                    RichText::new(format!("本地IP: {}", ext.ip))
                        .color(secondary_color)
                        .size(11.0),
                );
                ui.label(
                    RichText::new(format!("网关: {}", ext.gateway))
                        .color(secondary_color)
                        .size(11.0),
                );
            } else {
                ui.label(
                    RichText::new("请在上方选择网卡")
                        .color(secondary_color)
                        .size(11.0)
                        .italics(),
                );
            }
        });

    ui.add_space(8.0);

    // 内网分流卡片
    egui::Frame::group(ui.style())
        .fill(ui.visuals().extreme_bg_color)
        .rounding(8.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width()); // 关键：使用当前上下文的可用宽度

            ui.horizontal(|ui| {
                ui.label(RichText::new("🏢 内网分流").color(text_color).strong());
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if route_exists {
                        ui.label(RichText::new("● 已生效").color(SUCCESS_COLOR).size(10.0));
                    } else {
                        ui.label(RichText::new("○ 未生效").color(secondary_color).size(10.0));
                    }
                });
            });
            ui.add_space(4.0);
            if let Some(int) = internal_adapter {
                ui.label(RichText::new(&int.name).color(text_color).size(13.0));
                ui.label(
                    RichText::new(format!("本地IP: {}", int.ip))
                        .color(secondary_color)
                        .size(11.0),
                );
                ui.label(
                    RichText::new(format!("网关: {}", int.gateway))
                        .color(secondary_color)
                        .size(11.0),
                );
            } else {
                ui.label(
                    RichText::new("请在上方选择网卡")
                        .color(secondary_color)
                        .size(11.0)
                        .italics(),
                );
            }
        });

    ui.add_space(12.0);

    // 系统代理状态
    egui::Frame::group(ui.style()).rounding(8.0).show(ui, |ui| {
        ui.set_width(ui.available_width()); // 关键：使用当前上下文的可用宽度
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("🛡 系统代理:")
                    .color(secondary_color)
                    .size(12.0),
            );
            if !proxy_enabled {
                ui.label(
                    RichText::new("已绕过")
                        .color(SUCCESS_COLOR)
                        .size(12.0)
                        .strong(),
                );
            } else {
                ui.label(
                    RichText::new("开启中 (可能干扰分流)")
                        .color(WARNING_COLOR)
                        .size(12.0)
                        .strong(),
                );
            }
        });
    });
}

// action_panel.rs - 快捷操作区组件

use super::theme;
use eframe::egui::{self, Button, Color32, Layout, RichText, Ui, Vec2};

/// 按钮点击结果
pub enum ActionResult {
    /// 无操作
    None,
    /// 点击了配置按钮
    Fix,
    /// 点击了回滚按钮
    Rollback,
    /// 点击了刷新按钮
    Refresh,
}

/// 渲染动作控制区
pub fn render_action_panel(ui: &mut Ui, is_admin: bool, is_processing: bool) -> ActionResult {
    let mut result = ActionResult::None;
    let dark_mode = ui.visuals().dark_mode;
    let text_color = theme::get_text_color(dark_mode);

    // 将快捷操作也打包进一致的 14px 圆角卡片中
    egui::Frame::none()
        .fill(theme::get_card_bg_color(dark_mode))
        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
        .rounding(theme::CARD_ROUNDING)
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                // 顶部标题和刷新按钮 (次级 Ghost 按钮)
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚡ 快捷操作").color(text_color).size(theme::FONT_SIZE_SUBTITLE).strong());
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        let refresh_btn = Button::new(
                            RichText::new("🔄 刷新网卡")
                                .color(text_color)
                                .size(theme::FONT_SIZE_SMALL)
                                .strong(),
                        )
                        .fill(theme::get_card_soft_color(dark_mode))
                        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
                        .rounding(theme::BUTTON_ROUNDING)
                        .min_size(Vec2::new(90.0, 28.0));

                        if ui.add(refresh_btn).clicked() {
                            result = ActionResult::Refresh;
                        }
                    });
                });
                ui.add_space(12.0);

                // 操作按钮对齐
                ui.horizontal(|ui| {
                    // 1. 一键配置按钮 (Primary 主按钮：蓝底白字)
                    let fix_text = if is_processing { "⏳ 处理中..." } else { "🚀 一键配置分流" };
                    let fix_btn_style = Button::new(
                        RichText::new(fix_text)
                            .color(Color32::WHITE)
                            .size(theme::FONT_SIZE_BODY)
                            .strong(),
                    )
                    .fill(if is_admin && !is_processing {
                        theme::ACCENT_COLOR
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    })
                    .rounding(theme::BUTTON_ROUNDING)
                    .min_size(Vec2::new(160.0, 38.0));

                    let fix_enabled = is_admin && !is_processing;
                    if ui.add_enabled(fix_enabled, fix_btn_style).clicked() {
                        result = ActionResult::Fix;
                    }

                    ui.add_space(8.0);

                    // 2. 恢复默认按钮 (Danger 危险按钮：淡红底红字 + 红色描边)
                    let danger_bg = if dark_mode {
                        Color32::from_rgba_unmultiplied(239, 68, 68, 25)
                    } else {
                        Color32::from_rgba_unmultiplied(239, 68, 68, 15)
                    };
                    
                    let rollback_btn_style = Button::new(
                        RichText::new("↩ 恢复默认")
                            .color(if is_admin && !is_processing {
                                theme::DANGER_COLOR
                            } else {
                                theme::get_secondary_text_color(dark_mode)
                            })
                            .size(theme::FONT_SIZE_BODY)
                            .strong(),
                    )
                    .fill(if is_admin && !is_processing {
                        danger_bg
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    })
                    .stroke(egui::Stroke::new(
                        1.0,
                        if is_admin && !is_processing {
                            theme::DANGER_COLOR.gamma_multiply(0.4)
                        } else {
                            theme::get_border_color(dark_mode)
                        }
                    ))
                    .rounding(theme::BUTTON_ROUNDING)
                    .min_size(Vec2::new(110.0, 38.0));

                    let rollback_enabled = is_admin && !is_processing;
                    if ui.add_enabled(rollback_enabled, rollback_btn_style).clicked() {
                        result = ActionResult::Rollback;
                    }
                });

                if !is_admin {
                    ui.add_space(10.0);
                    // 警告横幅也使用 CARD_SOFT 风格填充，并且显示更加克制
                    egui::Frame::none()
                        .fill(theme::get_card_soft_color(dark_mode))
                        .rounding(theme::BUTTON_ROUNDING)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.label(
                                RichText::new("⚠ 提示：请以管理员身份运行此程序，否则无法执行配置或恢复操作。")
                                    .color(theme::DANGER_COLOR)
                                    .size(theme::FONT_SIZE_SMALL)
                                    .strong()
                            );
                        });
                }
            });
        });

    result
}

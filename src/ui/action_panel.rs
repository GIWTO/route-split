use super::theme::{self, ACCENT_COLOR, DANGER_COLOR};
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

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("⚡ 快捷操作").color(text_color).size(15.0).strong());
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🔄 刷新").clicked() {
                    result = ActionResult::Refresh;
                }
            });
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            // 一键配置按钮 (Primary)
            let fix_text = if is_processing { "⏳ 处理中..." } else { "🚀 一键配置分流" };
            let fix_button = Button::new(
                RichText::new(fix_text)
                    .color(Color32::WHITE)
                    .size(14.0)
                    .strong(),
            )
            .fill(if is_admin && !is_processing {
                ACCENT_COLOR
            } else {
                ui.visuals().widgets.inactive.bg_fill
            })
            .rounding(10.0)
            .min_size(Vec2::new(160.0, 36.0));

            let fix_enabled = is_admin && !is_processing;
            if ui.add_enabled(fix_enabled, fix_button).clicked() {
                result = ActionResult::Fix;
            }

            ui.add_space(8.0);

            // 回滚按钮 (Secondary)
            let rollback_button = Button::new(
                RichText::new("↩ 恢复默认")
                    .color(if is_admin && !is_processing {
                        DANGER_COLOR
                    } else {
                        text_color
                    })
                    .size(14.0),
            )
            .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
            .rounding(10.0)
            .min_size(Vec2::new(100.0, 36.0));

            let rollback_enabled = is_admin && !is_processing;
            if ui.add_enabled(rollback_enabled, rollback_button).clicked() {
                result = ActionResult::Rollback;
            }
        });

        if !is_admin {
            ui.add_space(8.0);
            egui::Frame::none()
                .fill(Color32::from_black_alpha(40))
                .rounding(6.0)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.label(RichText::new("⚠ 请以管理员身份运行以解锁上述操作").color(DANGER_COLOR).size(11.0));
                });
        }
    });

    result
}

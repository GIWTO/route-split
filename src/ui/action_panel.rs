// 动作控制区组件

use super::theme::{self, DANGER_COLOR, SUCCESS_COLOR};
use eframe::egui::{Button, RichText, Ui, Vec2};

/// 按钮点击结果
pub enum ActionResult {
    /// 无操作
    None,
    /// 点击了配置按钮
    Fix,
    /// 点击了回滚按钮
    Rollback,
}

/// 渲染动作控制区
pub fn render_action_panel(ui: &mut Ui, is_admin: bool, is_processing: bool) -> ActionResult {
    let mut result = ActionResult::None;
    let dark_mode = ui.visuals().dark_mode;
    let text_color = theme::get_text_color(dark_mode);

    ui.vertical(|ui| {
        ui.label(
            RichText::new("⚡ 快捷操作")
                .color(text_color)
                .size(16.0)
                .strong(),
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            // 一键配置按钮 (Primary)
            let fix_text = if is_processing {
                "⏳ 正在处理..."
            } else {
                "🚀 一键配置分流"
            };
            let fix_button = Button::new(
                RichText::new(fix_text)
                    .color(if dark_mode {
                        theme::get_bg_color(dark_mode)
                    } else {
                        Color32::WHITE
                    })
                    .size(15.0)
                    .strong(),
            )
            .fill(if is_admin && !is_processing {
                SUCCESS_COLOR
            } else {
                ui.visuals().widgets.inactive.bg_fill
            })
            .min_size(Vec2::new(180.0, 40.0));

            let fix_enabled = is_admin && !is_processing;
            if ui.add_enabled(fix_enabled, fix_button).clicked() {
                result = ActionResult::Fix;
            }

            ui.add_space(12.0);

            // 回滚按钮 (Secondary/Danger)
            let rollback_button = Button::new(
                RichText::new("↩ 恢复默认")
                    .color(if is_admin && !is_processing {
                        DANGER_COLOR
                    } else {
                        text_color
                    })
                    .size(15.0),
            )
            .min_size(Vec2::new(120.0, 40.0));

            let rollback_enabled = is_admin && !is_processing;
            if ui.add_enabled(rollback_enabled, rollback_button).clicked() {
                result = ActionResult::Rollback;
            }
        });

        if !is_admin {
            ui.add_space(4.0);
            ui.label(
                RichText::new("⚠ 请以管理员身份运行以解锁上述操作")
                    .color(DANGER_COLOR)
                    .size(12.0),
            );
        }
    });

    result
}

// 补充导入缺失
use eframe::egui::Color32;

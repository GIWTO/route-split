// 咒语区组件 - FlClash & Clash Verge 配置指南

use super::theme;
use arboard;
use eframe::egui::{self, Color32, RichText, Sense, Ui};
use std::time::Instant;

/// 支持的代理配置工具
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigTool {
    FlClash,
    ClashVerge,
}

/// 咒语区状态
pub struct CheatsheetState {
    pub last_copy_time: Option<Instant>,
    pub active_tool: ConfigTool,
}

impl Default for CheatsheetState {
    fn default() -> Self {
        Self {
            last_copy_time: None,
            active_tool: ConfigTool::FlClash,
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

/// 生成 FlClash 排除指南配置
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

/// 生成 Clash Verge 全局拓展覆写配置
fn generate_clash_verge_config(target_dest: &str) -> String {
    let target_ip_range = if target_dest.starts_with("10.") || target_dest.is_empty() {
        "10.0.0.0/8"
    } else {
        &format!("{}/8", target_dest)
    };

    let mut config = String::new();
    config.push_str("profile:\n  store-selected: true\n\ndns:\n  enable: true\n\n");
    config.push_str("  # 默认使用的公网 DNS（解析绝大多数公网域名）\n");
    config.push_str("  nameserver:\n    - 223.5.5.5\n    - 119.29.29.29\n\n");
    config.push_str("  # 备用 DNS（可以设置为 TLS 加密的公共 DNS，防止污染）\n");
    config.push_str("  fallback:\n    - tls://1.1.1.1\n    - https://dns.alidns.com/dns-query\n\n");
    config.push_str("  # fallback 过滤规则：只对特定情况进行 fallback，避免内网 DNS 干扰\n");
    config.push_str("  fallback-filter:\n    geoip: true\n    geoip-code: CN\n    domain:\n");
    config.push_str("      - '+.sinopec.com'\n      - '+.internal.company.com'\n    ipcidr:\n");
    config.push_str(&format!("      - {}\n", target_ip_range));
    config.push_str("      - 192.168.0.0/16\n      - 172.16.0.0/12\n\n");
    config.push_str("  # 🔥 关键：强制指定“内网域名”只使用“内网 DNS”\n");
    config.push_str("  nameserver-policy:\n    '+.sinopec.com': 10.107.11.4\n    '+.internal.company.com': 10.107.11.4\n\n");
    config.push_str("  # fake-ip 白名单（这些域名返回真实 IP）\n");
    config.push_str("  fake-ip-filter:\n    - '+.sinopec.com'\n    - '*.internal.company.com'\n    - '+.quantum-air.xyz'\n");
    config.push_str(&format!("    - '{}'\n", target_ip_range));
    config.push_str("    - '192.168.0.0/16'\n    - '+.local'\n    - '+.lan'\n");
    
    config
}

/// 渲染咒语区
pub fn render_cheatsheet(ui: &mut Ui, state: &mut CheatsheetState, target_dest: &str) {
    let is_highlighted = state.is_highlight_active();
    let dark_mode = ui.visuals().dark_mode;
    let text_color = theme::get_text_color(dark_mode);
    let secondary_color = theme::get_secondary_text_color(dark_mode);
    let border_color = theme::get_border_color(dark_mode);

    // 根据选中的标签动态生成配置
    let current_config = match state.active_tool {
        ConfigTool::FlClash => generate_config(target_dest),
        ConfigTool::ClashVerge => generate_clash_verge_config(target_dest),
    };

    // 标题与提示语
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("📖 排除配置指南 (防止代理干扰内网)")
                .color(text_color)
                .size(theme::FONT_SIZE_SUBTITLE)
                .strong(),
        );
        if is_highlighted {
            ui.label(RichText::new(" ✨ 已复制").color(theme::SUCCESS_COLOR).size(theme::FONT_SIZE_BODY));
        }
    });
    ui.add_space(8.0);

    // 渲染精致的 Toggle-Tab 选择控件
    ui.horizontal(|ui| {
        ui.style_mut().spacing.item_spacing.x = 2.0;

        let fl_selected = state.active_tool == ConfigTool::FlClash;
        let cv_selected = state.active_tool == ConfigTool::ClashVerge;

        let active_bg = theme::ACCENT_COLOR;
        let inactive_bg = theme::get_card_soft_color(dark_mode); // 统一用 soft 背景

        let active_fg = Color32::WHITE;
        let inactive_fg = theme::get_secondary_text_color(dark_mode);

        // FlClash 标签
        let fl_btn = egui::Button::new(
            RichText::new("FlClash 配置")
                .color(if fl_selected { active_fg } else { inactive_fg })
                .size(theme::FONT_SIZE_SMALL)
                .strong()
        )
        .fill(if fl_selected { active_bg } else { inactive_bg })
        .rounding(egui::Rounding {
            nw: theme::BUTTON_ROUNDING,
            sw: theme::BUTTON_ROUNDING,
            ne: 0.0,
            se: 0.0,
        });

        if ui.add(fl_btn).clicked() {
            state.active_tool = ConfigTool::FlClash;
        }

        // Clash Verge 标签
        let cv_btn = egui::Button::new(
            RichText::new("Clash Verge 覆写")
                .color(if cv_selected { active_fg } else { inactive_fg })
                .size(theme::FONT_SIZE_SMALL)
                .strong()
        )
        .fill(if cv_selected { active_bg } else { inactive_bg })
        .rounding(egui::Rounding {
            nw: 0.0,
            sw: 0.0,
            ne: theme::BUTTON_ROUNDING,
            se: theme::BUTTON_ROUNDING,
        });

        if ui.add(cv_btn).clicked() {
            state.active_tool = ConfigTool::ClashVerge;
        }
    });
    ui.add_space(8.0);

    // 代码框展示 (采用 CARD_SOFT 风格以区分外部卡片背景)
    egui::Frame::none()
        .fill(theme::get_card_soft_color(dark_mode))
        .stroke(egui::Stroke::new(
            1.0,
            if is_highlighted {
                theme::SUCCESS_COLOR
            } else {
                border_color
            },
        ))
        .rounding(theme::CARD_ROUNDING)
        .inner_margin(16.0) // 充足内边距
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            let response = ui.add(
                egui::Label::new(
                    RichText::new(&current_config)
                        .color(if is_highlighted {
                            text_color
                        } else {
                            secondary_color
                        })
                        .size(theme::FONT_SIZE_SMALL)
                        .family(egui::FontFamily::Monospace),
                )
                .wrap_mode(egui::TextWrapMode::Wrap)
                .sense(Sense::click()),
            );

            if response.clicked() {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
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
        RichText::new("💡 点击上方代码块可直接复制配置内容")
            .color(secondary_color)
            .size(theme::FONT_SIZE_SMALL),
    );
}


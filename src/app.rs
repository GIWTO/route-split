// route-split - 主应用逻辑

use eframe::egui::{self, CentralPanel, Color32, ComboBox, RichText, TopBottomPanel};
use std::time::{Duration, Instant};

use crate::network::{self, AdapterInfo};
use crate::registry;
use crate::ui::{
    cheatsheet::{self, CheatsheetState},
    log_panel::{self, LogManager},
    status_panel,
    theme::{self, DANGER_COLOR},
};

/// 应用状态
pub struct RouteSplitApp {
    is_admin: bool,
    route_exists: bool,
    proxy_enabled: bool,
    all_adapters: Vec<AdapterInfo>,
    internal_adapter_idx: Option<usize>,
    external_adapter_idx: Option<usize>,
    log_manager: LogManager,
    cheatsheet_state: CheatsheetState,
    is_processing: bool,
    last_refresh: Instant,
    last_dark_mode: Option<bool>,
    target_dest: String,
    target_mask: String,
}

impl Default for RouteSplitApp {
    fn default() -> Self {
        let is_admin = network::is_admin();
        let all_adapters = network::get_all_adapters();

        let internal_adapter_idx = all_adapters
            .iter()
            .position(|a| a.gateway.starts_with("192.168."));
        let external_adapter_idx = all_adapters
            .iter()
            .position(|a| !a.gateway.starts_with("192.168."));

        let mut app = Self {
            is_admin,
            route_exists: false,
            proxy_enabled: false,
            all_adapters,
            internal_adapter_idx,
            external_adapter_idx,
            log_manager: LogManager::default(),
            cheatsheet_state: CheatsheetState::default(),
            is_processing: false,
            last_refresh: Instant::now(),
            last_dark_mode: None,
            target_dest: "10.0.0.0".to_string(),
            target_mask: "255.0.0.0".to_string(),
        };

        app.refresh_status();
        app.log_manager.info("应用启动，网卡列表已加载");
        app
    }
}

impl RouteSplitApp {
    fn refresh_status(&mut self) {
        if self.is_processing {
            return;
        }
        self.route_exists = network::check_route_exists(&self.target_dest);
        self.proxy_enabled = registry::get_proxy_enabled().unwrap_or(false);
    }

    fn refresh_adapters(&mut self) {
        self.all_adapters = network::get_all_adapters();
        self.log_manager.info("网卡列表已刷新");
    }

    fn do_fix(&mut self) {
        let idx = match self.internal_adapter_idx {
            Some(i) => i,
            None => {
                self.log_manager.error("❌ 请先选择内网网卡");
                return;
            }
        };

        let adapter = &self.all_adapters[idx];
        self.is_processing = true;
        self.log_manager.info(format!(
            "🚀 正在将内网流量指向: {} ({})",
            adapter.name, adapter.gateway
        ));

        let route_result =
            network::add_route(&self.target_dest, &self.target_mask, &adapter.gateway);
        if route_result.success {
            self.log_manager.ok(&route_result.message);
        } else {
            self.log_manager.error(&route_result.message);
        }

        let proxy_result = registry::set_proxy_enabled(false);
        if proxy_result.success {
            self.log_manager.ok("系统代理已停用");
        }

        let _ = network::flush_network();
        self.is_processing = false;
        self.refresh_status();
        self.log_manager.ok("✅ 配置完成!");
    }

    fn do_rollback(&mut self) {
        self.is_processing = true;
        let _ = network::delete_route(&self.target_dest);
        self.is_processing = false;
        self.refresh_status();
        self.log_manager.ok("已清除内网路由规则");
    }
}

impl eframe::App for RouteSplitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dark_mode = ctx.style().visuals.dark_mode;
        if self.last_dark_mode != Some(dark_mode) {
            ctx.set_visuals(theme::get_theme_visuals(dark_mode));
            self.last_dark_mode = Some(dark_mode);
        }

        // 降低刷新频率到 10 秒，减少卡顿
        if self.last_refresh.elapsed() >= Duration::from_secs(10) && !self.is_processing {
            self.refresh_status();
            self.last_refresh = Instant::now();
        }

        if !self.is_admin {
            TopBottomPanel::top("admin_warning")
                .frame(egui::Frame::none().fill(DANGER_COLOR).inner_margin(4.0))
                .show(ctx, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("🔒 权限受限：请以管理员身份运行")
                                .color(Color32::WHITE)
                                .size(13.0)
                                .strong(),
                        );
                    });
                });
        }

        CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(theme::get_bg_color(dark_mode))
                    .inner_margin(egui::Margin::ZERO),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .inner_margin(egui::Margin::symmetric(20.0, 16.0))
                            .show(ui, |ui| {
                                // 标题
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        RichText::new("ROUTE-SPLIT")
                                            .color(theme::ACCENT_COLOR)
                                            .size(11.0)
                                            .strong(),
                                    );
                                    ui.add_space(-4.0);
                                    ui.label(
                                        RichText::new("路由分流工具")
                                            .color(theme::get_text_color(dark_mode))
                                            .size(26.0)
                                            .strong(),
                                    );
                                });
                                ui.add_space(24.0);

                                // 1. 网卡配置
                                ui.label(
                                    RichText::new("🔌 网卡流向手动配置")
                                        .color(theme::get_text_color(dark_mode))
                                        .size(16.0)
                                        .strong(),
                                );
                                ui.add_space(8.0);
                                egui::Frame::group(ui.style()).rounding(8.0).show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    egui::Grid::new("adapters")
                                        .num_columns(2)
                                        .spacing([12.0, 16.0])
                                        .show(ui, |ui| {
                                            ui.label("🌐 外网出口卡:");
                                            ComboBox::from_id_salt("ext")
                                                .width(ui.available_width())
                                                .selected_text(
                                                    self.external_adapter_idx
                                                        .map(|i| self.all_adapters[i].name.as_str())
                                                        .unwrap_or("请选择..."),
                                                )
                                                .show_ui(ui, |ui| {
                                                    for (i, a) in
                                                        self.all_adapters.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut self.external_adapter_idx,
                                                            Some(i),
                                                            format!("{} ({})", a.name, a.ip),
                                                        );
                                                    }
                                                });
                                            ui.end_row();
                                            ui.label("🏢 内网分流卡:");
                                            ComboBox::from_id_salt("int")
                                                .width(ui.available_width())
                                                .selected_text(
                                                    self.internal_adapter_idx
                                                        .map(|i| self.all_adapters[i].name.as_str())
                                                        .unwrap_or("请选择..."),
                                                )
                                                .show_ui(ui, |ui| {
                                                    for (i, a) in
                                                        self.all_adapters.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut self.internal_adapter_idx,
                                                            Some(i),
                                                            format!("{} ({})", a.name, a.ip),
                                                        );
                                                    }
                                                });
                                            ui.end_row();
                                        });
                                    ui.add_space(8.0);
                                    if ui.button("🔄 刷新网卡列表").clicked() {
                                        self.refresh_adapters();
                                    }
                                });
                                ui.add_space(24.0);

                                // 2. 状态预览
                                status_panel::render_status_panel(
                                    ui,
                                    self.route_exists,
                                    self.proxy_enabled,
                                    &self
                                        .internal_adapter_idx
                                        .and_then(|i| self.all_adapters.get(i).cloned()),
                                    &self
                                        .external_adapter_idx
                                        .and_then(|i| self.all_adapters.get(i).cloned()),
                                    dark_mode,
                                );
                                ui.add_space(24.0);

                                // 3. 目标配置
                                ui.label(
                                    RichText::new("⚙ 自定义内网目标网段")
                                        .color(theme::get_text_color(dark_mode))
                                        .size(16.0)
                                        .strong(),
                                );
                                ui.add_space(8.0);
                                egui::Frame::group(ui.style()).rounding(8.0).show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.label("网段:");
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.target_dest)
                                                .desired_width(120.0),
                                        );
                                        ui.add_space(16.0);
                                        ui.label("掩码:");
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.target_mask)
                                                .desired_width(120.0),
                                        );
                                    });
                                });
                                ui.add_space(24.0);

                                // 4. 动作按钮
                                ui.horizontal(|ui| {
                                    // 主按钮 - 强调色背景
                                    let primary_btn = egui::Frame::none()
                                        .fill(theme::ACCENT_COLOR)
                                        .rounding(8.0)
                                        .inner_margin(egui::Margin::symmetric(28.0, 12.0))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new("🚀 一键配置")
                                                    .color(Color32::WHITE)
                                                    .size(15.0)
                                                    .strong(),
                                            );
                                        });
                                    if primary_btn
                                        .response
                                        .interact(egui::Sense::click())
                                        .clicked()
                                    {
                                        self.do_fix();
                                    }
                                    if primary_btn
                                        .response
                                        .interact(egui::Sense::hover())
                                        .hovered()
                                    {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    }

                                    ui.add_space(12.0);

                                    // 次要按钮 - 边框样式
                                    let secondary_btn = egui::Frame::none()
                                        .stroke(egui::Stroke::new(
                                            1.5,
                                            theme::get_border_color(dark_mode),
                                        ))
                                        .rounding(8.0)
                                        .inner_margin(egui::Margin::symmetric(28.0, 12.0))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new("↩ 回滚配置")
                                                    .color(theme::get_text_color(dark_mode))
                                                    .size(15.0),
                                            );
                                        });
                                    if secondary_btn
                                        .response
                                        .interact(egui::Sense::click())
                                        .clicked()
                                    {
                                        self.do_rollback();
                                    }
                                    if secondary_btn
                                        .response
                                        .interact(egui::Sense::hover())
                                        .hovered()
                                    {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    }
                                });
                                ui.add_space(24.0);

                                // 5. 指南与日志
                                cheatsheet::render_cheatsheet(
                                    ui,
                                    &mut self.cheatsheet_state,
                                    &self.target_dest,
                                );
                                ui.add_space(24.0);
                                log_panel::render_log_panel(ui, &self.log_manager);

                                ui.add_space(40.0);
                            });
                    });
            });
    }
}

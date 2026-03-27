// route-split - 主应用逻辑

use eframe::egui::{self, CentralPanel, Color32, ComboBox, RichText, TopBottomPanel};
use std::time::{Duration, Instant};

use crate::network::{self, AdapterInfo};
use crate::registry;
use crate::ui::{
    action_panel::{self, ActionResult},
    cheatsheet::{self, CheatsheetState},
    log_panel::{self, LogManager},
    status_panel,
    theme::{self, DANGER_COLOR},
};
use std::sync::mpsc::{self, Receiver, Sender};

/// 后台任务结果
enum TaskResult {
    FixComplete(network::RouteResult, bool), // 路由结果, 代理是否成功
    RollbackComplete(network::RouteResult),
    RefreshComplete(Vec<AdapterInfo>),
    PingResult(bool, Option<u32>), // (is_internal, rtt_ms)
    RoutesUpdate(Vec<network::RouteEntry>),
}

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
    last_monitor: Instant,
    last_dark_mode: Option<bool>,
    target_dest: String,
    target_mask: String,
    internal_ping: Option<u32>,
    external_ping: Option<u32>,
    active_routes: Vec<network::RouteEntry>,
    task_sender: Sender<TaskResult>,
    task_receiver: Receiver<TaskResult>,
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

        let (tx, rx) = mpsc::channel();

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
            last_monitor: Instant::now() - Duration::from_secs(10), // 启动即触发第一次探测
            last_dark_mode: None,
            target_dest: "10.0.0.0".to_string(),
            target_mask: "255.0.0.0".to_string(),
            internal_ping: None,
            external_ping: None,
            active_routes: Vec::new(),
            task_sender: tx,
            task_receiver: rx,
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

    fn check_task_results(&mut self) {
        while let Ok(result) = self.task_receiver.try_recv() {
            match result {
                TaskResult::FixComplete(route_res, proxy_ok) => {
                    if route_res.success {
                        self.log_manager.ok(&route_res.message);
                    } else {
                        self.log_manager.error(&route_res.message);
                    }
                    if proxy_ok {
                        self.log_manager.ok("系统代理已停用");
                    }
                    let _ = network::flush_network();
                    self.is_processing = false;
                    self.refresh_status();
                    self.log_manager.ok("✅ 配置完成!");
                }
                TaskResult::RollbackComplete(_res) => {
                    self.is_processing = false;
                    self.refresh_status();
                    self.log_manager.ok("已清除内网路由规则");
                }
                TaskResult::RefreshComplete(adapters) => {
                    self.all_adapters = adapters;
                    self.is_processing = false;
                    self.log_manager.info("网卡列表已刷新");
                }
                TaskResult::PingResult(is_internal, rtt) => {
                    if is_internal {
                        self.internal_ping = rtt;
                    } else {
                        self.external_ping = rtt;
                    }
                }
                TaskResult::RoutesUpdate(routes) => {
                    self.active_routes = routes;
                }
            }
        }
    }

    fn refresh_adapters(&mut self) {
        self.is_processing = true;
        let tx = self.task_sender.clone();
        std::thread::spawn(move || {
            let adapters = network::get_all_adapters();
            let _ = tx.send(TaskResult::RefreshComplete(adapters));
        });
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
        let gateway = adapter.gateway.clone();
        let name = adapter.name.clone();
        let target_dest = self.target_dest.clone();
        let target_mask = self.target_mask.clone();

        self.is_processing = true;
        self.log_manager.info(format!(
            "🚀 正在后台执行分流配置: {} ({})",
            name, gateway
        ));

        let tx = self.task_sender.clone();
        std::thread::spawn(move || {
            let route_result = network::add_route(&target_dest, &target_mask, &gateway);
            let proxy_result = registry::set_proxy_enabled(false);
            let _ = tx.send(TaskResult::FixComplete(route_result, proxy_result.success));
        });
    }

    fn do_rollback(&mut self) {
        self.is_processing = true;
        let target_dest = self.target_dest.clone();
        let tx = self.task_sender.clone();
        
        self.log_manager.info("🚀 正在后台恢复默认网络配置...");
        std::thread::spawn(move || {
            let res = network::delete_route(&target_dest);
            let _ = tx.send(TaskResult::RollbackComplete(res));
        });
    }
}

impl eframe::App for RouteSplitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dark_mode = ctx.style().visuals.dark_mode;
        if self.last_dark_mode != Some(dark_mode) {
            ctx.set_visuals(theme::get_theme_visuals(dark_mode));
            self.last_dark_mode = Some(dark_mode);
        }

        // 检查后台任务结果
        self.check_task_results();

        // 降低刷新频率到 10 秒，减少卡顿 (状态同步)
        if self.last_refresh.elapsed() >= Duration::from_secs(10) && !self.is_processing {
            self.refresh_status();
            self.last_refresh = Instant::now();
        }

        // 监测循环：每 5 秒 Ping 一次，每 10 秒过滤一次路由
        if self.last_monitor.elapsed() >= Duration::from_secs(5) && !self.is_processing {
            let tx = self.task_sender.clone();
            
            // 检测内网网关
            if let Some(idx) = self.internal_adapter_idx {
                let gateway = self.all_adapters[idx].gateway.clone();
                let tx_c = tx.clone();
                std::thread::spawn(move || {
                    let rtt = network::ping_gateway(&gateway);
                    let _ = tx_c.send(TaskResult::PingResult(true, rtt));
                });
            }

            // 检测外网网关
            if let Some(idx) = self.external_adapter_idx {
                let gateway = self.all_adapters[idx].gateway.clone();
                let tx_c = tx.clone();
                std::thread::spawn(move || {
                    let rtt = network::ping_gateway(&gateway);
                    let _ = tx_c.send(TaskResult::PingResult(false, rtt));
                });
            }

            // 过滤活跃路由 (针对 10.x.x.x)
            let tx_c = tx.clone();
            std::thread::spawn(move || {
                let routes = network::get_active_routes_filtered("10.");
                let _ = tx_c.send(TaskResult::RoutesUpdate(routes));
            });

            self.last_monitor = Instant::now();
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
                            .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                            .show(ui, |ui| {
                                // 标题部分
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new("ROUTE-SPLIT").color(theme::ACCENT_COLOR).size(10.0).strong());
                                    ui.add_space(-2.0);
                                    ui.label(RichText::new("路由分流工具").color(theme::get_text_color(dark_mode)).size(24.0).strong());
                                });
                                ui.add_space(32.0);

                                // 1. 快捷操作面板 (Top Actions)
                                let action = action_panel::render_action_panel(ui, self.is_admin, self.is_processing);
                                match action {
                                    ActionResult::Fix => self.do_fix(),
                                    ActionResult::Rollback => self.do_rollback(),
                                    ActionResult::Refresh => self.refresh_adapters(),
                                    _ => {}
                                }
                                ui.add_space(24.0);

                                // 2. 状态监测面板 (Status & Cards)
                                status_panel::render_status_panel(
                                    ui,
                                    self.route_exists,
                                    self.proxy_enabled,
                                    &self.internal_adapter_idx.and_then(|i| self.all_adapters.get(i).cloned()),
                                    &self.external_adapter_idx.and_then(|i| self.all_adapters.get(i).cloned()),
                                    self.internal_ping,
                                    self.external_ping,
                                    &self.active_routes,
                                    dark_mode,
                                );
                                ui.add_space(24.0);

                                // 3. 配置细节 (Adapters & Target)
                                ui.label(RichText::new("⚙ 配置详情").color(theme::get_text_color(dark_mode)).size(16.0).strong());
                                ui.add_space(8.0);
                                egui::Frame::none()
                                    .fill(theme::get_card_bg_color(dark_mode))
                                    .rounding(10.0)
                                    .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
                                    .inner_margin(16.0)
                                    .show(ui, |ui| {
                                        ui.set_width(ui.available_width());
                                        egui::Grid::new("config_grid").num_columns(2).spacing([12.0, 12.0]).show(ui, |ui| {
                                            ui.label("🌎 外网出口:");
                                            ComboBox::from_id_salt("ext")
                                                .width(ui.available_width())
                                                .selected_text(self.external_adapter_idx.map(|i| self.all_adapters[i].name.as_str()).unwrap_or("未选择"))
                                                .show_ui(ui, |ui| {
                                                    for (i, a) in self.all_adapters.iter().enumerate() {
                                                        ui.selectable_value(&mut self.external_adapter_idx, Some(i), format!("{} ({})", a.name, a.ip));
                                                    }
                                                });
                                            ui.end_row();

                                            ui.label("🏢 内网网段:");
                                            ComboBox::from_id_salt("int")
                                                .width(ui.available_width())
                                                .selected_text(self.internal_adapter_idx.map(|i| self.all_adapters[i].name.as_str()).unwrap_or("未选择"))
                                                .show_ui(ui, |ui| {
                                                    for (i, a) in self.all_adapters.iter().enumerate() {
                                                        ui.selectable_value(&mut self.internal_adapter_idx, Some(i), format!("{} ({})", a.name, a.ip));
                                                    }
                                                });
                                            ui.end_row();

                                            ui.label("🎯 目标地址:");
                                            ui.horizontal(|ui| {
                                                ui.add(egui::TextEdit::singleline(&mut self.target_dest).desired_width(100.0));
                                                ui.label("/");
                                                ui.add(egui::TextEdit::singleline(&mut self.target_mask).desired_width(100.0));
                                            });
                                            ui.end_row();
                                        });
                                    });
                                ui.add_space(32.0);

                                // 4. 指南与日志
                                cheatsheet::render_cheatsheet(ui, &mut self.cheatsheet_state, &self.target_dest);
                                ui.add_space(24.0);
                                log_panel::render_log_panel(ui, &self.log_manager);
                                ui.add_space(40.0);
                            });
                    });
            });
    }
}

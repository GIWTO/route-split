// 三网卡分流修复工具 - 程序入口
// Triple-Net Fixer: 解决 Windows 多网卡路由冲突问题

#![windows_subsystem = "windows"]

mod app;
mod network;
mod registry;
mod ui;

use app::TripleNetFixerApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // 窗口配置
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("三网卡分流修复工具")
            .with_inner_size([600.0, 800.0])
            .with_resizable(true)
            .with_maximize_button(true),
        ..Default::default()
    };

    // 启动应用
    eframe::run_native(
        "Triple-Net Fixer",
        options,
        Box::new(|cc| {
            // 配置中文字体
            ui::theme::setup_fonts(&cc.egui_ctx);

            // 仅初始化基础 Style (DPI 等)
            let mut style = (*cc.egui_ctx.style()).clone();
            ui::theme::configure_style(&mut style);
            cc.egui_ctx.set_style(style);

            Ok(Box::new(TripleNetFixerApp::default()))
        }),
    )
}

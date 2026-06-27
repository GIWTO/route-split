// route-split - 程序入口
// route-split: 解决 Windows 多网卡路由冲突问题

#![cfg_attr(windows, windows_subsystem = "windows")]

mod app;
mod network;
mod platform;
mod registry;
mod ui;

use app::RouteSplitApp;
use eframe::egui;

fn load_icon_data() -> Option<egui::IconData> {
    // 载入内嵌的图标数据并解析为 RGBA 格式 (自动根据文件头判断格式)
    let png_bytes = include_bytes!("../app.png");
    match image::load_from_memory(png_bytes) {
        Ok(image) => {
            let rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();
            Some(egui::IconData {
                rgba: rgba.into_raw(),
                width,
                height,
            })
        }
        Err(e) => {
            eprintln!("解析应用图标失败: {}", e);
            None
        }
    }
}

fn main() -> eframe::Result<()> {
    // 基础窗口配置
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("route-split")
        .with_inner_size([800.0, 920.0])
        .with_resizable(true)
        .with_maximize_button(true);

    // 尝试载入并设置窗口图标
    if let Some(icon) = load_icon_data() {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    // 启动应用
    eframe::run_native(
        "route-split",
        options,
        Box::new(|cc| {
            // 配置中文字体
            ui::theme::setup_fonts(&cc.egui_ctx);

            // 仅初始化基础 Style (DPI 等)
            let mut style = (*cc.egui_ctx.style()).clone();
            ui::theme::configure_style(&mut style);
            cc.egui_ctx.set_style(style);

            Ok(Box::new(RouteSplitApp::default()))
        }),
    )
}

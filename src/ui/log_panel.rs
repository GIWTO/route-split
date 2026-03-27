// 日志追踪区组件

use super::theme::{self, DANGER_COLOR, SUCCESS_COLOR, WARNING_COLOR};
use chrono::Local;
use eframe::egui::{self, RichText, ScrollArea, Ui};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Info,
    Ok,
    Warning,
    Error,
}

impl LogLevel {
    pub fn color(&self, dark_mode: bool) -> eframe::egui::Color32 {
        match self {
            LogLevel::Info => theme::get_secondary_text_color(dark_mode),
            LogLevel::Ok => SUCCESS_COLOR,
            LogLevel::Warning => WARNING_COLOR,
            LogLevel::Error => DANGER_COLOR,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Ok => "OK",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            timestamp: Local::now().format("%H:%M:%S").to_string(),
            level,
            message: message.into(),
        }
    }
}

pub struct LogManager {
    entries: Vec<LogEntry>,
    max_entries: usize,
}

impl Default for LogManager {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100,
        }
    }
}

impl LogManager {
    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) {
        let entry = LogEntry::new(level, message);
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Info, message);
    }

    pub fn ok(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Ok, message);
    }

    #[allow(dead_code)]
    pub fn warn(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Error, message);
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }
}

/// 渲染日志追踪区
pub fn render_log_panel(ui: &mut Ui, log_manager: &LogManager) {
    let dark_mode = ui.visuals().dark_mode;
    let text_color = theme::get_text_color(dark_mode);
    let secondary_color = theme::get_secondary_text_color(dark_mode);

    ui.label(
        RichText::new("📜 执行日志")
            .color(text_color)
            .size(16.0)
            .strong(),
    );
    ui.add_space(4.0);

    egui::Frame::none()
        .fill(ui.visuals().extreme_bg_color)
        .rounding(10.0)
        .stroke(egui::Stroke::new(1.0, theme::get_border_color(dark_mode)))
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width()); // 关键配置

            ScrollArea::vertical()
                .max_height(200.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width()); // 关键配置

                    if log_manager.entries().is_empty() {
                        ui.label(
                            RichText::new("暂无日志...")
                                .color(secondary_color)
                                .size(12.0),
                        );
                    } else {
                        for entry in log_manager.entries() {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(format!("[{}]", entry.timestamp))
                                        .color(secondary_color)
                                        .size(11.0)
                                        .family(egui::FontFamily::Monospace),
                                );

                                ui.label(
                                    RichText::new(format!("[{}]", entry.level.label()))
                                        .color(entry.level.color(dark_mode))
                                        .size(11.0)
                                        .strong()
                                        .family(egui::FontFamily::Monospace),
                                );

                                ui.label(
                                    RichText::new(&entry.message)
                                        .color(entry.level.color(dark_mode))
                                        .size(11.0)
                                        .family(egui::FontFamily::Monospace),
                                );
                            });
                        }
                    }
                });
        });
}

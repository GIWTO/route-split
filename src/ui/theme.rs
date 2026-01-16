// Terminal 风格主题配置

use eframe::egui::{Color32, FontData, FontDefinitions, FontFamily, Rounding, Style, Visuals};

/// 背景色 - 动态获取
pub fn get_bg_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(0x0F, 0x17, 0x2A)
    } else {
        Color32::from_rgb(0xF8, 0xFA, 0xFC)
    }
}

/// 卡片背景色 - 动态获取
pub fn get_card_bg_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(0x02, 0x06, 0x17)
    } else {
        Color32::from_rgb(0xFF, 0xFF, 0xFF)
    }
}

/// 文字色 - 动态获取
pub fn get_text_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(0xF8, 0xFA, 0xFC)
    } else {
        Color32::from_rgb(0x0F, 0x17, 0x2A)
    }
}

/// 次要文字色 - 动态获取
pub fn get_secondary_text_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(0x94, 0xA1, 0xB8)
    } else {
        Color32::from_rgb(0x64, 0x74, 0x8B)
    }
}

/// 边框色 - 动态获取
pub fn get_border_color(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(0x1F, 0x29, 0x37)
    } else {
        Color32::from_rgb(0xE2, 0xE8, 0xF0)
    }
}

pub const SUCCESS_COLOR: Color32 = Color32::from_rgb(0x10, 0xB9, 0x81);
pub const WARNING_COLOR: Color32 = Color32::from_rgb(0xF5, 0x9E, 0x0B);
pub const DANGER_COLOR: Color32 = Color32::from_rgb(0xF4, 0x3F, 0x5E);
pub const ACCENT_COLOR: Color32 = Color32::from_rgb(0x3B, 0x82, 0xF6);

pub fn get_theme_visuals(dark_mode: bool) -> Visuals {
    let mut visuals = if dark_mode {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    let bg = get_bg_color(dark_mode);
    let card_bg = get_card_bg_color(dark_mode);
    let text = get_text_color(dark_mode);
    let border = get_border_color(dark_mode);
    let secondary_text = get_secondary_text_color(dark_mode);

    visuals.panel_fill = bg;
    visuals.window_fill = bg;
    visuals.extreme_bg_color = card_bg;
    visuals.widgets.noninteractive.bg_fill = card_bg;
    visuals.widgets.noninteractive.bg_stroke.color = border;
    visuals.widgets.noninteractive.fg_stroke.color = secondary_text;
    visuals.widgets.inactive.bg_fill = if dark_mode {
        Color32::from_rgb(0x1F, 0x29, 0x37)
    } else {
        Color32::from_rgb(0xF1, 0xF5, 0xF9)
    };
    visuals.widgets.inactive.bg_stroke.color = border;
    visuals.widgets.inactive.fg_stroke.color = text;
    visuals.widgets.hovered.bg_fill = if dark_mode {
        Color32::from_rgb(0x37, 0x41, 0x51)
    } else {
        Color32::from_rgb(0xE2, 0xE8, 0xF0)
    };
    visuals.widgets.hovered.bg_stroke.color = ACCENT_COLOR;
    visuals.widgets.hovered.fg_stroke.color = text;
    visuals.widgets.active.bg_fill = ACCENT_COLOR;
    visuals.widgets.active.fg_stroke.color = Color32::WHITE;

    visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
    visuals.widgets.inactive.rounding = Rounding::same(8.0);
    visuals.widgets.hovered.rounding = Rounding::same(8.0);
    visuals.widgets.active.rounding = Rounding::same(8.0);
    visuals.window_rounding = Rounding::same(12.0);

    visuals
}

pub fn setup_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = FontDefinitions::default();
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\msyh.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
    ];

    let mut font_loaded = false;
    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts
                .font_data
                .insert("my_font".to_owned(), FontData::from_owned(font_data));
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "my_font".to_owned());
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, "my_font".to_owned());
            font_loaded = true;
            break;
        }
    }

    if font_loaded {
        ctx.set_fonts(fonts);
    }
}

pub fn configure_style(style: &mut Style) {
    style.spacing.item_spacing = eframe::egui::vec2(8.0, 12.0);
    style.spacing.button_padding = eframe::egui::vec2(20.0, 10.0);
    style.spacing.interact_size = eframe::egui::vec2(60.0, 28.0);

    // --- 极简现代滚动条配置 ---
    style.spacing.scroll.bar_width = 6.0;
    style.spacing.scroll.handle_min_length = 24.0;
    style.spacing.scroll.bar_inner_margin = 2.0;
    style.spacing.scroll.bar_outer_margin = 0.0;
}

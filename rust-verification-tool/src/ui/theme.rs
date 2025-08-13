use egui::{Color32, Rounding, Stroke, Style, Visuals};

/// 应用主题配置
pub struct AppTheme;

impl AppTheme {
    /// 深色主题
    pub fn dark() -> Visuals {
        let mut visuals = Visuals::dark();
        
        // 自定义颜色
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(40, 40, 40);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 50);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 60, 60);
        visuals.widgets.active.bg_fill = Color32::from_rgb(70, 70, 70);
        
        // 按钮颜色
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(45, 45, 45);
        visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(55, 55, 55);
        visuals.widgets.active.weak_bg_fill = Color32::from_rgb(65, 65, 65);
        
        // 文本颜色 - 暂时注释掉，使用默认值
        // visuals.widgets.noninteractive.text_color = Color32::from_rgb(220, 220, 220);
        // visuals.widgets.inactive.text_color = Color32::from_rgb(200, 200, 200);
        // visuals.widgets.hovered.text_color = Color32::from_rgb(240, 240, 240);
        // visuals.widgets.active.text_color = Color32::WHITE;
        
        // 边框和描边
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(80, 80, 80));
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(100, 100, 100));
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(120, 120, 120));
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(140, 140, 140));
        
        // 圆角
        visuals.widgets.noninteractive.rounding = Rounding::same(4.0);
        visuals.widgets.inactive.rounding = Rounding::same(4.0);
        visuals.widgets.hovered.rounding = Rounding::same(4.0);
        visuals.widgets.active.rounding = Rounding::same(4.0);
        
        // 窗口背景
        visuals.window_fill = Color32::from_rgb(35, 35, 35);
        visuals.panel_fill = Color32::from_rgb(40, 40, 40);
        
        visuals
    }

    /// 浅色主题
    pub fn light() -> Visuals {
        let mut visuals = Visuals::light();
        
        // 自定义颜色
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(245, 245, 245);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(235, 235, 235);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(225, 225, 225);
        visuals.widgets.active.bg_fill = Color32::from_rgb(215, 215, 215);
        
        // 按钮颜色
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(240, 240, 240);
        visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(230, 230, 230);
        visuals.widgets.active.weak_bg_fill = Color32::from_rgb(220, 220, 220);
        
        // 文本颜色 - 暂时注释掉，使用默认值
        // visuals.widgets.noninteractive.text_color = Color32::from_rgb(60, 60, 60);
        // visuals.widgets.inactive.text_color = Color32::from_rgb(40, 40, 40);
        // visuals.widgets.hovered.text_color = Color32::from_rgb(20, 20, 20);
        // visuals.widgets.active.text_color = Color32::BLACK;
        
        // 边框和描边
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(200, 200, 200));
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(180, 180, 180));
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(160, 160, 160));
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(140, 140, 140));
        
        // 圆角
        visuals.widgets.noninteractive.rounding = Rounding::same(4.0);
        visuals.widgets.inactive.rounding = Rounding::same(4.0);
        visuals.widgets.hovered.rounding = Rounding::same(4.0);
        visuals.widgets.active.rounding = Rounding::same(4.0);
        
        // 窗口背景
        visuals.window_fill = Color32::from_rgb(250, 250, 250);
        visuals.panel_fill = Color32::from_rgb(245, 245, 245);
        
        visuals
    }

    /// 应用样式配置
    pub fn configure_style(style: &mut Style) {
        // 间距配置
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.menu_margin = egui::Margin::same(8.0);
        style.spacing.indent = 20.0;
        
        // 滚动条配置 - 暂时注释掉，使用默认值
        // style.spacing.scroll_bar_width = 12.0;
        // style.spacing.scroll_handle_min_length = 20.0;
        
        // 窗口配置
        style.spacing.window_margin = egui::Margin::same(8.0);
        
        // 文本选择颜色
        style.visuals.selection.bg_fill = Color32::from_rgb(100, 150, 200);
        style.visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(120, 170, 220));
    }
}

/// 颜色常量
pub struct Colors;

impl Colors {
    // 状态颜色
    pub const SUCCESS: Color32 = Color32::from_rgb(76, 175, 80);
    pub const WARNING: Color32 = Color32::from_rgb(255, 152, 0);
    pub const ERROR: Color32 = Color32::from_rgb(244, 67, 54);
    pub const INFO: Color32 = Color32::from_rgb(33, 150, 243);
    
    // 项目颜色
    pub const YCURSOR: Color32 = Color32::from_rgb(103, 58, 183);
    pub const YAUGMENT: Color32 = Color32::from_rgb(156, 39, 176);
    
    // 功能颜色
    pub const PRIMARY: Color32 = Color32::from_rgb(63, 81, 181);
    pub const SECONDARY: Color32 = Color32::from_rgb(96, 125, 139);
    pub const ACCENT: Color32 = Color32::from_rgb(255, 193, 7);
    
    // 背景颜色
    pub const BACKGROUND_DARK: Color32 = Color32::from_rgb(35, 35, 35);
    pub const BACKGROUND_LIGHT: Color32 = Color32::from_rgb(250, 250, 250);
    pub const SURFACE_DARK: Color32 = Color32::from_rgb(45, 45, 45);
    pub const SURFACE_LIGHT: Color32 = Color32::from_rgb(240, 240, 240);
}

/// 图标常量
pub struct Icons;

impl Icons {
    // 使用更兼容的字符，避免emoji显示问题
    pub const PLAY: &'static str = "▶";
    pub const PAUSE: &'static str = "||";
    pub const STOP: &'static str = "■";
    pub const REFRESH: &'static str = "↻";
    pub const SETTINGS: &'static str = "⚙";
    pub const SAVE: &'static str = "💾";
    pub const LOAD: &'static str = "📁";
    pub const CLEAR: &'static str = "🗑";
    pub const INFO: &'static str = "i";
    pub const WARNING: &'static str = "!";
    pub const ERROR: &'static str = "X";
    pub const SUCCESS: &'static str = "✓";
    pub const COPY: &'static str = "📋";
    pub const EXPORT: &'static str = "↗";
    pub const IMPORT: &'static str = "↙";
    pub const HELP: &'static str = "?";
    pub const CLOSE: &'static str = "X";
    pub const MINIMIZE: &'static str = "_";
    pub const MAXIMIZE: &'static str = "□";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme() {
        let visuals = AppTheme::dark();
        assert_eq!(visuals.dark_mode, true);
    }

    #[test]
    fn test_light_theme() {
        let visuals = AppTheme::light();
        assert_eq!(visuals.dark_mode, false);
    }

    #[test]
    fn test_colors_constants() {
        assert_ne!(Colors::SUCCESS, Colors::ERROR);
        assert_ne!(Colors::WARNING, Colors::INFO);
        assert_ne!(Colors::YCURSOR, Colors::YAUGMENT);
    }

    #[test]
    fn test_icons_constants() {
        assert_eq!(Icons::PLAY, "▶️");
        assert_eq!(Icons::SUCCESS, "✅");
        assert_eq!(Icons::ERROR, "❌");
    }

    #[test]
    fn test_style_configuration() {
        let mut style = Style::default();
        AppTheme::configure_style(&mut style);
        
        assert!(style.spacing.item_spacing.x > 0.0);
        assert!(style.spacing.button_padding.x > 0.0);
        assert!(style.spacing.scroll_bar_width > 0.0);
    }
}

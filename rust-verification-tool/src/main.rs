use eframe::egui;

mod app;
mod core;
mod ui;
mod config;
mod error;

use app::VerificationApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "验证码获取工具 - Rust版本",
        options,
        Box::new(|cc| {
            // 设置字体
            setup_custom_fonts(&cc.egui_ctx);

            // 设置主题
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Ok(Box::new(VerificationApp::new(cc)))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试加载Windows系统中文字体
    #[cfg(target_os = "windows")]
    {
        // 尝试加载微软雅黑字体
        if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc") {
            fonts.font_data.insert(
                "msyh".to_owned(),
                egui::FontData::from_owned(font_data),
            );

            // 将中文字体设置为最高优先级
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "msyh".to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("msyh".to_owned());

            ctx.set_fonts(fonts);
            return;
        }

        // 如果微软雅黑不可用，尝试宋体
        if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\simsun.ttc") {
            fonts.font_data.insert(
                "simsun".to_owned(),
                egui::FontData::from_owned(font_data),
            );

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "simsun".to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("simsun".to_owned());

            ctx.set_fonts(fonts);
            return;
        }
    }

    // 如果无法加载系统字体，使用默认字体
    println!("警告: 无法加载中文字体，将使用默认字体，中文可能显示为方块");
}

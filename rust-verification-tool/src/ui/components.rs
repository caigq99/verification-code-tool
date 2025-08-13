use crate::ui::theme::{Colors, Icons};
use egui::{Color32, RichText, Ui};



/// 进度指示器组件
pub struct ProgressIndicator {
    current: usize,
    total: usize,
    message: String,
    is_active: bool,
}

impl Default for ProgressIndicator {
    fn default() -> Self {
        Self {
            current: 0,
            total: 0,
            message: String::new(),
            is_active: false,
        }
    }
}

impl ProgressIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, total: usize, message: String) {
        self.current = 0;
        self.total = total;
        self.message = message;
        self.is_active = true;
    }

    pub fn update(&mut self, current: usize, message: Option<String>) {
        self.current = current;
        if let Some(msg) = message {
            self.message = msg;
        }
    }

    pub fn update_progress(&mut self, progress: f32, message: String) {
        self.current = (progress * self.total as f32) as usize;
        self.message = message;
    }

    pub fn finish(&mut self) {
        self.is_active = false;
        self.current = self.total;
    }

    pub fn show(&self, ui: &mut Ui) {
        if !self.is_active && self.total == 0 {
            return;
        }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(&self.message).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}/{}", self.current, self.total));
                });
            });

            let progress = if self.total > 0 {
                self.current as f32 / self.total as f32
            } else {
                0.0
            };

            let progress_bar = egui::ProgressBar::new(progress)
                .show_percentage()
                .animate(self.is_active);

            ui.add(progress_bar);
        });
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn progress(&self) -> f32 {
        if self.total > 0 {
            self.current as f32 / self.total as f32
        } else {
            0.0
        }
    }
}

/// 状态指示器组件
pub struct StatusIndicator {
    status: Status,
    message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Idle,
    Working,
    Success,
    Warning,
    Error,
}

impl Status {
    pub fn color(&self) -> Color32 {
        match self {
            Status::Idle => Color32::GRAY,
            Status::Working => Colors::INFO,
            Status::Success => Colors::SUCCESS,
            Status::Warning => Colors::WARNING,
            Status::Error => Colors::ERROR,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Status::Idle => "○",
            Status::Working => "...",
            Status::Success => Icons::SUCCESS,
            Status::Warning => Icons::WARNING,
            Status::Error => Icons::ERROR,
        }
    }
}

impl Default for StatusIndicator {
    fn default() -> Self {
        Self {
            status: Status::Idle,
            message: "就绪".to_string(),
        }
    }
}

impl StatusIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_status(&mut self, status: Status, message: String) {
        self.status = status;
        self.message = message;
    }

    pub fn set_idle(&mut self, message: Option<String>) {
        self.status = Status::Idle;
        self.message = message.unwrap_or_else(|| "就绪".to_string());
    }

    pub fn set_working(&mut self, message: String) {
        self.status = Status::Working;
        self.message = message;
    }

    pub fn set_success(&mut self, message: String) {
        self.status = Status::Success;
        self.message = message;
    }

    pub fn set_warning(&mut self, message: String) {
        self.status = Status::Warning;
        self.message = message;
    }

    pub fn set_error(&mut self, message: String) {
        self.status = Status::Error;
        self.message = message;
    }

    pub fn show(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(self.status.icon())
                    .color(self.status.color())
                    .size(16.0)
            );
            ui.label(
                RichText::new(&self.message)
                    .color(self.status.color())
            );
        });
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

/// 统计信息显示组件
pub struct StatsDisplay {
    total: usize,
    successful: usize,
    failed: usize,
    success_rate: f64,
}

impl Default for StatsDisplay {
    fn default() -> Self {
        Self {
            total: 0,
            successful: 0,
            failed: 0,
            success_rate: 0.0,
        }
    }
}

impl StatsDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, total: usize, successful: usize, failed: usize) {
        self.total = total;
        self.successful = successful;
        self.failed = failed;
        self.success_rate = if total > 0 {
            successful as f64 / total as f64 * 100.0
        } else {
            0.0
        };
    }

    pub fn show(&self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new("统计信息").strong().size(16.0));
            ui.separator();

            egui::Grid::new("stats_grid")
                .num_columns(2)
                .spacing([20.0, 8.0])
                .show(ui, |ui| {
                    ui.label("总请求:");
                    ui.label(RichText::new(self.total.to_string()).strong());
                    ui.end_row();

                    ui.label("成功:");
                    ui.label(
                        RichText::new(self.successful.to_string())
                            .color(Colors::SUCCESS)
                            .strong()
                    );
                    ui.end_row();

                    ui.label("失败:");
                    ui.label(
                        RichText::new(self.failed.to_string())
                            .color(Colors::ERROR)
                            .strong()
                    );
                    ui.end_row();

                    ui.label("成功率:");
                    ui.label(
                        RichText::new(format!("{:.1}%", self.success_rate))
                            .color(if self.success_rate >= 80.0 {
                                Colors::SUCCESS
                            } else if self.success_rate >= 50.0 {
                                Colors::WARNING
                            } else {
                                Colors::ERROR
                            })
                            .strong()
                    );
                    ui.end_row();
                });
        });
    }

    pub fn reset(&mut self) {
        self.total = 0;
        self.successful = 0;
        self.failed = 0;
        self.success_rate = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_selector_default() {
        let selector = ProjectSelector::default();
        assert_eq!(selector.get_selected().len(), 1);
        assert_eq!(selector.get_selected()[0], ProjectType::YCursor);
    }

    #[test]
    fn test_project_selector_operations() {
        let mut selector = ProjectSelector::new();

        // 测试选择所有项目
        selector.select_all();
        assert_eq!(selector.get_selected().len(), 2);
        assert!(!selector.is_empty());

        // 测试清空选择
        selector.clear();
        assert!(selector.is_empty());

        // 测试设置选择
        selector.set_selected(vec![ProjectType::YAugment]);
        assert_eq!(selector.get_selected().len(), 1);
        assert_eq!(selector.get_selected()[0], ProjectType::YAugment);
    }

    #[test]
    fn test_progress_indicator() {
        let mut progress = ProgressIndicator::new();
        assert!(!progress.is_active());
        assert_eq!(progress.progress(), 0.0);

        progress.start(10, "测试进度".to_string());
        assert!(progress.is_active());
        assert_eq!(progress.progress(), 0.0);

        progress.update(5, None);
        assert_eq!(progress.progress(), 0.5);

        progress.finish();
        assert!(!progress.is_active());
        assert_eq!(progress.progress(), 1.0);
    }

    #[test]
    fn test_status_indicator() {
        let mut status = StatusIndicator::new();
        assert_eq!(status.get_status(), &Status::Idle);

        status.set_working("正在工作".to_string());
        assert_eq!(status.get_status(), &Status::Working);
        assert_eq!(status.get_message(), "正在工作");

        status.set_success("成功完成".to_string());
        assert_eq!(status.get_status(), &Status::Success);

        status.set_error("发生错误".to_string());
        assert_eq!(status.get_status(), &Status::Error);
    }

    #[test]
    fn test_status_colors_and_icons() {
        assert_ne!(Status::Success.color(), Status::Error.color());
        assert_ne!(Status::Working.icon(), Status::Idle.icon());
        assert_eq!(Status::Success.icon(), Icons::SUCCESS);
        assert_eq!(Status::Error.icon(), Icons::ERROR);
    }

    #[test]
    fn test_stats_display() {
        let mut stats = StatsDisplay::new();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.success_rate, 0.0);

        stats.update(10, 8, 2);
        assert_eq!(stats.total, 10);
        assert_eq!(stats.successful, 8);
        assert_eq!(stats.failed, 2);
        assert_eq!(stats.success_rate, 80.0);

        stats.reset();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.success_rate, 0.0);
    }

    #[test]
    fn test_stats_success_rate_calculation() {
        let mut stats = StatsDisplay::new();

        // 测试100%成功率
        stats.update(5, 5, 0);
        assert_eq!(stats.success_rate, 100.0);

        // 测试50%成功率
        stats.update(4, 2, 2);
        assert_eq!(stats.success_rate, 50.0);

        // 测试0%成功率
        stats.update(3, 0, 3);
        assert_eq!(stats.success_rate, 0.0);

        // 测试空数据
        stats.update(0, 0, 0);
        assert_eq!(stats.success_rate, 0.0);
    }
}

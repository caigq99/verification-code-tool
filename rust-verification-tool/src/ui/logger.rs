use chrono::{DateTime, Local};
use egui::{Color32, RichText, ScrollArea};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
    Debug,
}

impl LogLevel {
    pub fn color(&self) -> Color32 {
        match self {
            LogLevel::Info => Color32::from_rgb(200, 200, 200),
            LogLevel::Warning => Color32::from_rgb(255, 165, 0),
            LogLevel::Error => Color32::from_rgb(255, 100, 100),
            LogLevel::Success => Color32::from_rgb(100, 255, 100),
            LogLevel::Debug => Color32::from_rgb(150, 150, 150),
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERROR]",
            LogLevel::Success => "[OK]",
            LogLevel::Debug => "[DEBUG]",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            timestamp: Local::now(),
            level,
            message,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "[{}] {} {}",
            self.timestamp.format("%H:%M:%S"),
            self.level.prefix(),
            self.message
        )
    }
}

/// 实时日志显示器
pub struct LogViewer {
    logs: Arc<Mutex<VecDeque<LogEntry>>>,
    max_logs: usize,
    auto_scroll: bool,
    filter_level: Option<LogLevel>,
}

impl Default for LogViewer {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl LogViewer {
    pub fn new(max_logs: usize) -> Self {
        Self {
            logs: Arc::new(Mutex::new(VecDeque::new())),
            max_logs,
            auto_scroll: true,
            filter_level: None,
        }
    }

    /// 添加日志条目
    pub fn add_log(&mut self, level: LogLevel, message: String) {
        let entry = LogEntry::new(level, message);
        
        if let Ok(mut logs) = self.logs.lock() {
            logs.push_back(entry);
            
            // 限制日志数量
            while logs.len() > self.max_logs {
                logs.pop_front();
            }
        }
    }

    /// 添加信息日志
    pub fn info(&mut self, message: impl Into<String>) {
        self.add_log(LogLevel::Info, message.into());
    }

    /// 添加警告日志
    pub fn warning(&mut self, message: impl Into<String>) {
        self.add_log(LogLevel::Warning, message.into());
    }

    /// 添加错误日志
    pub fn error(&mut self, message: impl Into<String>) {
        self.add_log(LogLevel::Error, message.into());
    }

    /// 添加成功日志
    pub fn success(&mut self, message: impl Into<String>) {
        self.add_log(LogLevel::Success, message.into());
    }

    /// 添加调试日志
    pub fn debug(&mut self, message: impl Into<String>) {
        self.add_log(LogLevel::Debug, message.into());
    }

    /// 清空日志
    pub fn clear(&mut self) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.clear();
        }
    }

    /// 获取日志副本
    pub fn get_logs(&self) -> Vec<LogEntry> {
        if let Ok(logs) = self.logs.lock() {
            logs.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// 过滤日志
    pub fn set_filter(&mut self, level: Option<LogLevel>) {
        self.filter_level = level;
    }

    /// 设置自动滚动
    pub fn set_auto_scroll(&mut self, auto_scroll: bool) {
        self.auto_scroll = auto_scroll;
    }

    /// 渲染日志视图
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("日志级别过滤:");
            
            if ui.button("全部").clicked() {
                self.filter_level = None;
            }
            
            if ui.button("信息").clicked() {
                self.filter_level = Some(LogLevel::Info);
            }
            
            if ui.button("警告").clicked() {
                self.filter_level = Some(LogLevel::Warning);
            }
            
            if ui.button("错误").clicked() {
                self.filter_level = Some(LogLevel::Error);
            }
            
            if ui.button("成功").clicked() {
                self.filter_level = Some(LogLevel::Success);
            }

            ui.separator();

            ui.checkbox(&mut self.auto_scroll, "自动滚动");

            if ui.button("清空日志").clicked() {
                self.clear();
            }
        });

        ui.separator();

        let logs = self.get_logs();
        let filtered_logs: Vec<_> = logs
            .iter()
            .filter(|entry| {
                if let Some(ref filter_level) = self.filter_level {
                    std::mem::discriminant(&entry.level) == std::mem::discriminant(filter_level)
                } else {
                    true
                }
            })
            .collect();

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(self.auto_scroll)
            .show(ui, |ui| {
                for entry in filtered_logs {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&entry.timestamp.format("%H:%M:%S").to_string())
                                .color(Color32::GRAY)
                                .monospace(),
                        );
                        
                        ui.label(
                            RichText::new(entry.level.prefix())
                                .color(entry.level.color()),
                        );
                        
                        ui.label(
                            RichText::new(&entry.message)
                                .color(entry.level.color()),
                        );
                    });
                }
            });
    }

    /// 获取日志统计信息
    pub fn get_stats(&self) -> LogStats {
        let logs = self.get_logs();
        let mut stats = LogStats::default();
        
        for entry in logs {
            match entry.level {
                LogLevel::Info => stats.info_count += 1,
                LogLevel::Warning => stats.warning_count += 1,
                LogLevel::Error => stats.error_count += 1,
                LogLevel::Success => stats.success_count += 1,
                LogLevel::Debug => stats.debug_count += 1,
            }
        }
        
        stats.total_count = stats.info_count + stats.warning_count + stats.error_count + stats.success_count + stats.debug_count;
        stats
    }
}

#[derive(Debug, Default)]
pub struct LogStats {
    pub total_count: usize,
    pub info_count: usize,
    pub warning_count: usize,
    pub error_count: usize,
    pub success_count: usize,
    pub debug_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(LogLevel::Info, "Test message".to_string());
        assert_eq!(entry.message, "Test message");
        assert!(matches!(entry.level, LogLevel::Info));
    }

    #[test]
    fn test_log_entry_format() {
        let entry = LogEntry::new(LogLevel::Success, "Test success".to_string());
        let formatted = entry.format();
        assert!(formatted.contains("✅"));
        assert!(formatted.contains("Test success"));
    }

    #[test]
    fn test_log_viewer_add_log() {
        let mut viewer = LogViewer::new(10);
        viewer.info("Test info message");
        viewer.error("Test error message");
        
        let logs = viewer.get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message, "Test info message");
        assert_eq!(logs[1].message, "Test error message");
    }

    #[test]
    fn test_log_viewer_max_logs() {
        let mut viewer = LogViewer::new(2);
        viewer.info("Message 1");
        viewer.info("Message 2");
        viewer.info("Message 3");
        
        let logs = viewer.get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message, "Message 2");
        assert_eq!(logs[1].message, "Message 3");
    }

    #[test]
    fn test_log_viewer_clear() {
        let mut viewer = LogViewer::new(10);
        viewer.info("Test message");
        assert_eq!(viewer.get_logs().len(), 1);
        
        viewer.clear();
        assert_eq!(viewer.get_logs().len(), 0);
    }

    #[test]
    fn test_log_stats() {
        let mut viewer = LogViewer::new(10);
        viewer.info("Info 1");
        viewer.info("Info 2");
        viewer.error("Error 1");
        viewer.success("Success 1");
        
        let stats = viewer.get_stats();
        assert_eq!(stats.total_count, 4);
        assert_eq!(stats.info_count, 2);
        assert_eq!(stats.error_count, 1);
        assert_eq!(stats.success_count, 1);
    }

    #[test]
    fn test_log_level_colors() {
        assert_ne!(LogLevel::Info.color(), LogLevel::Error.color());
        assert_ne!(LogLevel::Warning.color(), LogLevel::Success.color());
    }

    #[test]
    fn test_log_level_prefixes() {
        assert_eq!(LogLevel::Info.prefix(), "ℹ️");
        assert_eq!(LogLevel::Error.prefix(), "❌");
        assert_eq!(LogLevel::Success.prefix(), "✅");
        assert_eq!(LogLevel::Warning.prefix(), "⚠️");
        assert_eq!(LogLevel::Debug.prefix(), "🔍");
    }
}

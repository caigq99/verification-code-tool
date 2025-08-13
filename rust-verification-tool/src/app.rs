use crate::config::{Config, ProjectType};
use crate::core::verification::{VerificationCodeGetter, VerificationResult};
use crate::ui::{
    components::{ProgressIndicator, StatsDisplay, StatusIndicator},
    logger::{LogLevel, LogViewer},
    theme::{AppTheme, Colors, Icons},
};
use egui::{CentralPanel, Context, RichText, SidePanel, TopBottomPanel};
use std::sync::mpsc::{self, Receiver, Sender};
use tokio::runtime::Runtime;
use arboard::Clipboard;

/// 应用程序状态
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Idle,
    SingleGetting,
}

/// 任务消息
#[derive(Debug, Clone)]
pub enum TaskMessage {
    SingleResult(Result<VerificationResult, String>),
    LogMessage(LogLevel, String),
    ProgressUpdate(f32, String), // 进度百分比和描述
}

/// 主应用程序
pub struct VerificationApp {
    // 核心组件
    config: Config,
    runtime: Runtime,
    
    // UI状态
    state: AppState,
    dark_mode: bool,
    
    // UI组件
    log_viewer: LogViewer,
    progress_indicator: ProgressIndicator,
    status_indicator: StatusIndicator,
    stats_display: StatsDisplay,
    
    // 任务通信
    task_sender: Option<Sender<TaskMessage>>,
    task_receiver: Option<Receiver<TaskMessage>>,

    // 结果数据
    current_results: Vec<VerificationResult>,
    
    // 窗口状态
    show_settings: bool,
    show_about: bool,
}

impl VerificationApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::default();
        let runtime = Runtime::new().expect("Failed to create tokio runtime");
        
        let (sender, receiver) = mpsc::channel();
        
        Self {
            config,
            runtime,
            state: AppState::Idle,
            dark_mode: true,
            log_viewer: LogViewer::default(),
            progress_indicator: ProgressIndicator::new(),
            status_indicator: StatusIndicator::new(),
            stats_display: StatsDisplay::new(),
            task_sender: Some(sender),
            task_receiver: Some(receiver),
            current_results: Vec::new(),
            show_settings: false,
            show_about: false,
        }
    }

    /// 处理任务消息
    fn handle_task_messages(&mut self) {
        if let Some(receiver) = &self.task_receiver {
            while let Ok(message) = receiver.try_recv() {
                match message {
                    TaskMessage::SingleResult(result) => {
                        match result {
                            Ok(verification_result) => {
                                self.log_viewer.success(format!(
                                    "✅ {} 验证码获取成功: {}",
                                    verification_result.project,
                                    verification_result.code
                                ));

                                // 复制验证码到剪贴板
                                match Clipboard::new() {
                                    Ok(mut clipboard) => {
                                        if let Err(e) = clipboard.set_text(&verification_result.code) {
                                            self.log_viewer.warning(format!("⚠️ 复制到剪贴板失败: {}", e));
                                        } else {
                                            self.log_viewer.success("📋 验证码已自动复制到剪贴板".to_string());
                                        }
                                    }
                                    Err(e) => {
                                        self.log_viewer.warning(format!("⚠️ 无法访问剪贴板: {}", e));
                                    }
                                }

                                self.current_results.push(verification_result.clone());
                                self.status_indicator.set_success(format!(
                                    "获取成功: {} (已复制到剪贴板)",
                                    verification_result.code
                                ));
                                self.stats_display.update(
                                    self.current_results.len(),
                                    self.current_results.iter().filter(|r| r.success).count(),
                                    self.current_results.iter().filter(|r| !r.success).count(),
                                );
                            }
                            Err(error) => {
                                self.log_viewer.error(format!("❌ 验证码获取失败: {}", error));
                                self.status_indicator.set_error(format!("获取失败: {}", error));
                            }
                        }
                        self.state = AppState::Idle;
                        self.progress_indicator.finish();
                    }

                    TaskMessage::LogMessage(level, message) => {
                        self.log_viewer.add_log(level, message);
                    }
                    TaskMessage::ProgressUpdate(progress, description) => {
                        self.progress_indicator.update_progress(progress, description);
                    }
                }
            }
        }
    }

    /// 开始单个验证码获取
    fn start_single_get(&mut self, project: ProjectType) {
        if self.state != AppState::Idle {
            return;
        }

        self.state = AppState::SingleGetting;
        self.status_indicator.set_working(format!("正在获取 {} 验证码...", project));
        self.progress_indicator.start(1, format!("获取 {} 验证码", project));
        self.log_viewer.info(format!("开始获取 {} 验证码...", project));

        if let Some(sender) = &self.task_sender {
            let sender = sender.clone();
            let config = self.config.clone();
            
            self.runtime.spawn(async move {
                let result = match VerificationCodeGetter::new(config) {
                    Ok(mut getter) => {
                        // 创建进度回调
                        let progress_sender = sender.clone();
                        let progress_callback = move |progress: f32, description: String| {
                            let _ = progress_sender.send(TaskMessage::ProgressUpdate(progress, description));
                        };

                        // 创建日志回调
                        let log_sender = sender.clone();
                        let log_callback = move |level: crate::ui::LogLevel, message: String| {
                            let _ = log_sender.send(TaskMessage::LogMessage(level, message));
                        };

                        match getter.get_code_for_project_with_progress(project, Box::new(progress_callback), Box::new(log_callback)).await {
                            Ok(result) => Ok(result),
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    Err(e) => Err(e.to_string()),
                };

                let _ = sender.send(TaskMessage::SingleResult(result));
            });
        }
    }





    /// 清空结果
    fn clear_results(&mut self) {
        self.current_results.clear();
        self.stats_display.reset();
        self.log_viewer.clear();
        self.status_indicator.set_idle(None);
        self.log_viewer.info("已清空所有结果");
    }


}

impl eframe::App for VerificationApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // 处理任务消息
        self.handle_task_messages();

        // 设置主题
        if self.dark_mode {
            ctx.set_visuals(AppTheme::dark());
        } else {
            ctx.set_visuals(AppTheme::light());
        }

        // 顶部菜单栏
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button(format!("{} 清空结果", Icons::CLEAR)).clicked() {
                        self.clear_results();
                        ui.close_menu();
                    }
                });

                ui.menu_button("设置", |ui| {
                    if ui.button(format!("{} 偏好设置", Icons::SETTINGS)).clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.checkbox(&mut self.dark_mode, "深色模式").clicked() {
                        ui.close_menu();
                    }
                });

                ui.menu_button("帮助", |ui| {
                    if ui.button(format!("{} 关于", Icons::INFO)).clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.status_indicator.show(ui);
                });
            });
        });

        // 左侧控制面板
        SidePanel::left("control_panel")
            .resizable(true)
            .default_width(300.0)
            .width_range(250.0..=400.0)
            .show(ctx, |ui| {
                ui.heading(RichText::new("验证码获取工具").size(18.0).strong());
                ui.separator();

                // 验证码获取
                ui.group(|ui| {
                    ui.label(RichText::new("验证码获取").strong());
                    ui.vertical(|ui| {
                        let ycursor_enabled = self.state == AppState::Idle;
                        if ui.add_enabled(ycursor_enabled,
                            egui::Button::new(format!("{} 获取 YCursor 验证码", Icons::PLAY))
                                .fill(Colors::YCURSOR)
                                .min_size([200.0, 40.0].into())
                        ).clicked() {
                            self.start_single_get(ProjectType::YCursor);
                        }

                        ui.add_space(8.0);

                        let yaugment_enabled = self.state == AppState::Idle;
                        if ui.add_enabled(yaugment_enabled,
                            egui::Button::new(format!("{} 获取 YAugment 验证码", Icons::PLAY))
                                .fill(Colors::YAUGMENT)
                                .min_size([200.0, 40.0].into())
                        ).clicked() {
                            self.start_single_get(ProjectType::YAugment);
                        }
                    });
                });

                ui.separator();

                // 进度指示器
                if self.progress_indicator.is_active() {
                    ui.group(|ui| {
                        ui.label(RichText::new("进度").strong());
                        self.progress_indicator.show(ui);
                    });
                    ui.separator();
                }

                // 统计信息
                self.stats_display.show(ui);
            });

        // 主内容区域 - 日志显示
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("实时日志");
            ui.separator();
            self.log_viewer.show(ui);
        });

        // 设置窗口
        if self.show_settings {
            egui::Window::new(format!("{} 设置", Icons::SETTINGS))
                .open(&mut self.show_settings)
                .default_size([400.0, 300.0])
                .show(ctx, |ui| {
                    ui.label("配置选项:");
                    ui.separator();

                    ui.checkbox(&mut self.dark_mode, "深色模式");

                    ui.horizontal(|ui| {
                        ui.label("请求超时(秒):");
                        ui.add(egui::Slider::new(&mut self.config.request.timeout, 10..=120));
                    });

                    ui.horizontal(|ui| {
                        ui.label("重试次数:");
                        ui.add(egui::Slider::new(&mut self.config.request.retry_times, 1..=5));
                    });

                    ui.horizontal(|ui| {
                        ui.label("模拟延时(秒):");
                        ui.add(egui::Slider::new(&mut self.config.default.simulate_delay, 1..=10));
                    });
                });
        }

        // 关于窗口
        if self.show_about {
            egui::Window::new(format!("{} 关于", Icons::INFO))
                .open(&mut self.show_about)
                .default_size([400.0, 200.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("验证码获取工具 - Rust版本");
                        ui.separator();
                        ui.label("版本: 1.0.0");
                        ui.label("基于 egui 构建的现代化验证码获取工具");
                        ui.separator();
                        ui.label("支持 YCursor 和 YAugment 项目");
                        ui.label("具有实时日志、详细进度显示等功能");
                    });
                });
        }

        // 请求重绘以保持UI响应
        ctx.request_repaint();
    }
}

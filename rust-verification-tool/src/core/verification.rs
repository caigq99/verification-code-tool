use crate::config::{Config, ProjectType};
use crate::core::client::ApiClient;
use crate::error::{Result, VerificationError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

/// 验证码结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub code: String,
    pub project: ProjectType,
    pub expires_at: Option<DateTime<Utc>>,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub success: bool,
}



/// 验证码获取器
pub struct VerificationCodeGetter {
    client: ApiClient,
    config: Config,
}

impl VerificationCodeGetter {
    pub fn new(config: Config) -> Result<Self> {
        let client = ApiClient::new(config.clone())?;
        Ok(Self { client, config })
    }

    /// 获取单个验证码
    pub async fn get_code_for_project(&mut self, project: ProjectType) -> Result<VerificationResult> {
        self.get_code_for_project_with_progress(
            project,
            Box::new(|_, _| {}),
            Box::new(|level, message| {
                match level {
                    crate::ui::LogLevel::Info => log::info!("{}", message),
                    crate::ui::LogLevel::Success => log::info!("{}", message),
                    crate::ui::LogLevel::Warning => log::warn!("{}", message),
                    crate::ui::LogLevel::Error => log::error!("{}", message),
                    crate::ui::LogLevel::Debug => log::debug!("{}", message),
                }
            })
        ).await
    }

    /// 获取单个验证码（带进度回调）
    pub async fn get_code_for_project_with_progress(
        &mut self,
        project: ProjectType,
        progress_callback: Box<dyn Fn(f32, String) + Send>,
        log_callback: Box<dyn Fn(crate::ui::LogLevel, String) + Send>
    ) -> Result<VerificationResult> {
        log_callback(crate::ui::LogLevel::Info, format!("🚀 开始获取 {} 验证码...", project));
        log_callback(crate::ui::LogLevel::Info, "=".repeat(50));

        let start_time = Utc::now();

        // 步骤1: 获取签名密钥 (20%)
        progress_callback(0.0, "获取API签名密钥...".to_string());
        log_callback(crate::ui::LogLevel::Info, "📋 步骤 1/5: 获取API签名密钥...".to_string());
        self.client.fetch_signature_key().await?;
        log_callback(crate::ui::LogLevel::Success, "✅ 签名密钥获取完成".to_string());
        progress_callback(0.2, "签名密钥获取完成".to_string());

        // 步骤2: 创建广告会话 (40%)
        progress_callback(0.2, "创建广告会话...".to_string());
        log_callback(crate::ui::LogLevel::Info, "🔗 步骤 2/5: 创建广告会话...".to_string());
        let session_data = self.client.create_ad_session(project.as_str()).await?;
        let session_id = session_data["session_id"]
            .as_str()
            .ok_or_else(|| VerificationError::session("会话ID不存在"))?
            .to_string();
        log_callback(crate::ui::LogLevel::Success, format!("✅ 广告会话创建成功: {}", session_id));
        progress_callback(0.4, "广告会话创建成功".to_string());

        // 步骤3: 模拟观看广告 (60%)
        progress_callback(0.4, format!("模拟观看广告 ({}秒)...", self.config.default.simulate_delay));
        log_callback(crate::ui::LogLevel::Info, format!("📺 步骤 3/5: 模拟观看广告 ({}秒)...", self.config.default.simulate_delay));
        sleep(Duration::from_secs(self.config.default.simulate_delay)).await;
        log_callback(crate::ui::LogLevel::Success, "✅ 广告观看模拟完成".to_string());
        progress_callback(0.6, "广告观看模拟完成".to_string());

        // 步骤4: 验证广告完成 (80%)
        progress_callback(0.6, "验证广告观看完成...".to_string());
        log_callback(crate::ui::LogLevel::Info, "🔍 步骤 4/5: 验证广告观看完成...".to_string());
        let verify_data = self.client
            .verify_ad_completion(&session_id, self.config.default.watch_duration)
            .await?;

        let temp_token = verify_data["temp_token"]
            .as_str()
            .ok_or_else(|| VerificationError::api("临时令牌不存在"))?;
        log_callback(crate::ui::LogLevel::Success, "✅ 广告观看验证完成，获得临时令牌".to_string());
        progress_callback(0.8, "广告观看验证完成".to_string());

        // 步骤5: 获取验证码 (100%)
        progress_callback(0.8, "获取最终验证码...".to_string());
        log_callback(crate::ui::LogLevel::Info, "🎯 步骤 5/5: 获取最终验证码...".to_string());
        let code_data = self.client
            .get_verification_code(project.as_str(), &session_id, temp_token)
            .await?;

        let code = code_data["code"]
            .as_str()
            .ok_or_else(|| VerificationError::code("验证码不存在"))?
            .to_string();

        let expires_at = code_data["expires_at"]
            .as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // 获取额外信息
        let usage_count = code_data["usage_count"].as_u64();
        let max_usage = code_data["max_usage"].as_u64();
        let remaining_time = code_data["remaining_time"].as_u64();

        log_callback(crate::ui::LogLevel::Success, format!("🎉 {} 验证码获取成功!", project));
        log_callback(crate::ui::LogLevel::Info, format!("📝 验证码: {}", code));
        if let Some(expires) = expires_at {
            log_callback(crate::ui::LogLevel::Info, format!("⏰ 过期时间: {}", expires.format("%Y-%m-%d %H:%M:%S UTC")));
        }
        if let (Some(used), Some(max)) = (usage_count, max_usage) {
            log_callback(crate::ui::LogLevel::Info, format!("📊 使用情况: {}/{}", used, max));
        }
        if let Some(remaining) = remaining_time {
            log_callback(crate::ui::LogLevel::Info, format!("⏳ 剩余时间: {}秒", remaining));
        }

        let duration = Utc::now().signed_duration_since(start_time);
        log_callback(crate::ui::LogLevel::Info, format!("⚡ 总耗时: {:.2}秒", duration.num_milliseconds() as f64 / 1000.0));
        log_callback(crate::ui::LogLevel::Info, "=".repeat(50));

        progress_callback(1.0, format!("验证码获取完成: {}", code));

        Ok(VerificationResult {
            code,
            project,
            expires_at,
            session_id,
            created_at: start_time,
            success: true,
        })
    }



    /// 获取公告信息
    pub async fn get_notice(&self) -> Result<Vec<String>> {
        let notice_data = self.client.get_notice().await?;
        
        if let Some(notices) = notice_data.as_array() {
            let mut notice_list = Vec::new();
            for notice in notices {
                if let Some(content) = notice.get("content").and_then(|c| c.as_str()) {
                    notice_list.push(content.to_string());
                }
            }
            Ok(notice_list)
        } else {
            Ok(vec!["暂无公告".to_string()])
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use tokio_test;

    #[tokio::test]
    async fn test_verification_result_creation() {
        let result = VerificationResult {
            code: "TEST123".to_string(),
            project: ProjectType::YCursor,
            expires_at: Some(Utc::now()),
            session_id: "test_session".to_string(),
            created_at: Utc::now(),
            success: true,
        };

        assert_eq!(result.code, "TEST123");
        assert_eq!(result.project, ProjectType::YCursor);
        assert!(result.success);
    }



    #[tokio::test]
    async fn test_verification_code_getter_creation() {
        let config = Config::default();
        let result = VerificationCodeGetter::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_project_type_display() {
        assert_eq!(ProjectType::YCursor.to_string(), "YCursor");
        assert_eq!(ProjectType::YAugment.to_string(), "YAugment");
    }

    #[test]
    fn test_project_type_from_str() {
        use std::str::FromStr;

        assert!(ProjectType::from_str("YCursor").is_ok());
        assert!(ProjectType::from_str("YAugment").is_ok());
        assert!(ProjectType::from_str("Invalid").is_err());
    }
}

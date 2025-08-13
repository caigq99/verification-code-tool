use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub device: DeviceConfig,
    pub request: RequestConfig,
    pub default: DefaultConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub endpoints: HashMap<String, String>,
    pub default_signature_key: String,
    pub key_cache_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub model: String,
    pub platform: String,
    pub version: String,
    pub system: String,
    pub language: String,
    pub screen_width: u32,
    pub screen_height: u32,
    pub pixel_ratio: u32,
    pub brand: String,
    pub app_id: String,
    pub env_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestConfig {
    pub timeout: u64,
    pub retry_times: u32,
    pub retry_delay: u64,
    pub user_agent: String,
    pub referer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub watch_duration: u32,
    pub completion_proof: String,
    pub simulate_delay: u64,
}

impl Default for Config {
    fn default() -> Self {
        let mut endpoints = HashMap::new();
        endpoints.insert("notice".to_string(), "/XiaoChengXu/notice.json".to_string());
        endpoints.insert("signature_key".to_string(), "/api/public/signature-key".to_string());
        endpoints.insert("ad_session".to_string(), "/api/public/ad-session".to_string());
        endpoints.insert("ad_verify".to_string(), "/api/public/ad-verify".to_string());
        endpoints.insert("verification_code".to_string(), "/api/public/verification-code".to_string());
        endpoints.insert("config".to_string(), "/api/public/config".to_string());

        Self {
            api: ApiConfig {
                base_url: "https://app.yan.vin".to_string(),
                endpoints,
                default_signature_key: "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210".to_string(),
                key_cache_duration: 300,
            },
            device: DeviceConfig {
                model: "iPhone 14".to_string(),
                platform: "ios".to_string(),
                version: "3.5.5".to_string(),
                system: "iOS 16.0".to_string(),
                language: "zh_CN".to_string(),
                screen_width: 390,
                screen_height: 844,
                pixel_ratio: 3,
                brand: "Apple".to_string(),
                app_id: "wx421aabd7feefa0ed".to_string(),
                env_version: "release".to_string(),
            },
            request: RequestConfig {
                timeout: 30,
                retry_times: 3,
                retry_delay: 2,
                user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15".to_string(),
                referer: "https://servicewechat.com/wx421aabd7feefa0ed/devtools/page-frame.html".to_string(),
            },
            default: DefaultConfig {
                watch_duration: 30000,
                completion_proof: "miniprogram_ad_completed".to_string(),
                simulate_delay: 3,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectType {
    YCursor,
    YAugment,
}

impl ProjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectType::YCursor => "YCursor",
            ProjectType::YAugment => "YAugment",
        }
    }
    
    pub fn all() -> Vec<ProjectType> {
        vec![ProjectType::YCursor, ProjectType::YAugment]
    }
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ProjectType {
    type Err = crate::error::VerificationError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "YCursor" => Ok(ProjectType::YCursor),
            "YAugment" => Ok(ProjectType::YAugment),
            _ => Err(crate::error::VerificationError::config(format!("不支持的项目类型: {}", s))),
        }
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("网络请求失败: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("JSON解析失败: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("加密操作失败: {message}")]
    CryptoError { message: String },
    
    #[error("API错误: {message}")]
    ApiError { message: String },
    
    #[error("签名生成失败: {message}")]
    SignatureError { message: String },
    
    #[error("解密失败: {message}")]
    DecryptionError { message: String },
    
    #[error("会话创建失败: {message}")]
    SessionError { message: String },
    
    #[error("验证码获取失败: {message}")]
    CodeError { message: String },
    
    #[error("频率限制: {message}")]
    RateLimitError { message: String },
    
    #[error("配置错误: {message}")]
    ConfigError { message: String },
    
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("未知错误: {message}")]
    Unknown { message: String },
}

impl VerificationError {
    pub fn crypto(message: impl Into<String>) -> Self {
        Self::CryptoError {
            message: message.into(),
        }
    }
    
    pub fn api(message: impl Into<String>) -> Self {
        Self::ApiError {
            message: message.into(),
        }
    }
    
    pub fn signature(message: impl Into<String>) -> Self {
        Self::SignatureError {
            message: message.into(),
        }
    }
    
    pub fn decryption(message: impl Into<String>) -> Self {
        Self::DecryptionError {
            message: message.into(),
        }
    }
    
    pub fn session(message: impl Into<String>) -> Self {
        Self::SessionError {
            message: message.into(),
        }
    }
    
    pub fn code(message: impl Into<String>) -> Self {
        Self::CodeError {
            message: message.into(),
        }
    }
    
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::RateLimitError {
            message: message.into(),
        }
    }
    
    pub fn config(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }
    
    pub fn unknown(message: impl Into<String>) -> Self {
        Self::Unknown {
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, VerificationError>;

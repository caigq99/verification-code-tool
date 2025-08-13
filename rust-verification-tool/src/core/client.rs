use crate::config::Config;
use crate::core::crypto::{CryptoUtils, SignatureGenerator};
use crate::core::device::DeviceFingerprint;
use crate::error::{Result, VerificationError};
use reqwest::{Client, header::HeaderMap};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// HTTP客户端
pub struct ApiClient {
    client: Client,
    config: Config,
    signature_generator: Option<SignatureGenerator>,
    device_fingerprint: DeviceFingerprint,
    signature_key: Option<String>,
    key_version: u32,
    key_last_update: u64,
}

impl ApiClient {
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Referer", config.request.referer.parse().unwrap());

        let client = Client::builder()
            .timeout(Duration::from_secs(config.request.timeout))
            .user_agent(&config.request.user_agent)
            .default_headers(headers)
            .build()
            .map_err(|e| VerificationError::NetworkError(e))?;

        let device_fingerprint = DeviceFingerprint::new(config.device.clone());

        Ok(Self {
            client,
            config: config.clone(),
            signature_generator: None,
            device_fingerprint,
            signature_key: None,
            key_version: 1,
            key_last_update: 0,
        })
    }

    /// 获取签名密钥
    pub async fn fetch_signature_key(&mut self) -> Result<()> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 总是尝试获取新的签名密钥，不使用缓存

        let url = format!("{}{}",
            self.config.api.base_url,
            self.config.api.endpoints.get("signature_key").unwrap()
        );

        log::info!("尝试获取签名密钥: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VerificationError::NetworkError(e))?;

        log::info!("签名密钥响应状态码: {}", response.status());

        if response.status().is_success() {
            let data: Value = response.json().await
                .map_err(|e| VerificationError::NetworkError(e))?;

            if let (Some(key), Some(version)) = (data.get("key"), data.get("version")) {
                self.signature_key = Some(key.as_str().unwrap().to_string());
                self.key_version = version.as_u64().unwrap() as u32;
                self.key_last_update = current_time;
                
                self.signature_generator = Some(SignatureGenerator::new(
                    self.signature_key.as_ref().unwrap().clone()
                ));
                
                log::info!("✓ 获取签名密钥成功，版本: {}", self.key_version);
                return Ok(());
            }
        }

        // 使用默认密钥
        self.signature_key = Some(self.config.api.default_signature_key.clone());
        self.key_version = 1;
        self.key_last_update = current_time;
        
        self.signature_generator = Some(SignatureGenerator::new(
            self.signature_key.as_ref().unwrap().clone()
        ));
        
        log::warn!("⚠ 使用默认签名密钥");
        Ok(())
    }

    /// 生成带签名的请求头
    fn generate_signed_headers(&self, method: &str, path: &str, data: Option<&HashMap<String, Value>>) -> Result<HashMap<String, String>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        
        let nonce = CryptoUtils::generate_random_string(16);

        let signature = self.signature_generator
            .as_ref()
            .ok_or_else(|| VerificationError::signature("签名生成器未初始化"))?
            .generate_signature(method, path, data, &timestamp, &nonce)?;

        log::info!("生成签名: method={}, path={}, timestamp={}, nonce={}", method, path, timestamp, nonce);
        log::info!("签名结果: {}", signature);

        let mut headers = HashMap::new();
        headers.insert("X-YAN-Signature".to_string(), signature);
        headers.insert("X-YAN-Timestamp".to_string(), timestamp);
        headers.insert("X-YAN-Nonce".to_string(), nonce);

        Ok(headers)
    }

    /// 创建广告会话
    pub async fn create_ad_session(&mut self, project_id: &str) -> Result<Value> {
        self.fetch_signature_key().await?;

        let device_fingerprint = self.device_fingerprint.generate();
        let mut data = HashMap::new();
        data.insert("project_id".to_string(), json!(project_id));
        data.insert("device_fingerprint".to_string(), json!(device_fingerprint));

        let path = self.config.api.endpoints.get("ad_session").unwrap();
        let headers = self.generate_signed_headers("POST", path, Some(&data))?;
        let url = format!("{}{}", self.config.api.base_url, path);

        let mut request = self.client.post(&url);

        // 先设置自定义头，再设置JSON数据
        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        request = request.json(&data);

        let response = request.send().await
            .map_err(|e| VerificationError::NetworkError(e))?;

        let status_code = response.status().as_u16();
        let response_text = response.text().await
            .map_err(|e| VerificationError::NetworkError(e))?;

        log::info!("API响应状态码: {}", status_code);
        log::info!("API响应内容: {}", response_text);

        match status_code {
            200 => {
                let result: Value = serde_json::from_str(&response_text)
                    .map_err(|e| VerificationError::JsonError(e))?;

                if let Some(session_id) = result.get("session_id") {
                    log::info!("✓ 创建会话成功: {}", session_id.as_str().unwrap());
                    Ok(result)
                } else {
                    let error_msg = result.get("error")
                        .and_then(|e| e.as_str())
                        .unwrap_or("未知错误");
                    Err(VerificationError::session(format!("创建会话失败: {}", error_msg)))
                }
            }
            429 => Err(VerificationError::rate_limit("请求频率过高，请稍后重试")),
            _ => Err(VerificationError::session(format!("创建会话失败，状态码: {}, 响应: {}", status_code, response_text))),
        }
    }

    /// 验证广告观看完成
    pub async fn verify_ad_completion(&self, session_id: &str, watch_duration: u32) -> Result<Value> {
        let mut data = HashMap::new();
        data.insert("session_id".to_string(), json!(session_id));
        data.insert("watch_duration".to_string(), json!(watch_duration));
        data.insert("completion_proof".to_string(), json!(self.config.default.completion_proof));

        let path = self.config.api.endpoints.get("ad_verify").unwrap();
        let headers = self.generate_signed_headers("POST", path, Some(&data))?;
        let url = format!("{}{}", self.config.api.base_url, path);

        let mut request = self.client.post(&url);

        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        request = request.json(&data);

        let response = request.send().await
            .map_err(|e| VerificationError::NetworkError(e))?;

        match response.status().as_u16() {
            200 => {
                let result: Value = response.json().await
                    .map_err(|e| VerificationError::NetworkError(e))?;
                
                if result.get("verified").and_then(|v| v.as_bool()).unwrap_or(false) 
                    && result.get("temp_token").is_some() {
                    log::info!("✓ 广告验证成功");
                    Ok(result)
                } else {
                    let error_msg = result.get("error")
                        .and_then(|e| e.as_str())
                        .unwrap_or("未知错误");
                    Err(VerificationError::api(format!("广告验证失败: {}", error_msg)))
                }
            }
            429 => Err(VerificationError::rate_limit("请求频率过高，请稍后重试")),
            _ => Err(VerificationError::api(format!("广告验证失败，状态码: {}", response.status()))),
        }
    }

    /// 获取验证码
    pub async fn get_verification_code(&self, project_id: &str, session_id: &str, temp_token: &str) -> Result<Value> {
        let mut data = HashMap::new();
        data.insert("project_id".to_string(), json!(project_id));
        data.insert("session_id".to_string(), json!(session_id));
        data.insert("temp_token".to_string(), json!(temp_token));

        let path = self.config.api.endpoints.get("verification_code").unwrap();
        let headers = self.generate_signed_headers("POST", path, Some(&data))?;
        let url = format!("{}{}", self.config.api.base_url, path);

        let mut request = self.client.post(&url);

        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        request = request.json(&data);

        let response = request.send().await
            .map_err(|e| VerificationError::NetworkError(e))?;

        if response.status().is_success() {
            let result: Value = response.json().await
                .map_err(|e| VerificationError::NetworkError(e))?;
            
            if let Some(encrypted_data) = result.get("data").and_then(|d| d.as_str()) {
                // 解密验证码
                let decrypted = CryptoUtils::aes_decrypt(encrypted_data, temp_token)?;
                
                if let Some(code) = decrypted.get("code").and_then(|c| c.as_str()) {
                    log::info!("✓ 获取验证码成功: {}", code);
                    if let Some(expires_at) = decrypted.get("expires_at") {
                        log::info!("  过期时间: {}", expires_at);
                    }
                    Ok(decrypted)
                } else {
                    Err(VerificationError::decryption("解密验证码失败"))
                }
            } else {
                let error_msg = result.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("未知错误");
                Err(VerificationError::code(format!("获取验证码失败: {}", error_msg)))
            }
        } else {
            Err(VerificationError::code(format!("获取验证码失败，状态码: {}", response.status())))
        }
    }

    /// 获取公告信息
    pub async fn get_notice(&self) -> Result<Value> {
        let url = format!("{}{}",
            self.config.api.base_url,
            self.config.api.endpoints.get("notice").unwrap()
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| VerificationError::NetworkError(e))?;

        if response.status().is_success() {
            let data: Value = response.json().await
                .map_err(|e| VerificationError::NetworkError(e))?;
            Ok(data)
        } else {
            Err(VerificationError::api(format!("获取公告失败，状态码: {}", response.status())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};
    use tokio_test;

    fn create_test_config() -> Config {
        let mut config = Config::default();
        config.api.base_url = mockito::server_url();
        config
    }

    #[tokio::test]
    async fn test_fetch_signature_key_success() {
        let _m = mock("GET", "/api/public/signature-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "test_key", "version": 2}"#)
            .create();

        let config = create_test_config();
        let mut client = ApiClient::new(config).unwrap();

        let result = client.fetch_signature_key().await;
        assert!(result.is_ok());
        assert_eq!(client.key_version, 2);
        assert!(client.signature_key.is_some());
    }

    #[tokio::test]
    async fn test_fetch_signature_key_fallback() {
        let _m = mock("GET", "/api/public/signature-key")
            .with_status(500)
            .create();

        let config = create_test_config();
        let mut client = ApiClient::new(config.clone()).unwrap();

        let result = client.fetch_signature_key().await;
        assert!(result.is_ok());
        assert_eq!(client.key_version, 1);
        assert_eq!(client.signature_key.as_ref().unwrap(), &config.api.default_signature_key);
    }

    #[tokio::test]
    async fn test_create_ad_session_success() {
        let _m1 = mock("GET", "/api/public/signature-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "test_key", "version": 1}"#)
            .create();

        let _m2 = mock("POST", "/api/public/ad-session")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"session_id": "test_session_id", "created_at": "2025-01-01T00:00:00Z"}"#)
            .create();

        let config = create_test_config();
        let mut client = ApiClient::new(config).unwrap();

        let result = client.create_ad_session("YCursor").await;
        assert!(result.is_ok());

        let session_data = result.unwrap();
        assert_eq!(session_data["session_id"], "test_session_id");
    }

    #[tokio::test]
    async fn test_create_ad_session_rate_limit() {
        let _m1 = mock("GET", "/api/public/signature-key")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key": "test_key", "version": 1}"#)
            .create();

        let _m2 = mock("POST", "/api/public/ad-session")
            .with_status(429)
            .create();

        let config = create_test_config();
        let mut client = ApiClient::new(config).unwrap();

        let result = client.create_ad_session("YCursor").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VerificationError::RateLimitError { .. }));
    }

    #[tokio::test]
    async fn test_verify_ad_completion_success() {
        let _m = mock("POST", "/api/public/ad-verify")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"verified": true, "temp_token": "test_token"}"#)
            .create();

        let config = create_test_config();
        let client = ApiClient::new(config).unwrap();

        // 手动设置签名生成器以避免网络请求
        let mut client = client;
        client.signature_generator = Some(SignatureGenerator::new("test_key".to_string()));

        let result = client.verify_ad_completion("test_session", 30000).await;
        assert!(result.is_ok());

        let verify_data = result.unwrap();
        assert_eq!(verify_data["verified"], true);
        assert_eq!(verify_data["temp_token"], "test_token");
    }

    #[tokio::test]
    async fn test_get_notice_success() {
        let _m = mock("GET", "/XiaoChengXu/notice.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"content": "test notice"}]"#)
            .create();

        let config = create_test_config();
        let client = ApiClient::new(config).unwrap();

        let result = client.get_notice().await;
        assert!(result.is_ok());

        let notice = result.unwrap();
        assert!(notice.is_array());
    }
}

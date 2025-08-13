use crate::error::{Result, VerificationError};
use aes::Aes256;
use aes::cipher::{KeyIvInit, BlockDecryptMut};
use base64::{engine::general_purpose, Engine as _};
use cbc::{cipher::block_padding::Pkcs7, Decryptor};
use hmac::{Hmac, Mac};
use rand::Rng;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;
type Aes256CbcDec = Decryptor<Aes256>;

/// 加密工具类
pub struct CryptoUtils;

impl CryptoUtils {
    /// 生成SHA256哈希
    pub fn sha256(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// 生成HMAC-SHA256签名
    pub fn hmac_sha256(key: &str, data: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(key.as_bytes())
            .map_err(|e| VerificationError::crypto(format!("HMAC密钥错误: {}", e)))?;
        
        mac.update(data.as_bytes());
        let result = mac.finalize();
        Ok(format!("{:x}", result.into_bytes()))
    }

    /// 生成随机字符串
    pub fn generate_random_string(length: usize) -> String {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// AES-256-CBC解密
    pub fn aes_decrypt(encrypted_data: &str, temp_token: &str) -> Result<Value> {
        // 检查数据格式
        if !encrypted_data.starts_with("YAN_TEMP_") || !encrypted_data.ends_with("_END") {
            return Err(VerificationError::decryption("无效的加密数据格式".to_string()));
        }

        // 提取加密内容
        let encrypted_content = &encrypted_data[9..encrypted_data.len() - 4];

        // Base64解码
        let key_bytes = general_purpose::STANDARD
            .decode(temp_token)
            .map_err(|e| VerificationError::decryption(format!("temp_token解码失败: {}", e)))?;

        let encrypted_bytes = general_purpose::STANDARD
            .decode(encrypted_content)
            .map_err(|e| VerificationError::decryption(format!("加密数据解码失败: {}", e)))?;

        if encrypted_bytes.len() < 16 {
            return Err(VerificationError::decryption("加密数据长度不足".to_string()));
        }

        // 分离IV和密文
        let (iv, ciphertext) = encrypted_bytes.split_at(16);

        // AES解密
        let cipher = Aes256CbcDec::new_from_slices(&key_bytes, iv)
            .map_err(|e| VerificationError::decryption(format!("AES初始化失败: {}", e)))?;

        let mut buffer = ciphertext.to_vec();
        let decrypted_data = cipher
            .decrypt_padded_mut::<Pkcs7>(&mut buffer)
            .map_err(|e| VerificationError::decryption(format!("AES解密失败: {}", e)))?;

        // 解析JSON
        let json_str = std::str::from_utf8(decrypted_data)
            .map_err(|e| VerificationError::decryption(format!("UTF-8解码失败: {}", e)))?;

        let result: Value = serde_json::from_str(json_str)
            .map_err(|e| VerificationError::decryption(format!("JSON解析失败: {}", e)))?;

        Ok(result)
    }
}

/// API签名生成器
pub struct SignatureGenerator {
    signature_key: String,
}

impl SignatureGenerator {
    pub fn new(signature_key: String) -> Self {
        Self { signature_key }
    }

    /// 生成API签名
    pub fn generate_signature(
        &self,
        method: &str,
        path: &str,
        data: Option<&HashMap<String, Value>>,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        let body_str = if let Some(data) = data {
            // 按键排序并生成JSON字符串，确保与Python版本一致
            let mut sorted_keys: Vec<_> = data.keys().collect();
            sorted_keys.sort();

            let mut ordered_map = serde_json::Map::new();
            for key in sorted_keys {
                if let Some(value) = data.get(key) {
                    ordered_map.insert(key.clone(), value.clone());
                }
            }

            // 使用紧凑格式，确保与Python的separators=(',', ':')一致
            serde_json::to_string(&ordered_map)
                .map_err(|e| VerificationError::signature(format!("JSON序列化失败: {}", e)))?
        } else {
            String::new()
        };

        // 构建签名字符串
        let sign_string = format!(
            "{}\n{}\n{}\n{}\n{}",
            method.to_uppercase(),
            path,
            body_str,
            timestamp,
            nonce
        );

        // 生成HMAC-SHA256签名
        CryptoUtils::hmac_sha256(&self.signature_key, &sign_string)
            .map_err(|e| VerificationError::signature(format!("签名生成失败: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sha256() {
        let input = "hello world";
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert_eq!(CryptoUtils::sha256(input), expected);
    }

    #[test]
    fn test_hmac_sha256() {
        let key = "test_key";
        let data = "test_data";
        let result = CryptoUtils::hmac_sha256(key, data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 64); // SHA256 hex string length
    }

    #[test]
    fn test_generate_random_string() {
        let length = 16;
        let result = CryptoUtils::generate_random_string(length);
        assert_eq!(result.len(), length);
        assert!(result.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_signature_generation() {
        let generator = SignatureGenerator::new("test_key".to_string());
        let mut data = HashMap::new();
        data.insert("project_id".to_string(), json!("YCursor"));
        data.insert("device_fingerprint".to_string(), json!("test_fingerprint"));

        let result = generator.generate_signature(
            "POST",
            "/api/test",
            Some(&data),
            "1234567890",
            "test_nonce",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 64);
    }

    #[test]
    fn test_aes_decrypt_invalid_format() {
        let result = CryptoUtils::aes_decrypt("invalid_format", "test_token");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VerificationError::DecryptionError { .. }));
    }
}

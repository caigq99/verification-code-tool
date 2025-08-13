use crate::config::DeviceConfig;
use crate::core::crypto::CryptoUtils;
use rand::Rng;
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

/// 设备指纹生成器
pub struct DeviceFingerprint {
    config: DeviceConfig,
}

impl DeviceFingerprint {
    pub fn new(config: DeviceConfig) -> Self {
        Self { config }
    }

    /// 生成设备指纹
    pub fn generate(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut rng = rand::thread_rng();
        let random_value: f64 = rng.gen();

        let device_info = json!({
            "model": self.config.model,
            "platform": self.config.platform,
            "version": self.config.version,
            "system": self.config.system,
            "language": self.config.language,
            "screenWidth": self.config.screen_width,
            "screenHeight": self.config.screen_height,
            "pixelRatio": self.config.pixel_ratio,
            "brand": self.config.brand,
            "timestamp": timestamp,
            "random": random_value,
            "appId": self.config.app_id,
            "envVersion": self.config.env_version
        });

        // 使用紧凑格式，与Python版本保持一致
        let device_str = serde_json::to_string(&device_info).unwrap()
            .replace(" ", "");
        CryptoUtils::sha256(&device_str)
    }

    /// 获取设备信息
    pub fn get_device_info(&self) -> Value {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut rng = rand::thread_rng();
        let random_value: f64 = rng.gen();

        json!({
            "model": self.config.model,
            "platform": self.config.platform,
            "version": self.config.version,
            "system": self.config.system,
            "language": self.config.language,
            "screenWidth": self.config.screen_width,
            "screenHeight": self.config.screen_height,
            "pixelRatio": self.config.pixel_ratio,
            "brand": self.config.brand,
            "timestamp": timestamp,
            "random": random_value,
            "appId": self.config.app_id,
            "envVersion": self.config.env_version
        })
    }

    /// 生成用户代理字符串
    pub fn generate_user_agent(&self) -> String {
        format!(
            "Mozilla/5.0 ({} {} like Mac OS X) AppleWebKit/605.1.15",
            self.config.model.replace(' ', ""),
            self.config.system.replace(' ', "_")
        )
    }

    /// 获取屏幕信息
    pub fn get_screen_info(&self) -> (u32, u32, u32) {
        (
            self.config.screen_width,
            self.config.screen_height,
            self.config.pixel_ratio,
        )
    }

    /// 获取系统信息
    pub fn get_system_info(&self) -> (&str, &str, &str) {
        (&self.config.platform, &self.config.system, &self.config.language)
    }

    /// 更新设备配置
    pub fn update_config(&mut self, config: DeviceConfig) {
        self.config = config;
    }

    /// 生成随机设备变体
    pub fn generate_variant(&self) -> DeviceFingerprint {
        let mut rng = rand::thread_rng();
        
        // 随机调整屏幕尺寸（模拟不同设备）
        let width_variants = [375, 390, 414, 428];
        let height_variants = [667, 844, 896, 926];
        let pixel_variants = [2, 3];

        let width = width_variants[rng.gen_range(0..width_variants.len())];
        let height = height_variants[rng.gen_range(0..height_variants.len())];
        let pixel_ratio = pixel_variants[rng.gen_range(0..pixel_variants.len())];

        let mut new_config = self.config.clone();
        new_config.screen_width = width;
        new_config.screen_height = height;
        new_config.pixel_ratio = pixel_ratio;

        DeviceFingerprint::new(new_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn create_test_device() -> DeviceFingerprint {
        let config = Config::default();
        DeviceFingerprint::new(config.device)
    }

    #[test]
    fn test_generate_fingerprint() {
        let device = create_test_device();
        let fingerprint1 = device.generate();
        let fingerprint2 = device.generate();

        // 指纹应该是64字符的十六进制字符串
        assert_eq!(fingerprint1.len(), 64);
        assert_eq!(fingerprint2.len(), 64);
        
        // 由于包含时间戳和随机数，两次生成的指纹应该不同
        assert_ne!(fingerprint1, fingerprint2);
        
        // 验证是否为有效的十六进制字符串
        assert!(fingerprint1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(fingerprint2.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_get_device_info() {
        let device = create_test_device();
        let info = device.get_device_info();

        assert!(info.is_object());
        assert!(info["model"].is_string());
        assert!(info["platform"].is_string());
        assert!(info["timestamp"].is_number());
        assert!(info["random"].is_number());
    }

    #[test]
    fn test_generate_user_agent() {
        let device = create_test_device();
        let user_agent = device.generate_user_agent();

        assert!(user_agent.contains("Mozilla/5.0"));
        assert!(user_agent.contains("iPhone"));
        assert!(user_agent.contains("WebKit"));
    }

    #[test]
    fn test_get_screen_info() {
        let device = create_test_device();
        let (width, height, pixel_ratio) = device.get_screen_info();

        assert!(width > 0);
        assert!(height > 0);
        assert!(pixel_ratio > 0);
    }

    #[test]
    fn test_get_system_info() {
        let device = create_test_device();
        let (platform, system, language) = device.get_system_info();

        assert_eq!(platform, "ios");
        assert!(system.contains("iOS"));
        assert_eq!(language, "zh_CN");
    }

    #[test]
    fn test_generate_variant() {
        let device = create_test_device();
        let variant = device.generate_variant();

        // 变体应该有不同的屏幕配置
        let original_screen = device.get_screen_info();
        let variant_screen = variant.get_screen_info();

        // 至少有一个屏幕参数应该不同
        assert!(
            original_screen.0 != variant_screen.0
                || original_screen.1 != variant_screen.1
                || original_screen.2 != variant_screen.2
        );
    }

    #[test]
    fn test_update_config() {
        let mut device = create_test_device();
        let original_model = device.config.model.clone();

        let mut new_config = device.config.clone();
        new_config.model = "iPhone 15".to_string();

        device.update_config(new_config);

        assert_ne!(device.config.model, original_model);
        assert_eq!(device.config.model, "iPhone 15");
    }

    #[test]
    fn test_fingerprint_consistency() {
        let device = create_test_device();
        
        // 使用相同的时间戳和随机数种子应该生成相同的指纹
        // 这里我们测试设备信息的一致性
        let info1 = device.get_device_info();
        let info2 = device.get_device_info();

        // 除了时间戳和随机数，其他信息应该相同
        assert_eq!(info1["model"], info2["model"]);
        assert_eq!(info1["platform"], info2["platform"]);
        assert_eq!(info1["version"], info2["version"]);
        assert_eq!(info1["appId"], info2["appId"]);
    }
}

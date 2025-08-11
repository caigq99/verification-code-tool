#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
配置文件
"""

# API配置
API_CONFIG = {
    "base_url": "https://app.yan.vin",
    "endpoints": {
        "notice": "/XiaoChengXu/notice.json",
        "signature_key": "/api/public/signature-key",
        "ad_session": "/api/public/ad-session",
        "ad_verify": "/api/public/ad-verify",
        "verification_code": "/api/public/verification-code",
        "config": "/api/public/config"
    },
    "default_signature_key": "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210",
    "key_cache_duration": 300  # 5分钟
}

# 设备配置
DEVICE_CONFIG = {
    "model": "iPhone 14",
    "platform": "ios", 
    "version": "3.5.5",
    "system": "iOS 16.0",
    "language": "zh_CN",
    "screenWidth": 390,
    "screenHeight": 844,
    "pixelRatio": 3,
    "brand": "Apple",
    "appId": "wx421aabd7feefa0ed",
    "envVersion": "release"
}

# 请求配置
REQUEST_CONFIG = {
    "timeout": 30,
    "retry_times": 3,
    "retry_delay": 2,
    "user_agent": "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15",
    "referer": "https://servicewechat.com/wx421aabd7feefa0ed/devtools/page-frame.html"
}

# 支持的项目
SUPPORTED_PROJECTS = ["YCursor", "YAugment"]

# 默认配置
DEFAULT_CONFIG = {
    "watch_duration": 30000,  # 模拟观看时长（毫秒）
    "completion_proof": "miniprogram_ad_completed",
    "simulate_delay": 3  # 模拟观看延时（秒）
}

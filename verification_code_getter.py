#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
验证码获取工具
基于小程序API逆向分析实现
"""

import requests
import json
import time
import hashlib
import hmac
import random
import string
import base64
from Crypto.Cipher import AES
from Crypto.Util.Padding import unpad
import uuid


class VerificationCodeGetter:
    def __init__(self):
        self.base_url = "https://app.yan.vin"
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15',
            'Referer': 'https://servicewechat.com/wx421aabd7feefa0ed/devtools/page-frame.html'
        })
        self.signature_key = None
        self.key_version = 1
        self.key_last_update = 0
        
    def generate_device_fingerprint(self):
        """生成设备指纹"""
        device_info = {
            "model": "iPhone 14",
            "platform": "ios",
            "version": "3.5.5",
            "system": "iOS 16.0",
            "language": "zh_CN",
            "screenWidth": 390,
            "screenHeight": 844,
            "pixelRatio": 3,
            "brand": "Apple",
            "timestamp": int(time.time() * 1000),
            "random": random.random(),
            "appId": "wx421aabd7feefa0ed",
            "envVersion": "release"
        }
        
        device_str = json.dumps(device_info, separators=(',', ':'))
        return hashlib.sha256(device_str.encode()).hexdigest()
    
    def fetch_signature_key(self):
        """获取签名密钥"""
        try:
            current_time = int(time.time())
            if self.signature_key and (current_time - self.key_last_update) < 300:
                return True
                
            response = self.session.get(f"{self.base_url}/api/public/signature-key")
            if response.status_code == 200:
                data = response.json()
                if 'key' in data and 'version' in data:
                    self.signature_key = data['key']
                    self.key_version = data['version']
                    self.key_last_update = current_time
                    print(f"✓ 获取签名密钥成功，版本: {self.key_version}")
                    return True
            
            # 使用默认密钥
            self.signature_key = "YAN_API_SIGN_2025_ULTRA_SECRET_KEY_FOR_SIGNATURE_VERIFICATION_9f8e7d6c5b4a3210"
            self.key_version = 1
            self.key_last_update = current_time
            print("⚠ 使用默认签名密钥")
            return True
            
        except Exception as e:
            print(f"✗ 获取签名密钥失败: {e}")
            return False
    
    def generate_api_signature(self, method, path, data, timestamp, nonce):
        """生成API签名"""
        try:
            if not self.signature_key:
                return None
                
            body_str = ""
            if data:
                # 按键排序并生成JSON字符串
                sorted_data = dict(sorted(data.items()))
                body_str = json.dumps(sorted_data, separators=(',', ':')).replace(' ', '')
            
            # 构建签名字符串
            sign_string = f"{method.upper()}\n{path}\n{body_str}\n{timestamp}\n{nonce}"
            
            # 生成HMAC-SHA256签名
            signature = hmac.new(
                self.signature_key.encode(),
                sign_string.encode(),
                hashlib.sha256
            ).hexdigest()
            
            return signature
            
        except Exception as e:
            print(f"✗ 生成签名失败: {e}")
            return None
    
    def generate_signed_headers(self, method, path, data=None):
        """生成带签名的请求头"""
        timestamp = str(int(time.time()))
        nonce = ''.join(random.choices(string.ascii_lowercase + string.digits, k=16))
        
        signature = self.generate_api_signature(method, path, data, timestamp, nonce)
        
        headers = {
            "Content-Type": "application/json",
            "X-YAN-Signature": signature,
            "X-YAN-Timestamp": timestamp,
            "X-YAN-Nonce": nonce
        }
        
        return headers
    
    def create_ad_session(self, project_id):
        """创建广告会话"""
        try:
            device_fingerprint = self.generate_device_fingerprint()
            data = {
                "project_id": project_id,
                "device_fingerprint": device_fingerprint
            }
            
            headers = self.generate_signed_headers("POST", "/api/public/ad-session", data)
            
            response = self.session.post(
                f"{self.base_url}/api/public/ad-session",
                json=data,
                headers=headers
            )
            
            if response.status_code == 200:
                result = response.json()
                if 'session_id' in result:
                    print(f"✓ 创建会话成功: {result['session_id']}")
                    return result
                else:
                    print(f"✗ 创建会话失败: {result.get('error', '未知错误')}")
                    return None
            elif response.status_code == 429:
                print("✗ 请求频率过高，请稍后重试")
                return None
            else:
                print(f"✗ 创建会话失败，状态码: {response.status_code}")
                return None
                
        except Exception as e:
            print(f"✗ 创建会话异常: {e}")
            return None
    
    def verify_ad_completion(self, session_id, watch_duration=30000):
        """验证广告观看完成"""
        try:
            data = {
                "session_id": session_id,
                "watch_duration": watch_duration,
                "completion_proof": "miniprogram_ad_completed"
            }
            
            headers = self.generate_signed_headers("POST", "/api/public/ad-verify", data)
            
            response = self.session.post(
                f"{self.base_url}/api/public/ad-verify",
                json=data,
                headers=headers
            )
            
            if response.status_code == 200:
                result = response.json()
                if result.get('verified') and 'temp_token' in result:
                    print(f"✓ 广告验证成功")
                    return result
                else:
                    print(f"✗ 广告验证失败: {result.get('error', '未知错误')}")
                    return None
            elif response.status_code == 429:
                print("✗ 请求频率过高，请稍后重试")
                return None
            else:
                print(f"✗ 广告验证失败，状态码: {response.status_code}")
                return None
                
        except Exception as e:
            print(f"✗ 广告验证异常: {e}")
            return None
    
    def decrypt_verification_data(self, encrypted_data, temp_token):
        """解密验证码数据"""
        try:
            # 检查数据格式
            if not encrypted_data.startswith("YAN_TEMP_") or not encrypted_data.endswith("_END"):
                raise ValueError("无效的加密数据格式")
            
            # 提取加密数据
            encrypted_content = encrypted_data[9:-4]  # 去除前缀和后缀
            
            # Base64解码
            key_bytes = base64.b64decode(temp_token)
            encrypted_bytes = base64.b64decode(encrypted_content)
            
            # 分离IV和密文
            iv = encrypted_bytes[:16]
            ciphertext = encrypted_bytes[16:]
            
            # AES解密
            cipher = AES.new(key_bytes, AES.MODE_CBC, iv)
            decrypted = cipher.decrypt(ciphertext)
            
            # 去除填充
            decrypted = unpad(decrypted, AES.block_size)
            
            # 解析JSON
            result = json.loads(decrypted.decode('utf-8'))
            return result
            
        except Exception as e:
            print(f"✗ 解密失败: {e}")
            return None
    
    def get_verification_code(self, project_id, session_id, temp_token):
        """获取验证码"""
        try:
            data = {
                "project_id": project_id,
                "session_id": session_id,
                "temp_token": temp_token
            }
            
            headers = self.generate_signed_headers("POST", "/api/public/verification-code", data)
            
            response = self.session.post(
                f"{self.base_url}/api/public/verification-code",
                json=data,
                headers=headers
            )
            
            if response.status_code == 200:
                result = response.json()
                if 'data' in result:
                    # 解密验证码
                    decrypted = self.decrypt_verification_data(result['data'], temp_token)
                    if decrypted and 'code' in decrypted:
                        print(f"✓ 获取验证码成功: {decrypted['code']}")
                        if 'expires_at' in decrypted:
                            print(f"  过期时间: {decrypted['expires_at']}")
                        return decrypted['code']
                    else:
                        print("✗ 解密验证码失败")
                        return None
                else:
                    print(f"✗ 获取验证码失败: {result.get('error', '未知错误')}")
                    return None
            else:
                print(f"✗ 获取验证码失败，状态码: {response.status_code}")
                return None
                
        except Exception as e:
            print(f"✗ 获取验证码异常: {e}")
            return None
    
    def get_code_for_project(self, project_id="YCursor"):
        """完整流程获取验证码"""
        print(f"开始获取 {project_id} 验证码...")
        print("=" * 50)
        
        # 1. 获取签名密钥
        if not self.fetch_signature_key():
            return None
        
        # 2. 创建广告会话
        session_data = self.create_ad_session(project_id)
        if not session_data:
            return None
        
        session_id = session_data['session_id']
        
        # 3. 模拟观看广告（等待几秒）
        print("⏳ 模拟观看广告...")
        time.sleep(3)
        
        # 4. 验证广告完成
        verify_data = self.verify_ad_completion(session_id)
        if not verify_data:
            return None
        
        temp_token = verify_data['temp_token']
        
        # 5. 获取验证码
        code = self.get_verification_code(project_id, session_id, temp_token)
        
        print("=" * 50)
        if code:
            print(f"🎉 最终验证码: {code}")
        else:
            print("❌ 获取验证码失败")
        
        return code


def main():
    """主函数"""
    print("验证码获取工具")
    print("支持的项目: YCursor, YAugment")
    print()
    
    # 选择项目
    project = input("请输入项目名称 (默认: YCursor): ").strip()
    if not project:
        project = "YCursor"
    
    if project not in ["YCursor", "YAugment"]:
        print("❌ 不支持的项目名称")
        return
    
    # 创建获取器并执行
    getter = VerificationCodeGetter()
    code = getter.get_code_for_project(project)
    
    if code:
        print(f"\n✅ 成功获取验证码: {code}")
    else:
        print("\n❌ 获取验证码失败，请检查网络或稍后重试")


if __name__ == "__main__":
    main()

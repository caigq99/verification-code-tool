# 使用示例

## 1. 快速获取单个验证码

### 命令行方式
```bash
# 获取 YCursor 验证码
python3 quick_get.py YCursor

# 获取 YAugment 验证码  
python3 quick_get.py YAugment
```

### 使用启动脚本
```bash
./run.sh
```
然后选择对应的选项。

## 2. 交互式获取

```bash
python3 verification_code_getter.py
```

输入项目名称即可获取验证码。

## 3. 批量获取

### 基本批量获取
```bash
# 获取两个项目各1次验证码
python3 batch_get.py

# 获取指定项目的验证码
python3 batch_get.py --projects YCursor

# 获取多次验证码
python3 batch_get.py --count 3

# 设置获取间隔
python3 batch_get.py --interval 120
```

### 高级批量获取
```bash
# 获取YCursor项目5次，间隔90秒
python3 batch_get.py --projects YCursor --count 5 --interval 90

# 获取两个项目各3次，间隔60秒
python3 batch_get.py --projects YCursor YAugment --count 3 --interval 60
```

## 4. 编程方式使用

### 基本使用
```python
from verification_code_getter import VerificationCodeGetter

# 创建获取器
getter = VerificationCodeGetter()

# 获取验证码
code = getter.get_code_for_project("YCursor")
if code:
    print(f"获取到验证码: {code}")
else:
    print("获取失败")
```

### 批量获取
```python
from verification_code_getter import VerificationCodeGetter
import time

getter = VerificationCodeGetter()
projects = ["YCursor", "YAugment"]

for project in projects:
    print(f"正在获取 {project} 验证码...")
    code = getter.get_code_for_project(project)
    if code:
        print(f"✅ {project}: {code}")
    else:
        print(f"❌ {project}: 获取失败")
    
    # 等待一段时间避免频率限制
    time.sleep(60)
```

### 错误处理
```python
from verification_code_getter import VerificationCodeGetter

def safe_get_code(project, max_retries=3):
    getter = VerificationCodeGetter()
    
    for attempt in range(max_retries):
        try:
            code = getter.get_code_for_project(project)
            if code:
                return code
            else:
                print(f"第 {attempt + 1} 次尝试失败")
        except Exception as e:
            print(f"第 {attempt + 1} 次尝试出错: {e}")
        
        if attempt < max_retries - 1:
            print("等待30秒后重试...")
            time.sleep(30)
    
    return None

# 使用
code = safe_get_code("YCursor")
if code:
    print(f"最终获取到验证码: {code}")
else:
    print("所有尝试都失败了")
```

## 5. 实际运行示例

### 成功获取示例
```
$ python3 quick_get.py YCursor
正在获取 YCursor 验证码...
开始获取 YCursor 验证码...
==================================================
✓ 获取签名密钥成功，版本: 4
✓ 创建会话成功: 47b64e5e-86ed-401d-b923-d1c8f0eb2b2c
⏳ 模拟观看广告...
✓ 广告验证成功
✓ 获取验证码成功: G85Y6M
  过期时间: 2025-08-09T23:49:31.971837
==================================================
🎉 最终验证码: G85Y6M

🎉 验证码: G85Y6M
```

### 批量获取示例
```
$ python3 batch_get.py --projects YCursor YAugment --count 2 --interval 60

批量获取验证码
项目: YCursor, YAugment
每个项目获取次数: 2
间隔时间: 60秒
============================================================

第 1 轮获取:

--- YCursor ---
开始获取 YCursor 验证码...
==================================================
✓ 获取签名密钥成功，版本: 4
✓ 创建会话成功: xxx
✓ 广告验证成功
✓ 获取验证码成功: ABC123
==================================================
🎉 最终验证码: ABC123
等待 30 秒...

--- YAugment ---
开始获取 YAugment 验证码...
==================================================
✓ 获取签名密钥成功，版本: 4
✓ 创建会话成功: yyy
✓ 广告验证成功
✓ 获取验证码成功: DEF456
==================================================
🎉 最终验证码: DEF456

等待 60 秒进行下一轮...

第 2 轮获取:
...

📁 结果已保存到: verification_codes_20250809_234500.json

============================================================
📊 统计信息:
总请求次数: 4
成功次数: 4
失败次数: 0
成功率: 100.0%

按项目统计:
  YCursor: 2/2 (100.0%)
  YAugment: 2/2 (100.0%)

✅ 成功获取的验证码:
  YCursor: ABC123 (2025-08-09 23:45:00)
  YAugment: DEF456 (2025-08-09 23:45:30)
  YCursor: GHI789 (2025-08-09 23:46:30)
  YAugment: JKL012 (2025-08-09 23:47:00)
```

## 6. 注意事项

1. **频率限制**: 不要过于频繁地调用API，建议间隔至少30-60秒
2. **网络环境**: 确保网络连接稳定，能够访问 `app.yan.vin`
3. **错误处理**: 程序已包含完整的错误处理，遇到问题会有详细提示
4. **结果保存**: 批量获取的结果会自动保存为JSON文件

## 7. 故障排除

### 常见错误及解决方案

1. **SSL警告**: 这是urllib3的兼容性警告，不影响功能
2. **频率限制**: 等待10-15分钟后重试
3. **网络连接失败**: 检查网络连接和防火墙设置
4. **解密失败**: 通常是临时问题，重试即可

### 调试模式

如果需要查看详细的调试信息，可以修改代码添加更多日志输出。

# 验证码获取工具 - Rust 版本

基于 Rust 和 egui 开发的现代化验证码获取工具，专注于简洁高效的验证码获取体验。

## 🚀 功能特点

- 🖥️ **现代化图形界面** - 基于 egui 构建，支持深色/浅色主题
- 📝 **详细步骤日志** - 5 个清晰的获取步骤，实时显示操作进度
- ⚡ **一键获取** - 支持 YCursor 和 YAugment 项目的快速获取
- � **实时进度显示** - 进度条从 0%到 100%实时更新
- � **自动复制** - 验证码获取成功后自动复制到剪贴板
- 🎯 **简洁界面** - 专注核心功能，去除复杂的批量操作
- 🔐 **完整加密** - 实现了完整的 HMAC-SHA256 签名和 AES-256-CBC 解密
- 🛡️ **安全机制** - 设备指纹生成、会话管理、频率限制保护

## 📋 系统要求

- Rust 1.70.0 或更高版本
- Windows 10/11, macOS 10.15+, 或 Linux (Ubuntu 18.04+)
- 网络连接以访问 API 服务

## 🔧 安装和构建

### 1. 克隆项目

```bash
git clone <repository-url>
cd rust-verification-tool
```

### 2. 安装依赖

使用 cargo 自动安装所有依赖：

```bash
cargo build
```

或者手动添加依赖（推荐，确保最新版本）：

```bash
# GUI框架
cargo add eframe@0.28
cargo add egui@0.28

# HTTP客户端
cargo add reqwest --features json,rustls-tls
cargo add tokio --features full

# 序列化
cargo add serde --features derive
cargo add serde_json

# 加密
cargo add sha2
cargo add hmac
cargo add aes
cargo add cbc
cargo add base64
cargo add rand

# 时间处理
cargo add chrono --features serde

# 日志
cargo add log
cargo add env_logger

# 错误处理
cargo add anyhow
cargo add thiserror

# 异步运行时
cargo add futures

# 剪贴板
cargo add arboard

# 开发依赖
cargo add --dev tokio-test
cargo add --dev mockito
```

### 3. 构建项目

```bash
# 调试构建
cargo build

# 发布构建
cargo build --release
```

### 4. 运行程序

```bash
# 调试模式
cargo run

# 发布模式
cargo run --release
```

## 🎮 使用方法

### 图形界面操作

1. **启动程序** - 运行 `cargo run` 启动图形界面
2. **选择验证码** - 点击对应的大按钮获取验证码
   - 🟦 **获取 YCursor 验证码** - 获取 YCursor 项目验证码
   - 🟪 **获取 YAugment 验证码** - 获取 YAugment 项目验证码
3. **查看进度** - 观察详细的 5 个步骤和实时进度条
4. **自动复制** - 验证码获取成功后自动复制到剪贴板
5. **查看日志** - 在右侧日志面板查看详细的执行过程
6. **清空结果** - 通过菜单栏清空日志和统计信息

### 获取步骤说明

每次获取验证码都会经历以下 5 个步骤：

1. **📋 步骤 1/5: 获取 API 签名密钥** (20%)
2. **🔗 步骤 2/5: 创建广告会话** (40%)
3. **📺 步骤 3/5: 模拟观看广告** (60%)
4. **🔍 步骤 4/5: 验证广告观看完成** (80%)
5. **🎯 步骤 5/5: 获取最终验证码** (100%)

### 界面功能说明

- **左侧控制面板**：

  - 验证码获取按钮（大按钮设计）
  - 进度指示器
  - 统计信息显示

- **右侧日志面板**：

  - 实时详细日志显示
  - 步骤进度追踪
  - 自动滚动控制
  - 日志清空功能

- **顶部菜单栏**：
  - 文件操作（清空结果）
  - 设置选项（主题切换）
  - 帮助信息

## ⚙️ 配置选项

程序支持以下配置选项：

- **请求超时时间**：10-120 秒
- **重试次数**：1-5 次
- **模拟观看延时**：1-10 秒
- **主题切换**：深色/浅色主题
- **自动复制**：验证码自动复制到剪贴板

## 🧪 测试

运行单元测试：

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test crypto
cargo test device
cargo test client

# 显示测试输出
cargo test -- --nocapture
```

## 📊 项目结构

```
src/
├── main.rs              # 程序入口
├── app.rs               # 主应用程序
├── config.rs            # 配置管理
├── error.rs             # 错误定义
├── core/                # 核心功能模块
│   ├── mod.rs
│   ├── crypto.rs        # 加密解密
│   ├── device.rs        # 设备指纹
│   ├── client.rs        # HTTP客户端
│   └── verification.rs  # 验证码获取
└── ui/                  # 用户界面模块
    ├── mod.rs
    ├── components.rs    # UI组件
    ├── logger.rs        # 日志显示
    └── theme.rs         # 主题配置
```

## 🔒 安全说明

- 本工具实现了完整的 API 签名验证机制
- 使用 HMAC-SHA256 进行请求签名
- 采用 AES-256-CBC 进行数据解密
- 生成真实的设备指纹以防止检测
- 支持动态签名密钥获取和缓存

## ⚠️ 注意事项

1. **使用限制**：本工具仅供学习和研究使用
2. **频率控制**：请勿过于频繁调用 API，建议间隔 30-60 秒
3. **合规使用**：请遵守相关服务的使用条款

## 🐛 故障排除

### 常见问题

1. **编译错误**

   - 确保 Rust 版本 >= 1.70.0
   - 运行 `cargo clean` 清理缓存后重新构建

2. **网络连接失败**

   - 检查网络连接和防火墙设置

3. **获取失败**

   - 检查日志中的详细错误信息
   - 可能是频率限制，等待后重试

4. **界面显示问题**
   - 尝试切换深色/浅色主题
   - 调整窗口大小

## 📈 性能特点

- **内存使用**：约 15-30MB
- **启动时间**：< 2 秒
- **响应时间**：UI 响应 < 16ms
- **获取速度**：单次获取约 10-30 秒
- **自动复制**：验证码获取后立即复制到剪贴板

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

本项目仅供学习研究使用，请勿用于商业用途。

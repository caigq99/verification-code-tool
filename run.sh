#!/bin/bash

# 验证码获取工具启动脚本

echo "验证码获取工具"
echo "=================="

# 检查Python环境
if ! command -v python3 &> /dev/null; then
    echo "❌ 未找到 Python3，请先安装 Python3"
    exit 1
fi

# 检查依赖
echo "检查依赖..."
if ! python3 -c "import requests, Crypto" &> /dev/null; then
    echo "⚠️ 缺少依赖，正在安装..."
    pip3 install -r requirements.txt
fi

echo "✅ 环境检查完成"
echo ""

# 显示菜单
echo "请选择操作:"
echo "1. 快速获取验证码 (YCursor)"
echo "2. 快速获取验证码 (YAugment)" 
echo "3. 交互式获取"
echo "4. 批量获取"
echo "5. 退出"
echo ""

read -p "请输入选项 (1-5): " choice

case $choice in
    1)
        echo "获取 YCursor 验证码..."
        python3 quick_get.py YCursor
        ;;
    2)
        echo "获取 YAugment 验证码..."
        python3 quick_get.py YAugment
        ;;
    3)
        echo "启动交互式获取..."
        python3 verification_code_getter.py
        ;;
    4)
        echo "启动批量获取..."
        python3 batch_get.py
        ;;
    5)
        echo "退出"
        exit 0
        ;;
    *)
        echo "❌ 无效选项"
        exit 1
        ;;
esac

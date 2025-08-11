#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
快速获取验证码脚本
"""

import sys
import os
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from verification_code_getter import VerificationCodeGetter


def quick_get_code(project="YCursor"):
    """快速获取验证码"""
    print(f"正在获取 {project} 验证码...")
    
    getter = VerificationCodeGetter()
    code = getter.get_code_for_project(project)
    
    if code:
        print(f"\n🎉 验证码: {code}")
        return code
    else:
        print("\n❌ 获取失败")
        return None


if __name__ == "__main__":
    # 支持命令行参数
    project = "YCursor"
    if len(sys.argv) > 1:
        project = sys.argv[1]
    
    if project not in ["YCursor", "YAugment"]:
        print("❌ 支持的项目: YCursor, YAugment")
        print("用法: python quick_get.py [YCursor|YAugment]")
        sys.exit(1)
    
    quick_get_code(project)

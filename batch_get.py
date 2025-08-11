#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
批量获取验证码脚本
"""

import sys
import os
import time
import json
from datetime import datetime

sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from verification_code_getter import VerificationCodeGetter


def batch_get_codes(projects=None, count=1, interval=60):
    """
    批量获取验证码
    
    Args:
        projects: 项目列表，默认为 ["YCursor", "YAugment"]
        count: 每个项目获取次数
        interval: 获取间隔（秒）
    """
    if projects is None:
        projects = ["YCursor", "YAugment"]
    
    results = []
    getter = VerificationCodeGetter()
    
    print(f"批量获取验证码")
    print(f"项目: {', '.join(projects)}")
    print(f"每个项目获取次数: {count}")
    print(f"间隔时间: {interval}秒")
    print("=" * 60)
    
    for i in range(count):
        print(f"\n第 {i+1} 轮获取:")
        round_results = {}
        
        for project in projects:
            print(f"\n--- {project} ---")
            code = getter.get_code_for_project(project)
            
            result = {
                "project": project,
                "code": code,
                "timestamp": datetime.now().isoformat(),
                "success": code is not None
            }
            
            round_results[project] = result
            results.append(result)
            
            # 项目间间隔
            if project != projects[-1]:
                print(f"等待 {interval//2} 秒...")
                time.sleep(interval // 2)
        
        # 轮次间间隔
        if i < count - 1:
            print(f"\n等待 {interval} 秒进行下一轮...")
            time.sleep(interval)
    
    # 保存结果
    save_results(results)
    
    # 显示统计
    show_statistics(results)
    
    return results


def save_results(results):
    """保存结果到文件"""
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    filename = f"verification_codes_{timestamp}.json"
    
    with open(filename, 'w', encoding='utf-8') as f:
        json.dump(results, f, ensure_ascii=False, indent=2)
    
    print(f"\n📁 结果已保存到: {filename}")


def show_statistics(results):
    """显示统计信息"""
    print("\n" + "=" * 60)
    print("📊 统计信息:")
    
    total = len(results)
    success = sum(1 for r in results if r['success'])
    
    print(f"总请求次数: {total}")
    print(f"成功次数: {success}")
    print(f"失败次数: {total - success}")
    print(f"成功率: {success/total*100:.1f}%")
    
    # 按项目统计
    projects = {}
    for result in results:
        project = result['project']
        if project not in projects:
            projects[project] = {'total': 0, 'success': 0}
        
        projects[project]['total'] += 1
        if result['success']:
            projects[project]['success'] += 1
    
    print("\n按项目统计:")
    for project, stats in projects.items():
        rate = stats['success'] / stats['total'] * 100
        print(f"  {project}: {stats['success']}/{stats['total']} ({rate:.1f}%)")
    
    # 显示成功的验证码
    print("\n✅ 成功获取的验证码:")
    for result in results:
        if result['success']:
            time_str = result['timestamp'][:19].replace('T', ' ')
            print(f"  {result['project']}: {result['code']} ({time_str})")


def main():
    """主函数"""
    import argparse
    
    parser = argparse.ArgumentParser(description='批量获取验证码')
    parser.add_argument('--projects', nargs='+', default=['YCursor', 'YAugment'],
                       help='项目列表 (默认: YCursor YAugment)')
    parser.add_argument('--count', type=int, default=1,
                       help='每个项目获取次数 (默认: 1)')
    parser.add_argument('--interval', type=int, default=60,
                       help='获取间隔秒数 (默认: 60)')
    
    args = parser.parse_args()
    
    # 验证项目名称
    valid_projects = ['YCursor', 'YAugment']
    for project in args.projects:
        if project not in valid_projects:
            print(f"❌ 无效的项目名称: {project}")
            print(f"支持的项目: {', '.join(valid_projects)}")
            return
    
    try:
        batch_get_codes(args.projects, args.count, args.interval)
    except KeyboardInterrupt:
        print("\n\n⚠️ 用户中断操作")
    except Exception as e:
        print(f"\n❌ 发生错误: {e}")


if __name__ == "__main__":
    main()

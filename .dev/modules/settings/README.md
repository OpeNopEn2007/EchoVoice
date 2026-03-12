# Settings 模块

> EchoVoice 设置面板模块

## 概述

Settings 模块提供 EchoVoice 的用户配置界面，允许用户自定义：
- 快捷键配置
- 提示音设置
- 模型选择
- 界面主题
- 系统选项

## 相关文档

- [设计文档](./design.md) - 完整的界面设计和实现方案
- [接口定义](./interface.md) - 与前端通信的 API

## 技术实现

- 使用 Tauri Webview 实现跨平台 UI
- 通过 Tauri 命令与 Rust 后端通信
- 配置文件使用 YAML 格式

## 文件结构

```
settings/
├── design.md      # 设计文档
├── interface.md  # 接口定义
└── README.md     # 本文件
```
# XiaoTian Gradio 前端界面

这是 XiaoTian 项目的 Gradio 网页界面实现。该界面提供了一个友好的方式来使用 XiaoTian 的核心功能。

## 功能特性

- 模型选择（支持 llama3.2）
- GitHub 仓库更新获取
- Markdown 格式的原始数据和生成内容显示
- 邮件通知功能
- 内容下载功能

## 环境要求

- Python 3.8+
- pip

## 安装步骤

1. 创建并激活虚拟环境（推荐）：

```bash
# 创建虚拟环境
python -m venv venv

# 激活虚拟环境
# Windows
venv\Scripts\activate
# macOS/Linux
source venv/bin/activate
```

2. 安装依赖：

```bash
pip install -r requirements.txt
```

## 运行应用

```bash
# 确保在 frontend-gradio 目录下
python run.py
```

应用将在 http://localhost:7860 启动。

## 开发指南

### 项目结构

```
frontend-gradio/
├── requirements.txt    # 项目依赖
├── README.md          # 项目说明
├── run.py             # 启动脚本
├── src/               # 源代码目录
│   ├── __init__.py
│   ├── app.py         # 主应用入口
│   ├── components/    # UI 组件
│   │   └── interface.py
│   └── utils/         # 工具函数
│       └── mock.py    # 模拟数据
└── tests/             # 测试目录
    └── test_interface.py
```

### 测试

运行测试：

```bash
# 确保在 frontend-gradio 目录下
python -m pytest
```

## 注意事项

- 当前版本使用模拟数据，实际的 Rust 后端集成将在后续版本中实现
- 所有配置项都可以通过环境变量进行覆盖
- 运行时请确保在项目根目录（frontend-gradio）下执行命令

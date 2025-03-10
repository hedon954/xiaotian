# XiaoTian 模型关系说明文档

## 模型概述

XiaoTian 项目中的 model 模块定义了系统的核心数据结构，包含以下几个主要模型：

- **Repository**: 代表 GitHub 仓库
- **Source**: 内容源的抽象接口（如 GitHub 等）
- **Subscription**: 用户对内容源的订阅
- **Update**: 从内容源获取的更新信息

## 模型关系图

```
┌────────────┐         ┌───────────────┐
│ Repository │         │ SourceFactory │
└────────────┘         └───────────────┘
      │                        │
      │ 演变为                  │ 创建
      ▼                        ▼
┌────────────┐  配置  ┌───────────────┐  获取更新 ┌────────┐
│SourceConfig├───────►│    Source     ├──────────►│ Update │
└────────────┘        └───────────────┘           └────────┘
      ▲                        ▲
      │ 包含                    │ 引用
      │                        │
┌────────────┐                 │
│Subscription├─────────────────┘
└────────────┘
```

## 核心模型详解

### Repository（仓库）

Repository 模型表示一个 GitHub 仓库，包含基本的仓库信息。

**属性**:

- **id**: UUID，系统内部的唯一标识符
- **owner**: 仓库所有者（用户名或组织名）
- **name**: 仓库名称
- **url**: 完整的仓库 URL

**方法**:

- **new(owner, name)**: 创建新的仓库实例
- **full_name()**: 获取"owner/name"格式的完整仓库名

### Source（内容源）

Source 是一个 trait，定义了内容源的行为接口。它表示可以被订阅并获取更新的信息源。

**方法**:

- **get_type()**: 获取源类型（如 GitHub）
- **get_id()**: 获取源标识符
- **get_name()**: 获取显示名称
- **get_description()**: 获取描述（如果有）
- **get_url()**: 获取 URL
- **fetch_updates()**: 获取自指定时间以来的更新
- **get_metadata()**: 获取源特定的元数据
- **is_duplicate()**: 检查两个更新是否重复

相关类型:

- **SourceType**: 源类型枚举（目前仅支持 GitHub）
- **SourceError**: 源操作错误类型
- **SourceMetadata**: 通用源元数据结构
- **SourceConfig**: 源配置信息，用于创建源实例
- **GitHubSourceConfig**: GitHub 特定的源配置
- **SourceConfigParser**: 解析源配置的工具 trait
- **SourceFactory**: 创建源实例的工厂 trait

### Subscription（订阅）

Subscription 表示用户对某个内容源的订阅。

**属性**:

- **id**: UUID，订阅的唯一标识符
- **source_type**: 源类型（如 GitHub）
- **source_id**: 源的唯一标识符
- **source_config**: 源配置信息
- **name**: 订阅的显示名称
- **tags**: 用于分类的标签
- **update_types**: 要跟踪的更新类型

**方法**:

- **new()**: 创建通用订阅
- **github_repo()**: 创建 GitHub 仓库订阅
- **simple_github()**: 使用默认设置创建简单的 GitHub 订阅

相关枚举:

- **UpdateFrequency**: 更新检查频率（每日、每周、手动）
- **UpdateType**: 更新类型（提交、PR、问题、发布等）

### Update（更新）

Update 表示从内容源获取的更新事件。

**属性**:

- **id**: UUID，更新的唯一标识符
- **source_type**: 源类型
- **source_id**: 源标识符
- **event_type**: 事件类型
- **title**: 事件标题
- **description**: 事件描述或内容
- **url**: 事件 URL
- **author**: 事件作者
- **event_date**: 事件发生时间
- **fetched_at**: 获取时间
- **additional_data**: 额外数据

**方法**:

- **new()**: 创建新的更新事件
- **with_data()**: 创建带有额外数据的更新事件

相关枚举:

- **UpdateEventType**: 更新事件类型（提交、PR、问题等）

## 模型间的关系

1. **Repository 与 Source 的关系**:

   - Repository 是一种具体的信息源
   - 在当前实现中，Repository 主要作为独立模型存在，通过 SourceConfig 转换为 Source 接口

2. **Source 与 Subscription 的关系**:

   - Subscription 包含 SourceConfig，用于配置和引用 Source
   - Subscription 通过 source_id 和 source_type 标识特定的 Source
   - 一个 Source 可以有多个 Subscription

3. **Source 与 Update 的关系**:

   - Source 通过 fetch_updates()方法生成 Update 对象
   - Update 包含 source_id 和 source_type，标识其来源
   - 一个 Source 可以生成多个 Update

4. **Subscription 与 Update 的关系**:

   - Subscription 指定要跟踪的 UpdateType
   - Update 的 event_type 属性对应于 Subscription 的 update_types
   - 系统会根据 Subscription 过滤和组织相关的 Update

5. **SourceFactory 与 Source 的关系**:
   - SourceFactory 负责根据 SourceConfig 创建 Source 实例
   - 实现了依赖注入模式，使系统更容易测试和扩展

## 扩展性设计

XiaoTian 的模型设计展现出良好的扩展性:

1. **内容源扩展**:

   - 通过添加新的 SourceType 枚举值
   - 实现对应的 Source trait 实现
   - 添加特定的源配置解析器

2. **更新类型扩展**:

   - 扩展 UpdateEventType 枚举
   - 在 Source 实现中处理新的事件类型

3. **订阅选项扩展**:
   - 通过扩展 UpdateType 和 UpdateFrequency 枚举
   - 增强 Subscription 模型的功能

这种设计使 XiaoTian 可以在未来轻松添加对新内容源（如 GitLab、Bitbucket 等）的支持，以及处理更多类型的更新事件。

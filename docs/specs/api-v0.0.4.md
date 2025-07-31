# Xiaotian v0.0.4 RESTful API 接口文档

**版本**: v0.0.4
**基础 URL**: `/api/v1`
**发布日期**: 2025 年 7 月 31 日
**内容类型**: `application/json`
**字符编码**: `UTF-8`

## 📋 概述

本文档定义了 Xiaotian UI v0.0.4 版本所需的所有后端 RESTful API 接口，涵盖订阅源管理、内容摘要、智能问答、定时任务、邮件通知等核心功能模块。

## 🎯 API 设计原则

- **RESTful 风格**: 遵循 REST 架构风格和 HTTP 语义
- **统一响应格式**: 所有 API 返回统一的 JSON 响应格式
- **错误处理**: 规范的 HTTP 状态码和错误信息
- **分页支持**: 列表类接口统一支持分页参数
- **数据验证**: 完整的请求参数验证和约束

## 📊 统一响应格式

所有接口响应均采用以下统一格式：

### 成功响应格式

```json
{
  "code": 0,
  "data": {},
  "message": "操作成功",
  "trace_id": "trace-20250729-103000-abc123",
  "request_id": "req-20250729-103000-def456"
}
```

### 错误响应格式

```json
{
  "code": 40001,
  "data": null,
  "message": "请求参数无效：feedUrl 必须是有效的URL格式",
  "trace_id": "trace-20250729-103000-abc123",
  "request_id": "req-20250729-103000-def456"
}
```

### 分页数据格式

```json
{
  "items": [],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "total": 100,
    "totalPages": 5
  }
}
```

**说明**: 以下接口文档中的**返回结构**仅展示 `data` 字段的内容结构。

**ID 格式说明**:

- **资源 ID** (如订阅源、摘要等): 使用自增整数
- **消息 ID** (聊天消息): 使用 UUID 格式 (`msg-{uuid}`)
- **会话 ID** (聊天会话): 使用自增整数
- **执行 ID** (任务执行): 使用时间戳格式 (`exec-{timestamp}`)

---

## 🗂 1. 订阅源管理 (Feeds Management)

### 1.1 获取订阅源列表

**GET** `/feeds`

**请求参数**

- `page` (可选): 页码，默认 1
- `pageSize` (可选): 每页大小，默认 20，最大 100
- `category` (可选): 分类筛选
- `status` (可选): 状态筛选 (`active`, `loading`, `error`)

**返回结构**

```json
{
  "items": [
    {
      "id": 1,
      "name": "Hacker News",
      "type": "rss",
      "description": "技术新闻和讨论社区，汇聚全球程序员的智慧和前沿科技趋势",
      "feedUrl": "https://hnrss.org/frontpage",
      "category": "科技",
      "status": "active",
      "icon": "🔥",
      "createdAt": "2025-07-01T10:00:00Z",
      "lastUpdated": "2025-07-29T08:30:00Z",
      "summaryCount": 25
    }
  ]
}
```

### 1.2 添加订阅源

**POST** `/feeds`

**请求参数**

```json
{
  "name": "新订阅源",
  "type": "订阅类型", // rss
  "feedUrl": "https://example.com/feed.xml",
  "description": "订阅源描述",
  "category": "分类名称",
  "icon": "🔖"
}
```

**参数约束**

- `name`: 必填，长度 1-100 字符
- `type`: 必填，订阅类型，rss
- `feedUrl`: 必填，有效的 RSS/Atom URL
- `description`: 可选，最大 500 字符
- `category`: 可选，长度 1-50 字符
- `icon`: 可选，单个 emoji 字符

**返回结构**

```json
{
  "id": 1,
  "name": "新订阅源",
  "type": "rss",
  "feedUrl": "https://example.com/feed.xml",
  "description": "订阅源描述",
  "category": "分类名称",
  "icon": "🔖",
  "status": "loading",
  "lastUpdated": null,
  "createdAt": "2025-07-29T10:30:00Z",
  "summaryCount": 0
}
```

### 1.3 更新订阅源

**PUT** `/feeds/{feedId}`

**请求参数**

```json
{
  "name": "更新后的名称",
  "type": "rss",
  "description": "更新后的描述",
  "category": "更新后的分类",
  "icon": "🔖"
}
```

**返回结构**

```json
{
  "id": 1,
  "type": "rss",
  "name": "更新后的名称",
  "description": "更新后的描述",
  "category": "更新后的分类",
  "icon": "🔖",
  "feedUrl": "https://example.com/feed.xml",
  "status": "active",
  "lastUpdated": "2025-07-29T10:30:00Z",
  "createdAt": "2025-07-01T10:00:00Z",
  "summaryCount": 25
}
```

### 1.4 删除订阅源

**DELETE** `/feeds/{feedId}`

**请求参数**

- `cascade` (可选): 是否同时删除相关摘要，默认 false

**返回结构**

```json
null
```

### 1.5 获取订阅源详情

**GET** `/feeds/{feedId}`

**返回结构**

```json
{
  "id": 1,
  "type": "rss",
  "name": "Hacker News",
  "description": "技术新闻和讨论社区",
  "feedUrl": "https://hnrss.org/frontpage",
  "category": "科技",
  "icon": "🔥",
  "status": "active",
  "lastUpdated": "2025-07-29T08:30:00Z",
  "createdAt": "2025-07-01T10:00:00Z",
  "summaryCount": 25,
  "stats": {
    "totalSummaries": 150,
    "last30DaySummaries": 25,
    "avgSummariesPerDay": 5.2,
    "lastSyncDuration": 45
  }
}
```

---

## 📝 2. 内容摘要管理 (Summaries Management)

### 2.1 获取摘要列表

**GET** `/summaries`

**请求参数**

- `feedId` (可选): 订阅源 ID 筛选
- `page` (可选): 页码，默认 1
- `pageSize` (可选): 每页大小，默认 20
- `startDate` (可选): 开始日期 (ISO 8601)
- `endDate` (可选): 结束日期 (ISO 8601)
- `tags` (可选): 标签筛选，多个用逗号分隔
- `search` (可选): 关键词搜索

**返回结构**

```json
{
  "items": [
    {
      "id": 1,
      "title": "大型语言模型在代码生成领域的最新进展",
      "content": "近期研究表明，结合了静态分析工具的 LLM 在代码生成任务上表现出了惊人的准确性...",
      "originalUrl": "https://news.ycombinator.com/item?id=123456",
      "publishedAt": "2025-07-08T09:00:00Z",
      "tags": ["AI", "代码生成", "静态分析", "LLM"],
      "feedId": 1,
      "feedName": "Hacker News",
      "noteCount": 2,
      "sourceMaterialCount": 3,
      "relatedSummaryCount": 2,
      "createdAt": "2025-07-08T10:30:00Z",
      "updatedAt": "2025-07-08T15:20:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "total": 150,
    "totalPages": 8
  }
}
```

### 2.2 获取摘要详情

**GET** `/summaries/{summaryId}`

**返回结构**

```json
{
  "id": 1,
  "title": "大型语言模型在代码生成领域的最新进展",
  "content": "近期研究表明，结合了静态分析工具的 LLM...",
  "fullContent": "# LLM 代码生成领域的技术突破\n\n## 🔬 研究背景...",
  "originalUrl": "https://news.ycombinator.com/item?id=123456",
  "publishedAt": "2025-07-08T09:00:00Z",
  "tags": ["AI", "代码生成", "静态分析", "LLM"],
  "feedId": 1,
  "feedName": "Hacker News",
  "notesList": [
    {
      "content": "这个技术可能会改变整个编程行业",
      "createdAt": "2025-01-15T10:30:00Z"
    }
  ],
  "sourceMaterials": [
    {
      "id": "source-1-1",
      "title": "LLMs and Static Analysis: A Perfect Match for Code Generation",
      "url": "https://arxiv.org/abs/2025.12345",
      "publishedAt": "2025-01-14T10:30:00Z",
      "author": "Dr. Sarah Chen",
      "source": "arXiv.org",
      "excerpt": "本研究探讨了大型语言模型与静态分析工具结合...",
      "wordCount": 8500,
      "readingTime": 12,
      "language": "en",
      "contentType": "article"
    }
  ],
  "relatedSummaries": [
    {
      "id": 3,
      "title": "Rust 1.75 版本发布：异步编程的重大改进",
      "relevanceScore": 0.75,
      "relationType": "content",
      "sharedTags": ["编程语言", "性能优化"],
      "publishedAt": "2025-07-06T09:00:00Z",
      "excerpt": "Rust 1.75版本在编程语言演进方面的重要突破..."
    }
  ],
  "createdAt": "2025-07-08T10:30:00Z",
  "updatedAt": "2025-07-08T15:20:00Z"
}
```

### 2.3 获取相关摘要

**GET** `/summaries/{summaryId}/related`

**请求参数**

- `limit` (可选): 返回数量限制，默认 5，最大 10
- `minScore` (可选): 最小相关度分数，默认 0.3

**返回结构**

```json
{
  "items": [
    {
      "id": 3,
      "title": "Rust 1.75 版本发布：异步编程的重大改进",
      "relevanceScore": 0.75,
      "relationType": "content",
      "sharedTags": ["编程语言", "性能优化"],
      "publishedAt": "2025-07-06T09:00:00Z",
      "excerpt": "Rust 1.75版本在编程语言演进方面的重要突破..."
    }
  ]
}
```

### 2.4 添加摘要笔记

**POST** `/summaries/{summaryId}/notes`

**请求参数**

```json
{
  "content": "这是我的笔记内容"
}
```

**参数约束**

- `content`: 必填，长度 1-2000 字符

**返回结构**

```json
{
  "content": "这是我的笔记内容",
  "createdAt": "2025-07-29T10:30:00Z",
  "updatedAt": "2025-07-29T10:30:00Z"
}
```

### 2.5 更新摘要笔记

**PUT** `/summaries/{summaryId}/notes/{noteId}`

**请求参数**

```json
{
  "content": "这是我的笔记内容"
}
```

**返回结构**

```json
{
  "content": "这是我的笔记内容",
  "createdAt": "2025-07-29T10:30:00Z",
  "updatedAt": "2025-07-29T10:30:00Z"
}
```

### 2.6 删除摘要笔记

**DELETE** `/summaries/{summaryId}/notes/{noteId}`

**返回结构**

```json
null
```

### 2.7 添加/移除摘要标签

**POST** `/summaries/{summaryId}/tags`

**请求参数**

```json
{
  "tag": "新标签"
}
```

**返回结构**

```json
{
  "tags": ["AI", "代码生成", "静态分析", "LLM", "新标签"]
}
```

**DELETE** `/summaries/{summaryId}/tags/{tag}`

**返回结构**

```json
{
  "tags": ["AI", "代码生成", "静态分析", "LLM"]
}
```

---

## 💬 3. 聊天/问答系统 (Chat/QA System)

### 3.1 获取聊天会话列表

**GET** `/chat/sessions`

**请求参数**

- `page` (可选): 页码，默认 1
- `pageSize` (可选): 每页大小，默认 20

**返回结构**

```json
{
  "items": [
    {
      "id": 1,
      "title": "Rust 性能更新",
      "createdAt": "2025-07-29T10:00:00Z",
      "updatedAt": "2025-07-29T10:30:00Z",
      "messageCount": 4,
      "lastMessage": {
        "type": "assistant",
        "content": "根据你的知识库，Rust 在最新版本中发布了重要的异步编程改进...",
        "timestamp": "2025-07-29T10:30:00Z"
      }
    }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "total": 5,
    "totalPages": 1
  }
}
```

### 3.2 创建聊天会话

**POST** `/chat/sessions`

**请求参数**

```json
{
  "title": "新的聊天会话",
  "initialMessage": "你好，我想了解最新的技术动态"
}
```

**参数约束**

- `title`: 可选，长度 1-100 字符，默认为"新对话"
- `initialMessage`: 可选，长度 1-2000 字符

**返回结构**

```json
{
  "id": 1,
  "title": "新的聊天会话",
  "createdAt": "2025-07-29T10:30:00Z",
  "updatedAt": "2025-07-29T10:30:00Z",
  "messages": [
    {
      "id": "msg-550e8400-e29b-41d4-a716-446655440002",
      "type": "user",
      "content": "你好，我想了解最新的技术动态",
      "timestamp": "2025-07-29T10:30:00Z"
    }
  ]
}
```

### 3.3 获取聊天会话详情

**GET** `/chat/sessions/{sessionId}`

**返回结构**

```json
{
  "id": 1,
  "title": "Rust 性能更新",
  "createdAt": "2025-07-29T10:00:00Z",
  "updatedAt": "2025-07-29T10:30:00Z",
  "messages": [
    {
      "id": "msg-550e8400-e29b-41d4-a716-446655440000",
      "type": "user",
      "content": "最近 Rust 有哪些值得关注的性能更新?",
      "timestamp": "2025-07-29T10:00:00Z"
    },
    {
      "id": "msg-550e8400-e29b-41d4-a716-446655440001",
      "type": "assistant",
      "content": "根据你的知识库，Rust 在最新版本中发布了重要的异步编程改进...",
      "sources": ["Rust 1.75 版本发布：异步编程的重大改进"],
      "timestamp": "2025-07-29T10:30:00Z"
    }
  ]
}
```

### 3.4 发送聊天消息

**POST** `/chat/sessions/{sessionId}/messages`

**请求参数**

```json
{
  "content": "请详细介绍一下这些改进的具体内容",
  "context": {
    "summaryId": "summary-3",
    "feedId": "rust-blog"
  }
}
```

**参数约束**

- `content`: 必填，长度 1-2000 字符
- `context`: 可选，提供相关上下文信息

**返回结构**

```json
{
  "userMessage": {
    "id": "msg-550e8400-e29b-41d4-a716-446655440003",
    "type": "user",
    "content": "请详细介绍一下这些改进的具体内容",
    "timestamp": "2025-07-29T10:35:00Z"
  },
  "assistantMessage": {
    "id": "msg-550e8400-e29b-41d4-a716-446655440004",
    "type": "assistant",
    "content": "这些改进主要包括以下几个方面：\n1. 异步函数的性能优化...",
    "sources": ["Rust 1.75 版本发布：异步编程的重大改进"],
    "timestamp": "2025-07-29T10:35:30Z"
  }
}
```

### 3.5 删除聊天会话

**DELETE** `/chat/sessions/{sessionId}`

**返回结构**

```json
null
```

### 3.6 更新聊天会话标题

**PUT** `/chat/sessions/{sessionId}`

**请求参数**

```json
{
  "title": "更新后的会话标题"
}
```

**返回结构**

```json
{
  "id": 1,
  "title": "更新后的会话标题",
  "updatedAt": "2025-07-29T10:40:00Z"
}
```

---

## 🔄 4. 同步管理 (Sync Management)

### 4.1 执行手动同步

**POST** `/sync/manual`

**请求参数**

```json
{
  "feedIds": ["hacker-news", "rust-blog"],
  "options": {
    "includeAI": true,
    "sendEmail": false,
    "maxItems": 50,
    "summaryLength": "medium"
  }
}
```

**参数约束**

- `feedIds`: 可选，指定订阅源 ID 列表，为空则同步所有
- `options.includeAI`: 可选，是否生成 AI 摘要，默认 true
- `options.sendEmail`: 可选，是否发送邮件，默认 false
- `options.maxItems`: 可选，每个源最大同步条目数，默认 50
- `options.summaryLength`: 可选，摘要长度 (`short`|`medium`|`long`)

**返回结构**

```json
{
  "syncId": "sync-20250729-103000",
  "status": "started",
  "startTime": "2025-07-29T10:30:00Z",
  "estimatedDuration": 120,
  "feedCount": 2
}
```

### 4.2 获取同步状态

**GET** `/sync/status`

**请求参数**

- `syncId` (可选): 指定同步任务 ID

**返回结构**

```json
{
  "isRunning": true,
  "currentSync": {
    "syncId": "sync-20250729-103000",
    "startTime": "2025-07-29T10:30:00Z",
    "progress": 65,
    "currentAction": "正在分析 Rust Blog 的新内容...",
    "feedsProcessed": 1,
    "feedsTotal": 2,
    "itemsProcessed": 23,
    "itemsTotal": 35
  },
  "lastSyncTime": "2025-07-29T08:15:00Z",
  "lastSyncDuration": 85,
  "errors": [
    {
      "feedId": "reddit",
      "feedName": "Reddit Programming",
      "error": "连接超时",
      "timestamp": "2025-07-29T08:16:30Z"
    }
  ]
}
```

### 4.3 获取同步历史

**GET** `/sync/history`

**请求参数**

- `page` (可选): 页码，默认 1
- `pageSize` (可选): 每页大小，默认 20
- `startDate` (可选): 开始日期
- `endDate` (可选): 结束日期

**返回结构**

```json
{
  "items": [
    {
      "syncId": "sync-20250729-081500",
      "startTime": "2025-07-29T08:15:00Z",
      "endTime": "2025-07-29T08:16:25Z",
      "duration": 85,
      "status": "completed",
      "feedsProcessed": 3,
      "itemsProcessed": 47,
      "summariesGenerated": 12,
      "emailSent": false,
      "errors": 1
    }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "total": 50,
    "totalPages": 3
  }
}
```

### 4.4 取消同步任务

**POST** `/sync/cancel`

**请求参数**

```json
{
  "syncId": "sync-20250729-103000"
}
```

**返回结构**

```json
{
  "syncId": "sync-20250729-103000",
  "status": "cancelled",
  "cancelledAt": "2025-07-29T10:35:00Z"
}
```

---

## ⏰ 5. 定时任务管理 (Scheduled Tasks)

### 5.1 获取定时任务列表

**GET** `/tasks/scheduled`

**返回结构**

```json
{
  "items": [
    {
      "id": "task-daily-tech",
      "name": "每日技术资讯推送",
      "enabled": true,
      "cronExpression": "0 9 * * *",
      "nextRun": "2025-07-30T09:00:00Z",
      "lastRun": "2025-07-29T09:00:00Z",
      "lastRunStatus": "success",
      "lastRunDuration": 120,
      "emailConfig": {
        "enabled": true,
        "recipientEmails": ["user@example.com"],
        "senderName": "小天AI助手"
      },
      "selectedFeeds": ["hacker-news", "rust-blog"],
      "aiSummaryEnabled": true,
      "summaryLength": "medium",
      "createdAt": "2025-07-01T10:00:00Z",
      "updatedAt": "2025-07-29T09:00:00Z"
    }
  ]
}
```

### 5.2 创建定时任务

**POST** `/tasks/scheduled`

**请求参数**

```json
{
  "name": "工作日技术摘要",
  "cronExpression": "0 9 * * 1-5",
  "emailConfig": {
    "enabled": true,
    "recipientEmails": ["user@example.com", "team@example.com"],
    "senderName": "技术资讯助手"
  },
  "selectedFeeds": ["hacker-news", "vue-blog"],
  "aiSummaryEnabled": true,
  "summaryLength": "long",
  "enabled": false
}
```

**参数约束**

- `name`: 必填，长度 1-100 字符
- `cronExpression`: 必填，有效的 5 字段 cron 表达式
- `emailConfig.recipientEmails`: 必填，至少一个有效邮箱
- `selectedFeeds`: 必填，至少选择一个订阅源
- `summaryLength`: 必填，`short`|`medium`|`long`

**返回结构**

```json
{
  "id": "task-new",
  "name": "工作日技术摘要",
  "enabled": false,
  "cronExpression": "0 9 * * 1-5",
  "nextRun": "2025-07-30T09:00:00Z",
  "lastRun": null,
  "emailConfig": {
    "enabled": true,
    "recipientEmails": ["user@example.com", "team@example.com"],
    "senderName": "技术资讯助手"
  },
  "selectedFeeds": ["hacker-news", "vue-blog"],
  "aiSummaryEnabled": true,
  "summaryLength": "long",
  "createdAt": "2025-07-29T10:30:00Z",
  "updatedAt": "2025-07-29T10:30:00Z"
}
```

### 5.3 更新定时任务

**PUT** `/tasks/scheduled/{taskId}`

**请求参数**

```json
{
  "name": "更新后的任务名称",
  "enabled": true,
  "cronExpression": "0 18 * * *",
  "emailConfig": {
    "enabled": true,
    "recipientEmails": ["updated@example.com"],
    "senderName": "更新的发送者"
  }
}
```

**返回结构**

```json
{
  "id": "task-daily-tech",
  "name": "更新后的任务名称",
  "enabled": true,
  "cronExpression": "0 18 * * *",
  "nextRun": "2025-07-29T18:00:00Z",
  "updatedAt": "2025-07-29T10:35:00Z"
}
```

### 5.4 删除定时任务

**DELETE** `/tasks/scheduled/{taskId}`

**返回结构**

```json
null
```

### 5.5 手动执行定时任务

**POST** `/tasks/scheduled/{taskId}/execute`

**返回结构**

```json
{
  "taskId": "task-daily-tech",
  "executionId": "exec-20250729-103500",
  "startTime": "2025-07-29T10:35:00Z",
  "status": "started"
}
```

### 5.6 获取任务执行历史

**GET** `/tasks/scheduled/{taskId}/executions`

**请求参数**

- `page` (可选): 页码，默认 1
- `pageSize` (可选): 每页大小，默认 20

**返回结构**

```json
{
  "items": [
    {
      "executionId": "exec-20250729-090000",
      "startTime": "2025-07-29T09:00:00Z",
      "endTime": "2025-07-29T09:02:15Z",
      "duration": 135,
      "status": "success",
      "feedsProcessed": 2,
      "summariesGenerated": 8,
      "emailSent": true,
      "logs": [
        "开始同步 Hacker News",
        "获取到 12 条新内容",
        "生成 AI 摘要完成",
        "发送邮件成功"
      ]
    }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "total": 30,
    "totalPages": 2
  }
}
```

## 📧 6. 邮件配置 (Email Configuration)

### 6.1 获取邮件配置

**GET** `/email/config`

**返回结构**

```json
{
  "enabled": true,
  "recipientEmails": ["user@example.com", "team@example.com"],
  "senderName": "小天AI助手",
  "template": {
    "subject": "每日技术资讯摘要 - {date}",
    "headerText": "以下是今日为您精选的技术资讯摘要：",
    "footerText": "感谢使用小天AI助手",
    "includeOriginalLinks": true,
    "groupByFeed": true
  },
  "smtpConfig": {
    "host": "smtp.example.com",
    "port": 587,
    "username": "xiaotian@example.com",
    "authConfigured": true
  }
}
```

### 6.2 更新邮件配置

**PUT** `/email/config`

**请求参数**

```json
{
  "enabled": true,
  "recipientEmails": ["newuser@example.com", "newteam@example.com"],
  "senderName": "更新的发送者名称",
  "template": {
    "subject": "技术资讯摘要 - {date}",
    "headerText": "今日技术要闻：",
    "footerText": "祝您工作愉快",
    "includeOriginalLinks": true,
    "groupByFeed": false
  }
}
```

**参数约束**

- `recipientEmails`: 必填，至少一个有效邮箱地址
- `senderName`: 可选，长度 1-100 字符
- `template.subject`: 可选，长度 1-200 字符

**返回结构**

```json
{
  "enabled": true,
  "recipientEmails": ["newuser@example.com", "newteam@example.com"],
  "senderName": "更新的发送者名称",
  "template": {
    "subject": "技术资讯摘要 - {date}",
    "headerText": "今日技术要闻：",
    "footerText": "祝您工作愉快",
    "includeOriginalLinks": true,
    "groupByFeed": false
  },
  "updatedAt": "2025-07-29T10:40:00Z"
}
```

### 6.3 测试邮件配置

**POST** `/email/test`

**请求参数**

```json
{
  "recipientEmails": ["test@example.com"],
  "testContent": "这是一封测试邮件"
}
```

**返回结构**

```json
{
  "messageId": "test-20250729-104000",
  "sentAt": "2025-07-29T10:40:00Z",
  "recipients": ["test@example.com"],
  "deliveryStatus": "sent"
}
```

### 6.4 更新 SMTP 配置

**PUT** `/email/smtp`

**请求参数**

```json
{
  "host": "smtp.newserver.com",
  "port": 465,
  "username": "newemail@example.com",
  "password": "newpassword123",
  "useTLS": true
}
```

**参数约束**

- `host`: 必填，有效的 SMTP 服务器地址
- `port`: 必填，端口号 (1-65535)
- `username`: 必填，SMTP 用户名
- `password`: 必填，SMTP 密码

**返回结构**

```json
{
  "host": "smtp.newserver.com",
  "port": 465,
  "username": "newemail@example.com",
  "authConfigured": true,
  "useTLS": true,
  "updatedAt": "2025-07-29T10:45:00Z"
}
```

## 🔧 7. 系统管理 (System Management)

### 7.1 获取系统统计

**GET** `/system/stats`

**返回结构**

```json
{
  "feeds": {
    "total": 4,
    "active": 3,
    "error": 1
  },
  "summaries": {
    "total": 850,
    "today": 15,
    "thisWeek": 98,
    "thisMonth": 342
  },
  "chatSessions": {
    "total": 25,
    "active": 3,
    "messagesTotal": 156
  },
  "scheduledTasks": {
    "total": 2,
    "enabled": 1,
    "lastRunSuccess": true
  },
  "sync": {
    "totalRuns": 120,
    "successRate": 0.95,
    "avgDuration": 78,
    "lastSync": "2025-07-29T09:00:00Z"
  }
}
```

### 7.2 系统健康检查

**GET** `/system/health`

**返回结构**

```json
{
  "status": "healthy",
  "timestamp": "2025-07-29T10:50:00Z",
  "version": "v0.0.4",
  "uptime": 86400,
  "checks": {
    "database": {
      "status": "healthy",
      "responseTime": 15
    },
    "redis": {
      "status": "healthy",
      "responseTime": 3
    },
    "email": {
      "status": "healthy",
      "lastTest": "2025-07-29T09:30:00Z"
    },
    "feeds": {
      "status": "healthy",
      "accessibleCount": 3,
      "totalCount": 4
    }
  }
}
```

### 7.3 获取错误日志

**GET** `/system/logs`

**请求参数**

- `level` (可选): 日志级别 (`error`|`warn`|`info`)，默认`error`
- `page` (可选): 页码，默认 1
- `pageSize` (可选): 每页大小，默认 50
- `startDate` (可选): 开始日期
- `endDate` (可选): 结束日期

**返回结构**

```json
{
  "items": [
    {
      "id": "log-20250729-104500",
      "level": "error",
      "message": "订阅源同步失败",
      "module": "sync",
      "feedId": "reddit",
      "error": "Connection timeout after 30s",
      "timestamp": "2025-07-29T10:45:00Z",
      "details": {
        "url": "https://www.reddit.com/r/programming/.rss",
        "retryCount": 3
      }
    }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 50,
    "total": 25,
    "totalPages": 1
  }
}
```

### 7.4 清理系统数据

**POST** `/system/cleanup`

**请求参数**

```json
{
  "cleanupType": "old_logs",
  "retentionDays": 30,
  "dryRun": false
}
```

**参数约束**

- `cleanupType`: 必填，清理类型 (`old_logs`|`old_summaries`|`chat_sessions`)
- `retentionDays`: 必填，保留天数 (1-365)
- `dryRun`: 可选，是否为预览模式，默认 false

**返回结构**

```json
{
  "cleanupType": "old_logs",
  "itemsDeleted": 250,
  "spaceFreed": "15.2MB",
  "executedAt": "2025-07-29T10:50:00Z"
}
```

## 📋 8. 通用错误码

| 错误码                    | 枚举值 | HTTP 状态码 | 描述             |
| ------------------------- | ------ | ----------- | ---------------- |
| `VALIDATION_ERROR`        | 1001   | 400         | 请求参数验证失败 |
| `AUTHENTICATION_REQUIRED` | 1002   | 401         | 需要身份认证     |
| `FORBIDDEN`               | 1003   | 403         | 权限不足         |
| `RESOURCE_NOT_FOUND`      | 1004   | 404         | 资源不存在       |
| `METHOD_NOT_ALLOWED`      | 1005   | 405         | 请求方法不允许   |
| `CONFLICT`                | 1006   | 409         | 资源冲突         |
| `RATE_LIMIT_EXCEEDED`     | 1007   | 429         | 请求频率超限     |
| `INTERNAL_SERVER_ERROR`   | 1008   | 500         | 内部服务器错误   |
| `SERVICE_UNAVAILABLE`     | 1009   | 503         | 服务暂不可用     |

### 特定业务错误码

| 错误码                        | 枚举值 | HTTP 状态码 | 描述                |
| ----------------------------- | ------ | ----------- | ------------------- |
| `FEED_URL_INVALID`            | 2001   | 400         | RSS 订阅地址无效    |
| `FEED_ALREADY_EXISTS`         | 2002   | 409         | 订阅源已存在        |
| `CRON_EXPRESSION_INVALID`     | 2003   | 400         | Cron 表达式格式错误 |
| `EMAIL_CONFIG_INVALID`        | 2004   | 400         | 邮件配置无效        |
| `SYNC_IN_PROGRESS`            | 2005   | 409         | 同步任务正在进行中  |
| `TASK_NOT_ENABLED`            | 2006   | 400         | 定时任务未启用      |
| `CHAT_SESSION_LIMIT_EXCEEDED` | 2007   | 400         | 聊天会话数量超限    |

---

**文档版本**: v0.0.4
**最后更新**: 2025 年 7 月 31 日
**维护团队**: Xiaotian Development Team

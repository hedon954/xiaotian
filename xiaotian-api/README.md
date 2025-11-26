# Xiaotian API

基于 Rust 和领域驱动设计（DDD）架构的 RSS 订阅和 AI 摘要服务后端。

## 🚀 特性

- **RESTful API**: 完整的 REST API 支持，与前端无缝集成
- **RSS 订阅管理**: 支持添加、更新、删除 RSS 订阅源
- **内容摘要**: 自动抓取和生成文章摘要
- **智能问答**: 基于知识库的聊天问答系统
- **定时任务**: 支持定时同步和邮件推送
- **邮件通知**: 可配置的邮件摘要推送
- **系统监控**: 完整的系统健康检查和统计信息

## 🏗️ 架构设计

本项目采用领域驱动设计（DDD）架构：

```
xiaotian-api/
├── src/
│   ├── main.rs              # 应用程序入口
│   ├── lib.rs               # 库入口
│   ├── config/              # 配置模块
│   ├── infrastructure/      # 基础设施层
│   │   ├── database/        # 数据库
│   │   ├── email/           # 邮件服务
│   │   ├── rss/             # RSS 抓取
│   │   └── ai/              # AI 摘要服务
│   ├── domain/              # 领域层
│   │   ├── entities/        # 实体
│   │   ├── value_objects/   # 值对象
│   │   ├── repositories/    # 仓储接口
│   │   └── services/        # 领域服务
│   ├── application/         # 应用层
│   │   ├── commands/        # 命令
│   │   ├── queries/         # 查询
│   │   └── services/        # 应用服务
│   ├── interfaces/          # 接口层
│   │   ├── handlers/        # 处理器
│   │   ├── middleware/      # 中间件
│   │   └── routes/          # 路由
│   └── shared/              # 共享模块
│       ├── error.rs         # 错误处理
│       ├── response.rs      # 响应格式
│       └── utils.rs         # 工具函数
└── migration/               # 数据库迁移
```

## 🛠️ 技术栈

- **Web框架**: Axum
- **数据库**: PostgreSQL + SeaORM
- **缓存**: Redis
- **异步运行时**: Tokio
- **序列化**: Serde
- **日志**: Tracing
- **验证**: Validator
- **邮件**: Lettre
- **HTTP客户端**: Reqwest
- **RSS解析**: feed-rs

## 📋 环境要求

- Rust 1.70+
- PostgreSQL 12+
- Redis 6+

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone <repository-url>
cd xiaotian-api
```

### 2. 配置环境变量

```bash
cp .env.example .env
# 编辑 .env 文件，配置数据库和其他服务
```

### 3. 安装依赖

```bash
cargo build
```

### 4. 运行数据库迁移

```bash
# 首先安装 sea-orm-cli
cargo install sea-orm-cli

# 运行迁移
sea-orm-cli migrate up
```

### 5. 启动服务

```bash
cargo run
```

服务将在 `http://localhost:8080` 启动。

## 📡 API 文档

### 健康检查

```http
GET /health
```

### API v1 端点

基础路径: `/api/v1`

#### 订阅源管理
- `GET /feeds` - 获取订阅源列表
- `POST /feeds` - 添加订阅源
- `GET /feeds/{id}` - 获取订阅源详情
- `PUT /feeds/{id}` - 更新订阅源
- `DELETE /feeds/{id}` - 删除订阅源

#### 内容摘要管理
- `GET /summaries` - 获取摘要列表
- `GET /summaries/{id}` - 获取摘要详情
- `GET /summaries/{id}/related` - 获取相关摘要
- `POST /summaries/{id}/notes` - 添加摘要笔记
- `PUT /summaries/{id}/notes/{noteId}` - 更新摘要笔记
- `DELETE /summaries/{id}/notes/{noteId}` - 删除摘要笔记

#### 聊天/问答系统
- `GET /chat/sessions` - 获取聊天会话列表
- `POST /chat/sessions` - 创建聊天会话
- `GET /chat/sessions/{id}` - 获取聊天会话详情
- `POST /chat/sessions/{id}/messages` - 发送聊天消息
- `PUT /chat/sessions/{id}` - 更新会话标题
- `DELETE /chat/sessions/{id}` - 删除聊天会话

#### 同步管理
- `POST /sync/manual` - 执行手动同步
- `GET /sync/status` - 获取同步状态
- `GET /sync/history` - 获取同步历史
- `POST /sync/cancel` - 取消同步任务

#### 定时任务管理
- `GET /tasks/scheduled` - 获取定时任务列表
- `POST /tasks/scheduled` - 创建定时任务
- `PUT /tasks/scheduled/{id}` - 更新定时任务
- `DELETE /tasks/scheduled/{id}` - 删除定时任务
- `POST /tasks/scheduled/{id}/execute` - 手动执行定时任务

#### 邮件配置
- `GET /email/config` - 获取邮件配置
- `PUT /email/config` - 更新邮件配置
- `POST /email/test` - 测试邮件配置
- `PUT /email/smtp` - 更新SMTP配置

#### 系统管理
- `GET /system/stats` - 获取系统统计
- `GET /system/health` - 系统健康检查
- `GET /system/logs` - 获取错误日志
- `POST /system/cleanup` - 清理系统数据

详细的 API 文档请参考 `../docs/specs/api-v0.0.5.md`

## 🧪 测试

运行单元测试：

```bash
cargo test
```

运行集成测试：

```bash
cargo test --test integration
```

## 📝 开发

### 代码规范

项目使用 Rust 标准的代码格式化工具：

```bash
cargo fmt
```

使用 Clippy 进行代码检查：

```bash
cargo clippy
```

### 添加新功能

1. 在 `domain/entities/` 中定义实体
2. 在 `domain/repositories/` 中定义仓储接口
3. 在 `infrastructure/` 中实现仓储
4. 在 `application/` 中实现应用服务
5. 在 `interfaces/handlers/` 中实现处理器
6. 在 `interfaces/routes/` 中添加路由

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 联系

如有问题，请联系开发团队。
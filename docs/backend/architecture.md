# Xiaotian 后端架构文档

## 📋 概述

Xiaotian 后端服务基于 Rust 语言和领域驱动设计（DDD）架构构建，提供高性能、类型安全的 RSS 订阅和 AI 摘要服务。

## 🏗️ 架构设计

### 领域驱动设计（DDD）分层架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Interfaces Layer                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐     │
│  │ HTTP Routes  │ │ Middleware  │ │   DTOs      │     │
│  │   & API     │ │ & Handlers  │ │Validation   │     │
│  └─────────────┘ └─────────────┘ └─────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                   Application Layer                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐     │
│  │  Commands   │ │   Queries   │ │   Services  │     │
│  │ & Use Cases │ │ & Read      │ │ & Orches-   │     │
│  │             │ │ Models      │ │  tration    │     │
│  └─────────────┘ └─────────────┘ └─────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Domain Layer                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐     │
│  │  Entities   │ │ Value Obj   │ │ Repositories│     │
│  │             │ │   ects      │ │Interfaces   │     │
│  │             │ │             │ │             │     │
│  │ Business    │ │ Domain      │ │ Domain      │     │
│  │ Logic       │ │ Concepts    │ │ Services    │     │
│  └─────────────┘ └─────────────┘ └─────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                Infrastructure Layer                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐     │
│  │  Database   │ │  External   │ │  Messaging  │     │
│  │ PostgreSQL  │ │   APIs      │ │ (Redis)     │     │
│  │  + ORM     │ │ (RSS, AI,   │ │             │     │
│  │             │ │ Email)      │ │             │     │
│  └─────────────┘ └─────────────┘ └─────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### 核心设计原则

1. **依赖倒置**: 高层模块不依赖低层模块，都依赖于抽象
2. **单一职责**: 每个模块只有一个变化的理由
3. **开闭原则**: 对扩展开放，对修改封闭
4. **接口隔离**: 不应该强迫客户端依赖它们不使用的接口
5. **领域驱动**: 业务逻辑集中在领域层

## 🗂️ 目录结构

```
xiaotian-api/
├── Cargo.toml                    # 项目配置和依赖
├── README.md                     # 项目说明文档
├── .env.example                  # 环境变量示例
├── src/
│   ├── main.rs                   # 应用程序入口点
│   ├── lib.rs                    # 库入口
│   ├── config/                   # 配置模块
│   │   └── mod.rs                # 应用配置管理
│   ├── shared/                   # 共享模块
│   │   ├── error.rs              # 错误处理
│   │   ├── response.rs           # 响应格式
│   │   ├── utils.rs              # 工具函数
│   │   └── mod.rs               # 共享模块导出
│   ├── domain/                   # 领域层
│   │   ├── entities/             # 实体
│   │   │   ├── mod.rs           # 实体模块导出
│   │   │   ├── feed.rs          # 订阅源实体
│   │   │   ├── summary.rs       # 内容摘要实体
│   │   │   ├── chat_session.rs  # 聊天会话实体
│   │   │   ├── scheduled_task.rs # 定时任务实体
│   │   │   ├── source_material.rs # 源材料实体
│   │   │   ├── summary_note.rs  # 摘要笔记实体
│   │   │   ├── related_summary.rs # 相关摘要实体
│   │   │   ├── scheduled_task_feed.rs # 任务-订阅源关联
│   │   │   ├── sync_history.rs  # 同步历史实体
│   │   │   ├── task_execution.rs # 任务执行实体
│   │   │   └── system_log.rs    # 系统日志实体
│   │   └── mod.rs               # 领域模块导出
│   ├── application/              # 应用层
│   │   └── mod.rs               # 应用服务占位符
│   ├── infrastructure/           # 基础设施层
│   │   ├── database/            # 数据库相关
│   │   ├── email/               # 邮件服务
│   │   ├── rss/                 # RSS 抓取服务
│   │   ├── ai/                  # AI 服务
│   │   └── mod.rs               # 基础设施模块导出
│   └── interfaces/              # 接口层
│       ├── handlers/            # HTTP 处理器
│       ├── middleware/          # 中间件
│       ├── routes/              # 路由定义
│       └── mod.rs               # 接口模块导出
└── migration/                   # 数据库迁移
    └── src/
        ├── main.rs               # 迁移模块导出
        ├── lib.rs               # 迁移库入口
        └── m*.rs                # 具体迁移文件
```

## 🧩 核心组件

### 1. 配置管理 (`src/config/`)

使用 `config` crate 实现灵活的配置管理：

- **环境变量支持**: 通过 `.env` 文件和环境变量配置
- **分层配置**: 不同环境的配置分离
- **类型安全**: 编译时配置验证
- **默认值**: 合理的默认配置

### 2. 共享模块 (`src/shared/`)

#### 错误处理 (`error.rs`)

统一的错误处理机制：

- **自定义错误类型**: `AppError` 枚举涵盖所有业务错误
- **HTTP 状态码映射**: 自动映射到合适的 HTTP 状态码
- **错误码标准化**: 统一的错误码体系
- **错误响应格式**: 标准化的 JSON 错误响应

#### 响应格式 (`response.rs`)

统一的 API 响应格式：

- **标准响应结构**: `ApiResponse<T>` 包含 code、data、message 等
- **分页支持**: `PaginationData<T>` 处理列表分页
- **请求追踪**: trace_id 和 request_id 用于调试

#### 工具函数 (`utils.rs`)

通用工具函数：

- **URL 验证**: RSS 链接格式验证
- **Cron 表达式**: 定时任务表达式验证
- **HTML 清理**: 安全的 HTML 内容处理
- **标签处理**: 标签解析和格式化
- **日期格式**: 中文日期格式化

### 3. 领域实体 (`src/domain/entities/`)

#### 订阅源实体 (`feed.rs`)

```rust
pub struct Model {
    pub id: i32,
    pub name: String,
    pub feed_type: String,           // "rss"
    pub feed_url: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: String,             // "active", "loading", "error"
    pub icon: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
    pub summary_count: i32,
}
```

**业务方法**:
- `is_active()`: 检查是否为活跃状态
- `increment_summary_count()`: 增加摘要计数
- `set_status()`: 更新状态并记录时间戳

#### 内容摘要实体 (`summary.rs`)

```rust
pub struct Model {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub full_content: String,
    pub original_url: String,
    pub published_at: String,        // 中文日期格式
    pub tags: Option<serde_json::Value>,
    pub feed_id: i32,
    pub feed_name: String,
    pub note_count: i32,
    pub source_material_count: i32,
    pub related_summary_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**业务方法**:
- `add_tag()` / `remove_tag()`: 标签管理
- `content_preview()`: 内容预览生成
- `increment_note_count()`: 笔记计数管理

#### 聊天会话实体 (`chat_session.rs`)

```rust
pub struct Model {
    pub id: i32,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct MessageModel {
    pub id: i32,
    pub message_id: String,         // UUID 格式
    pub session_id: i32,
    pub message_type: String,       // "user" or "assistant"
    pub content: String,
    pub sources: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}
```

#### 定时任务实体 (`scheduled_task.rs`)

```rust
pub struct Model {
    pub id: String,                // 任务 ID
    pub name: String,
    pub enabled: bool,
    pub cron_expression: String,
    pub cron_description: Option<String>,
    pub next_run: Option<DateTime<Utc>>,
    pub last_run: Option<DateTime<Utc>>,
    pub last_run_status: Option<String>,
    pub last_run_duration: Option<i32>,
    pub email_config: serde_json::Value,
    pub selected_feeds: serde_json::Value,
    pub ai_summary_enabled: bool,
    pub summary_length: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 🔧 技术栈详解

### Web 框架: Axum

- **高性能**: 基于 Tokio 和 Hyper 的异步框架
- **类型安全**: 编译时路由参数检查
- **中间件支持**: 灵活的中间件系统
- **提取器模式**: 简洁的请求处理

### 数据库: PostgreSQL + SeaORM

- **类型安全**: 编译时 SQL 查询检查
- **迁移管理**: 自动化的数据库迁移
- **关系映射**: 清晰的实体关系定义
- **连接池**: 高效的数据库连接管理

### 缓存: Redis

- **高速缓存**: 热点数据缓存
- **会话存储**: 聊天会话数据
- **任务队列**: 异步任务处理
- **分布式锁**: 防止重复操作

### 序列化: Serde

- **JSON 处理**: 高效的序列化/反序列化
- **类型安全**: 编译时类型检查
- **自定义序列化**: 灵活的格式控制

### 验证: Validator

- **输入验证**: 完整的数据验证规则
- **自定义验证器**: 业务特定验证逻辑
- **错误信息**: 用户友好的错误提示

### 日志: Tracing

- **结构化日志**: JSON 格式的日志输出
- **分布式追踪**: 请求链路追踪
- **性能监控**: 请求耗时统计

## 🚀 部署架构

### 单机部署

```
┌─────────────────────────────────────────┐
│           Load Balancer               │
│             (Nginx)                 │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│          Xiaotian API              │
│         (Rust + Axum)              │
│  ┌─────────────┬─────────────┐     │
│  │ PostgreSQL  │    Redis     │     │
│  │             │             │     │
│  └─────────────┴─────────────┘     │
└─────────────────────────────────────┘
```

### 容器化部署

```yaml
version: '3.8'
services:
  xiaotian-api:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/xiaotian
      - REDIS_URL=redis://redis:6379
    depends_on:
      - db
      - redis

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=xiaotian
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

## 📊 性能优化

### 数据库优化

1. **索引策略**:
   - 主键索引: 所有表的自增主键
   - 唯一索引: RSS URL 防止重复
   - 复合索引: 查询频繁的字段组合
   - 时间索引: 按时间范围查询优化

2. **查询优化**:
   - 分页查询: 使用 LIMIT/OFFSET
   - 连接查询: 合理使用 JOIN
   - 批量操作: 减少数据库往返

### 缓存策略

1. **多级缓存**:
   - 内存缓存: 应用内部缓存
   - Redis 缓存: 分布式缓存
   - CDN 缓存: 静态资源缓存

2. **缓存模式**:
   - Cache-Aside: 应用管理缓存
   - Write-Through: 同步更新缓存
   - Write-Behind: 异步更新缓存

### 并发处理

1. **异步处理**:
   - Tokio 运行时: 高效的异步 I/O
   - 连接池: 数据库和 Redis 连接复用
   - 批处理: 减少系统调用

2. **资源管理**:
   - 内存管理: 避免内存泄漏
   - 连接限制: 防止资源耗尽
   - 超时控制: 避免长时间阻塞

## 🔒 安全设计

### 输入验证

- **类型检查**: 编译时类型安全
- **长度限制**: 防止缓冲区溢出
- **格式验证**: URL、邮箱等格式检查
- **SQL 注入防护**: 参数化查询

### 认证授权

- **JWT Token**: 无状态认证
- **RBAC**: 基于角色的访问控制
- **API 密钥**: 服务间认证
- **请求限流**: 防止滥用

### 数据保护

- **敏感信息加密**: 密码、API 密钥等
- **传输加密**: HTTPS/TLS 加密
- **数据脱敏**: 日志中的敏感数据处理
- **备份策略**: 定期数据备份

## 📈 监控和运维

### 健康检查

- **服务健康**: `/health` 端点
- **数据库连接**: 连接池状态检查
- **外部服务**: 第三方服务可用性
- **资源使用**: CPU、内存、磁盘监控

### 日志管理

- **结构化日志**: JSON 格式输出
- **日志级别**: ERROR、WARN、INFO、DEBUG
- **日志聚合**: 集中式日志收集
- **日志分析**: 错误模式识别

### 指标监控

- **业务指标**: API 调用量、响应时间
- **技术指标**: 系统资源使用率
- **错误指标**: 错误率、异常统计
- **性能指标**: 吞吐量、延迟统计

## 🧪 测试策略

### 单元测试

- **实体测试**: 业务逻辑验证
- **工具函数**: 边界条件测试
- **错误处理**: 异常场景覆盖
- **配置验证**: 配置正确性检查

### 集成测试

- **API 测试**: 端到端功能验证
- **数据库测试**: 数据一致性检查
- **外部服务**: Mock 服务测试
- **性能测试**: 负载和压力测试

### 测试工具

- **Cargo Test**: 内置测试框架
- **Testcontainers**: 数据库测试环境
- **Mockito**: 外部服务 Mock
- **Criterion**: 性能基准测试

## 🔄 持续集成/持续部署

### CI/CD 流水线

1. **代码检查**:
   - 格式检查: `cargo fmt`
   - 静态分析: `cargo clippy`
   - 安全扫描: 依赖漏洞检查

2. **自动测试**:
   - 单元测试: `cargo test`
   - 集成测试: `cargo test --test integration`
   - 代码覆盖率: 目标覆盖率 >80%

3. **构建部署**:
   - 多阶段构建: 优化镜像大小
   - 安全扫描: 镜像安全检查
   - 自动部署: 蓝绿部署策略

## 📚 相关文档

- [API 接口文档](../specs/api-v0.0.5.md)
- [前端集成指南](../frontend-integration.md)
- [部署运维手册](deployment.md)
- [性能调优指南](performance-tuning.md)
- [故障排查手册](troubleshooting.md)
# XiaoTian 重构计划

## 一、现存问题

1. **ID 设计过度复杂**

   - 目前使用 UUID 作为实体 ID
   - ID 设计不够直观，不利于命令行交互
   - 存储效率不高

2. **模型关系耦合**

   - Subscription 直接依赖 Repository
   - 不利于未来支持其他代码托管平台（如 GitLab）

## 二、重构目标

1. **简化 ID 系统**

   - 使用自增整数作为实体 ID
   - 提供更友好的命令行交互体验
   - 提高存储效率

2. **解耦数据模型**

   - 引入 Source 概念，替代直接的 Repository 依赖
   - 为支持多平台做准备
   - 保持模型简单清晰

3. **简化整体架构**
   - 移除过度设计的架构层次
   - 保持代码简单直观
   - 专注于核心功能实现

## 三、具体改动计划

### 1. 数据模型重构

```rust
// 核心数据模型
pub struct Source {
    pub id: i32,
    pub source_type: String,    // "github", "gitlab" 等
    pub identifier: String,     // "owner/repo"
    pub created_at: DateTime<Utc>,
}

pub struct Subscription {
    pub id: i32,
    pub source_id: i32,
    pub last_checked: Option<DateTime<Utc>>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

pub struct Update {
    pub id: String,            // "{source_id}:{event_type}:{native_id}"
    pub source_id: i32,
    pub event_type: String,    // "commit", "pr", "issue" 等
    pub content: String,       // JSON 格式存储具体内容
    pub created_at: DateTime<Utc>,
}
```

### 2. 存储接口简化

```rust
pub trait Storage {
    // ID 生成
    fn next_source_id(&self) -> i32;
    fn next_subscription_id(&self) -> i32;

    // Source 操作
    fn save_source(&mut self, source: Source) -> Result<Source>;
    fn get_source(&self, id: i32) -> Option<Source>;
    fn list_sources(&self) -> Vec<Source>;

    // Subscription 操作
    fn save_subscription(&mut self, sub: Subscription) -> Result<Subscription>;
    fn get_subscription(&self, id: i32) -> Option<Subscription>;
    fn list_subscriptions(&self) -> Vec<Subscription>;

    // Update 操作
    fn save_update(&mut self, update: Update) -> Result<Update>;
    fn get_updates_for_source(&self, source_id: i32) -> Vec<Update>;
}
```

### 3. 命令处理简化

```rust
// 命令处理器
pub struct CommandHandler {
    storage: Box<dyn Storage>,
    github_client: GitHubClient,
}

impl CommandHandler {
    // 添加数据源
    pub async fn add_source(&mut self, source_type: String, identifier: String) -> Result<Source> {
        let source = Source {
            id: self.storage.next_source_id(),
            source_type,
            identifier,
            created_at: Utc::now(),
        };
        self.storage.save_source(source)
    }

    // 创建订阅
    pub async fn add_subscription(&mut self, source_id: i32) -> Result<Subscription> {
        // 验证源是否存在
        let source = self.storage.get_source(source_id)
            .ok_or(Error::SourceNotFound(source_id))?;

        let subscription = Subscription {
            id: self.storage.next_subscription_id(),
            source_id,
            last_checked: None,
            active: true,
            created_at: Utc::now(),
        };

        self.storage.save_subscription(subscription)
    }
}
```

### 4. CLI 交互优化

```bash
# 添加源
> add source github rust-lang/rust
Source added with ID: 1

# 列出可用的源
> list sources
1: github:rust-lang/rust
2: github:golang/go

# 创建订阅
> subscribe 1
Subscription created with ID: 1

# 查看订阅
> show subscription 1
Subscription #1:
  Source: github:rust-lang/rust (ID: 1)
  Last checked: Never
  Status: Active
```

## 四、实施步骤

1. **准备阶段**

   - 创建新的分支进行重构
   - 编写数据迁移计划
   - 更新测试用例

2. **第一阶段：ID 系统重构**

   - 实现新的 ID 生成机制
   - 更新存储层代码
   - 修改相关的命令处理逻辑

3. **第二阶段：模型解耦**

   - 引入 Source 模型
   - 更新 Subscription 模型
   - 调整相关的业务逻辑

4. **第三阶段：架构简化**

   - 移除不必要的抽象层
   - 简化命令处理流程
   - 优化错误处理机制

5. **最终阶段**
   - 全面测试
   - 文档更新
   - 性能测试和优化

## 五、注意事项

1. **向后兼容**

   - 考虑现有数据的迁移
   - 保持命令行接口的兼容性
   - 提供平滑升级路径

2. **错误处理**

   - 确保错误消息清晰明确
   - 提供有用的错误恢复建议
   - 保持日志记录的完整性

3. **性能考虑**
   - 监控 ID 生成的性能
   - 确保存储操作效率
   - 优化内存使用

## 六、预期收益

1. **更好的用户体验**

   - 更简单的命令行交互
   - 更直观的 ID 引用
   - 更清晰的错误提示

2. **更高的开发效率**

   - 代码更容易理解和维护
   - 更少的抽象层次
   - 更直接的功能实现

3. **更好的扩展性**
   - 为支持新平台做好准备
   - 保持架构简单但灵活
   - 便于添加新功能

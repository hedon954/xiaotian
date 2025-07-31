# Xiaotian UI API 服务层

本文档说明了 Xiaotian UI 的 API 服务层实现，包括统一的 URL 前缀配置、mock 开关和完整的接口对接。

## 📁 文件结构

```
src/services/
├── index.ts          # 统一服务入口
├── api.ts           # 真实 API 服务实现
├── mock.ts          # Mock 数据服务实现
└── README.md        # 本文档

src/config/
└── api.ts           # API 配置文件

src/stores/
├── api.ts           # 新的 API Store
└── app.ts           # 原有 App Store（已更新）

src/types/
└── api.ts           # API 相关类型定义
```

## 🔧 配置说明

### API 配置 (`src/config/api.ts`)

```typescript
export const API_CONFIG = {
  // 基础 URL - 可通过环境变量配置
  BASE_URL: import.meta.env.VITE_API_BASE_URL || "/api/v1",

  // Mock 开关 - 开发时默认开启
  USE_MOCK: import.meta.env.VITE_USE_MOCK === "true" || true,

  // 其他配置...
};
```

### 环境变量配置

创建 `.env` 文件来配置 API：

```bash
# 开发环境
VITE_API_BASE_URL=http://localhost:8080/api/v1
VITE_USE_MOCK=true

# 生产环境
VITE_API_BASE_URL=/api/v1
VITE_USE_MOCK=false
```

## 🚀 使用方法

### 1. 基本使用

```typescript
import { apiService } from "@/services";

// 获取订阅源列表
const feeds = await apiService.feeds.getFeeds();

// 创建新订阅源
const newFeed = await apiService.feeds.createFeed({
  name: "新订阅源",
  type: "rss",
  feedUrl: "https://example.com/feed.xml",
});
```

### 2. 使用 API Store

```typescript
import { useApiStore } from "@/stores/api";

export default {
  setup() {
    const apiStore = useApiStore();

    // 加载数据
    const loadData = async () => {
      await apiStore.loadFeeds();
      await apiStore.loadSummaries();
    };

    // 获取缓存数据
    const feeds = computed(() => apiStore.feedsCache);
    const isLoading = computed(() => apiStore.isLoading);
    const error = computed(() => apiStore.error);

    return {
      feeds,
      isLoading,
      error,
      loadData,
    };
  },
};
```

### 3. Mock 模式切换

```typescript
import { shouldUseMock } from "@/services";

// 检查当前是否使用 Mock
if (shouldUseMock()) {
  console.log("当前使用 Mock 数据模式");
} else {
  console.log("当前使用真实 API 模式");
}
```

## 📋 API 功能覆盖

基于 API v0.0.4 文档，已实现以下模块：

### ✅ 已实现的功能

- **订阅源管理**

  - 获取订阅源列表（支持分页、筛选）
  - 获取订阅源详情
  - 添加/更新/删除订阅源

- **内容摘要管理**

  - 获取摘要列表（支持分页、搜索、标签筛选）
  - 获取摘要详情和相关摘要
  - 笔记管理（添加/更新/删除）
  - 标签管理（添加/删除）

- **聊天/问答系统**

  - 会话管理（创建/获取/更新/删除）
  - 消息发送和接收
  - 多会话支持

- **同步管理**

  - 手动同步执行
  - 同步状态查询
  - 同步历史记录
  - 同步任务取消

- **定时任务管理**

  - 任务 CRUD 操作
  - 任务执行和历史查询
  - 支持 Cron 表达式

- **邮件配置**

  - 邮件配置管理
  - SMTP 配置
  - 邮件测试功能

- **系统管理**
  - 系统统计信息
  - 健康检查
  - 错误日志查询
  - 数据清理

## 🎭 Mock 数据

Mock 服务提供了完整的测试数据，包括：

- 4 个示例订阅源（包含各种状态）
- 3 篇示例摘要（包含完整内容、标签、笔记等）
- 1 个示例聊天会话
- 1 个示例定时任务
- 完整的系统状态和配置数据

Mock 数据会模拟真实的网络延迟和各种业务场景，便于开发和测试。

## 🔄 向后兼容性

新的 API 系统与现有的 `useAppStore` 保持兼容：

- 现有组件无需修改即可继续使用
- 原有的数据结构和方法签名保持不变
- 新功能通过 `useApiStore` 提供，可逐步迁移

## 🛠️ 错误处理

```typescript
import { NetworkError } from "@/services";

try {
  const data = await apiService.feeds.getFeeds();
} catch (error) {
  if (error instanceof NetworkError) {
    console.error("网络错误:", error.message);
    console.error("状态码:", error.status);
    console.error("错误码:", error.code);
  }
}
```

## 📝 开发指南

### 添加新的 API 接口

1. 在 `src/types/api.ts` 中定义类型
2. 在 `src/config/api.ts` 中添加端点
3. 在 `src/services/api.ts` 中实现真实 API
4. 在 `src/services/mock.ts` 中实现 Mock 数据
5. 在 `src/stores/api.ts` 中添加 Store 方法

### 测试 API 接口

1. 设置 `VITE_USE_MOCK=true` 使用 Mock 数据测试
2. 设置 `VITE_USE_MOCK=false` 连接真实后端测试
3. 通过浏览器开发者工具检查网络请求

## 🔧 故障排除

### 常见问题

1. **API 请求失败**

   - 检查 `VITE_API_BASE_URL` 配置
   - 确认后端服务是否启动
   - 查看浏览器网络面板的错误信息

2. **Mock 数据不生效**

   - 确认 `VITE_USE_MOCK=true`
   - 检查控制台是否显示 "使用 Mock 数据模式"

3. **类型错误**
   - 确保导入了正确的类型定义
   - 检查 API 响应格式是否与类型定义匹配

## 📚 更多资源

- [API v0.0.4 接口文档](../../../docs/specs/api-v0.0.4.md)
- [Vue 3 Composition API](https://vuejs.org/guide/extras/composition-api-faq.html)
- [Pinia Store](https://pinia.vuejs.org/)
- [TypeScript 指南](https://www.typescriptlang.org/docs/)

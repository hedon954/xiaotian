# GitHub API 参考文档 (XiaoTian 项目)

本文档提供了与 XiaoTian 项目相关的 GitHub API 概述及使用建议。

## 身份验证

GitHub API 需要身份验证才能访问大多数端点，并获得更高的速率限制：

- **个人访问令牌 (PAT)**: 最直接的身份验证方法
- **OAuth 应用**: 用于更复杂的授权流程
- **GitHub 应用**: 用于代表用户执行操作的集成

对于 XiaoTian v0.2.0，我们建议使用具有以下权限范围的个人访问令牌：

- `repo` (用于私有仓库)
- `public_repo` (仅用于公共仓库)

## 速率限制

GitHub API 强制执行以下速率限制：

- 未认证请求: 每小时 60 次请求
- 认证请求: 每小时 5,000 次请求

XiaoTian 应实现缓存和速率限制处理，以避免服务中断。

## REST API 端点

### 仓库信息

```
GET /repos/{owner}/{repo}
```

**用途**: 获取基本仓库信息。

**返回数据**:

- 仓库元数据 (名称、描述、URL)
- 星标数
- Fork 数
- 默认分支
- 创建和更新时间戳
- 许可证信息

**XiaoTian 用途**: 仓库订阅和元数据存储的核心 API。

### 提交记录

```
GET /repos/{owner}/{repo}/commits
```

**参数**:

- `since`: ISO 8601 时间戳 (例如, `2023-01-01T00:00:00Z`)
- `until`: ISO 8601 时间戳
- `per_page`: 每页结果数 (最大 100)
- `page`: 分页页码

**返回数据**:

- 提交列表，包含 SHA 值、提交消息、作者和时间戳
- 修改的文件
- 统计数据 (添加、删除的行数)

**XiaoTian 用途**: 跟踪仓库活动并生成更新通知。

### Issues

```
GET /repos/{owner}/{repo}/issues
```

**参数**:

- `state`: 按状态筛选 (open, closed, all)
- `since`: ISO 8601 时间戳
- `per_page`: 每页结果数 (最大 100)
- `page`: 分页页码
- `labels`: 以逗号分隔的标签名称列表

**返回数据**:

- Issue 标题和内容
- 状态 (开放/关闭)
- 标签和里程碑
- 创建者和受理人
- 评论数

**XiaoTian 用途**: 监控 issue 活动和跟踪项目进度。

### Pull Requests

```
GET /repos/{owner}/{repo}/pulls
```

**参数**:

- `state`: 按状态筛选 (open, closed, all)
- `sort`: 排序方式 (created, updated, popularity, long-running)
- `direction`: 排序方向 (asc, desc)
- `per_page`: 每页结果数 (最大 100)
- `page`: 分页页码

**返回数据**:

- PR 标题和内容
- 分支信息
- 审查状态
- 合并状态
- 关联的提交

**XiaoTian 用途**: 跟踪代码贡献和开发活动。

### 发布版本

```
GET /repos/{owner}/{repo}/releases
```

**参数**:

- `per_page`: 每页结果数 (最大 100)
- `page`: 分页页码

**返回数据**:

- 发布标题和描述
- 版本标签
- 发布日期
- 资源文件 (二进制文件等)

**XiaoTian 用途**: 监控新版本和更新日志信息。

## GraphQL API

GitHub 的 GraphQL API 允许在单个请求中获取多种类型的数据。

**端点**: `https://api.github.com/graphql`

**示例查询** (获取仓库数据、最近提交、issues 和 PRs):

```graphql
query {
  repository(owner: "owner", name: "repo") {
    name
    description
    stargazerCount
    forkCount

    # 最近提交
    defaultBranchRef {
      target {
        ... on Commit {
          history(first: 10) {
            nodes {
              messageHeadline
              committedDate
              author {
                name
              }
            }
          }
        }
      }
    }

    # 最近 issues
    issues(last: 5, states: OPEN) {
      nodes {
        title
        createdAt
        author {
          login
        }
      }
    }

    # 最近 PRs
    pullRequests(last: 5, states: OPEN) {
      nodes {
        title
        createdAt
        author {
          login
        }
      }
    }
  }
}
```

**XiaoTian 用途**: 用于未来版本的更高效数据检索。

## 使用 Octocrab 实现

XiaoTian 使用 `octocrab` Rust 库进行 GitHub API 交互。基本用法:

```rust
// 使用个人访问令牌初始化客户端
let octocrab = Octocrab::builder()
    .personal_token("YOUR_GITHUB_TOKEN".to_string())
    .build()?;

// 获取仓库信息
let repo = octocrab
    .repos("owner", "repo")
    .get()
    .await?;

// 获取最近提交
let commits = octocrab
    .repos("owner", "repo")
    .list_commits()
    .send()
    .await?;

// 获取最近 issues
let issues = octocrab
    .issues("owner", "repo")
    .list()
    .state(params::State::Open)
    .send()
    .await?;
```

## 最佳实践

1. **增量更新**: 使用 `since` 参数仅获取新数据。
2. **分页处理**: 始终处理 API 分页以获取完整数据。
3. **缓存**: 缓存响应以减少 API 调用次数。
4. **错误处理**: 实现健壮的错误处理，特别是针对速率限制。
5. **条件请求**: 使用 etags 和条件请求来节省速率限制。

## 未来扩展

在后续版本中，XiaoTian 可以扩展使用:

1. **Webhooks**: 用于仓库事件的实时通知。
2. **Repository Events API**: 用于更详细的活动跟踪。
3. **GraphQL API**: 用于优化数据检索。

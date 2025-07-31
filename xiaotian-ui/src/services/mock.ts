// Mock 数据服务
import { API_CONFIG } from '@/config/api'
import type {
  AddTagRequest,
  ApiChatSession,
  ApiEmailFullConfig,
  ApiFeed,
  ApiScheduledTask,
  ApiSmtpConfig,
  ApiSummary,
  ApiSyncHistory,
  ApiSystemHealth,
  ApiSystemLog,
  ApiSystemStats,
  ApiTaskExecution,
  CancelSyncRequest,
  CancelSyncResponse,
  ChatSessionListParams,
  CreateChatSessionRequest,
  CreateFeedRequest,
  CreateNoteRequest,
  CreateScheduledTaskRequest,
  ExecuteTaskResponse,
  FeedListParams,
  ManualSyncRequest,
  PaginationData,
  RelatedSummaryParams,
  SendMessageRequest,
  SendMessageResponse,
  SummaryListParams,
  SyncHistoryParams,
  SyncStartResponse,
  SyncStatusParams,
  SyncStatusResponse,
  SystemCleanupRequest,
  SystemCleanupResponse,
  SystemLogsParams,
  TaskExecutionParams,
  TestEmailRequest,
  TestEmailResponse,
  UpdateChatSessionRequest,
  UpdateEmailConfigRequest,
  UpdateFeedRequest,
  UpdateNoteRequest,
  UpdateScheduledTaskRequest,
  UpdateSmtpConfigRequest
} from '@/types/api'

// 辅助函数：获取订阅源的摘要数量
function getFeedSummaryCount(feedId: number): number {
  return mockSummaries.filter(s => s.feedId === feedId).length
}

// Mock 数据存储
let mockFeeds: ApiFeed[] = [
  {
    id: 1,
    name: 'Hacker News',
    type: 'rss',
    description: '技术新闻和讨论社区，汇聚全球程序员的智慧和前沿科技趋势',
    feedUrl: 'https://hnrss.org/frontpage',
    category: '科技',
    status: 'active',
    icon: '🔥',
    createdAt: new Date('2025-07-01T10:00:00Z'),
    lastUpdated: new Date('2025-07-29T08:30:00Z'),
    get count() { return getFeedSummaryCount(1) }
  },
  {
    id: 2,
    name: 'Rust Blog',
    type: 'rss',
    description: 'Rust 编程语言官方博客，最新版本发布、性能优化和社区动态',
    feedUrl: 'https://blog.rust-lang.org/feed.xml',
    category: '编程',
    status: 'active',
    icon: '🦀',
    createdAt: new Date('2025-06-15T10:00:00Z'),
    lastUpdated: new Date('2025-07-28T14:20:00Z'),
    get count() { return getFeedSummaryCount(2) }
  },
  {
    id: 3,
    name: 'Vue.js Blog',
    type: 'rss',
    description: 'Vue.js 官方博客，框架更新、最佳实践和前端生态发展资讯',
    feedUrl: 'https://blog.vuejs.org/feed.rss',
    category: '前端',
    status: 'active',
    icon: '💚',
    createdAt: new Date('2025-06-20T10:00:00Z'),
    lastUpdated: new Date('2025-07-29T06:45:00Z'),
    get count() { return getFeedSummaryCount(3) }
  },
  {
    id: 4,
    name: 'Reddit Programming',
    type: 'rss',
    description: '全球最大的在线社区和讨论平台，涵盖科技、编程、设计等话题',
    feedUrl: 'https://www.reddit.com/r/programming/.rss',
    category: '社区',
    status: 'error',
    icon: '🤖',
    createdAt: new Date('2025-06-25T10:00:00Z'),
    lastUpdated: new Date('2025-07-29T09:30:00Z'),
    get count() { return getFeedSummaryCount(4) }
  }
]

let mockSummaries: ApiSummary[] = [
  {
    id: 1,
    title: '大型语言模型在代码生成领域的最新进展',
    content: '近期研究表明，结合了静态分析工具的 LLM 在代码生成任务上表现出了惊人的准确性。模型不再是盲目生成代码，而是能够理解代码的上下文、依赖关系和潜在的空指针风险...',
    fullContent: `# LLM 代码生成领域的技术突破

## 🔬 研究背景

近期的多项研究表明，**结合静态分析工具的大型语言模型 (LLM)** 在代码生成任务上表现出了前所未有的准确性和可靠性。这一技术融合代表了人工智能辅助编程领域的重大进步。

## 🚀 核心技术创新

### 1. 智能上下文理解
- **语义分析增强**：模型能够深度理解代码的语义结构和业务逻辑
- **依赖关系映射**：自动识别并管理复杂的模块间依赖关系
- **类型推断优化**：基于上下文进行精确的类型推断和验证

### 2. 静态分析集成
\`\`\`python
def analyze_code_quality(code_snippet):
    # 集成静态分析工具的示例
    vulnerabilities = static_analyzer.scan(code_snippet)
    suggestions = llm_model.generate_improvements(code_snippet, vulnerabilities)
    return {
        'quality_score': calculate_score(code_snippet),
        'security_issues': vulnerabilities,
        'optimization_suggestions': suggestions
    }
\`\`\`

### 3. 预防性错误检测
> **关键优势**：在代码生成阶段就能发现潜在的空指针异常、内存泄漏和安全漏洞

## 📊 性能表现

| 指标 | 传统代码生成 | LLM + 静态分析 | 提升幅度 |
|------|-------------|---------------|----------|
| **准确率** | 65% | 89% | +24% |
| **Bug检出率** | 42% | 78% | +36% |
| **代码质量分数** | 6.2/10 | 8.7/10 | +40% |

*这一技术突破不仅提高了代码生成的质量，更为软件开发工作流程的自动化开辟了新的可能性。*`,
    originalUrl: 'https://news.ycombinator.com/item?id=123456',
    publishedAt: '2025年7月8日',
    tags: ['AI', '代码生成', '静态分析', 'LLM'],
    feedId: 1,
    feedName: 'Hacker News',
    noteCount: 2,
    sourceMaterialCount: 3,
    relatedSummaryCount: 1,
    createdAt: '2025-07-08T10:30:00Z',
    updatedAt: '2025-07-08T15:20:00Z',
    notesList: [
      {
        content: '这个技术可能会改变整个编程行业',
        createdAt: '2025-01-15 10:30'
      },
      {
        content: '需要关注对传统开发流程的影响',
        createdAt: '2025-01-15 11:00'
      }
    ],
    sourceMaterials: [
      {
        id: 'source-1-1',
        title: 'LLMs and Static Analysis: A Perfect Match for Code Generation',
        url: 'https://arxiv.org/abs/2025.12345',
        publishedAt: '2025-01-14T10:30:00Z',
        author: 'Dr. Sarah Chen',
        source: 'arXiv.org',
        excerpt: '本研究探讨了大型语言模型与静态分析工具结合在代码生成中的应用...',
        wordCount: 8500,
        readingTime: 12,
        language: 'en',
        contentType: 'article'
      },
      {
        id: 'source-1-2',
        title: 'GitHub Copilot X: Enhanced Code Generation with Static Analysis',
        url: 'https://github.blog/2025-01-14-copilot-x-static-analysis/',
        publishedAt: '2025-01-14T14:15:00Z',
        author: 'GitHub Team',
        source: 'GitHub Blog',
        excerpt: 'GitHub Copilot X 集成了先进的静态分析能力，显著提升代码质量...',
        wordCount: 3200,
        readingTime: 5,
        language: 'en',
        contentType: 'article'
      },
      {
        id: 'source-1-3',
        title: 'The Future of AI-Assisted Programming',
        url: 'https://news.ycombinator.com/item?id=123456',
        publishedAt: '2025-01-15T08:00:00Z',
        author: 'tech_enthusiast',
        source: 'Hacker News',
        excerpt: 'HN社区对AI辅助编程的深度讨论，涵盖了最新技术趋势...',
        wordCount: 1800,
        readingTime: 3,
        language: 'en',
        contentType: 'article'
      }
    ],
    relatedSummaries: [
      {
        id: 3,
        title: 'Rust 1.75 版本发布：异步编程的重大改进',
        relevanceScore: 0.75,
        relationType: 'content',
        sharedTags: ['编程语言', '性能优化'],
        publishedAt: '2025年7月6日',
        excerpt: 'Rust 1.75版本在编程语言演进方面的重要突破...'
      }
    ]
  },
  {
    id: 2,
    title: 'WebAssembly 在浏览器性能优化中的实际应用',
    content: 'WebAssembly (WASM) 作为新一代 Web 技术，在实际应用中展现了强大的性能潜力。本文通过多个真实案例，展示了 WASM 如何在图像处理、游戏引擎、加密算法等场景中显著提升性能...',
    fullContent: 'WebAssembly (WASM) 作为新一代 Web 技术的完整技术文档内容...',
    originalUrl: 'https://news.ycombinator.com/item?id=789012',
    publishedAt: '2025年7月7日',
    tags: ['WebAssembly', '性能优化', '浏览器技术'],
    feedId: 1,
    feedName: 'Hacker News',
    noteCount: 0,
    sourceMaterialCount: 1,
    relatedSummaryCount: 1,
    createdAt: '2025-07-07T10:30:00Z',
    updatedAt: '2025-07-07T15:20:00Z',
    notesList: [],
    sourceMaterials: [
      {
        id: 'source-2-1',
        title: 'WebAssembly Performance Benchmarks in Production',
        url: 'https://bytecodealliance.org/articles/wasm-performance-2025',
        publishedAt: '2025-01-12T16:45:00Z',
        author: 'Bytecode Alliance',
        source: 'Bytecode Alliance Blog',
        excerpt: '生产环境中WebAssembly性能基准测试的综合分析...',
        wordCount: 4800,
        readingTime: 8,
        language: 'en',
        contentType: 'article'
      }
    ],
    relatedSummaries: [
      {
        id: 1,
        title: '大型语言模型在代码生成领域的最新进展',
        relevanceScore: 0.65,
        relationType: 'content',
        sharedTags: ['性能优化'],
        publishedAt: '2025年7月8日',
        excerpt: 'LLM技术在代码生成方面的突破...'
      }
    ]
  },
  {
    id: 3,
    title: 'Rust 1.75 版本发布：异步编程的重大改进',
    content: 'Rust 1.75 版本带来了期待已久的异步编程改进，包括更好的错误处理、性能优化和开发体验提升。新版本的 async/await 语法更加直观，同时引入了更强大的并发原语...',
    fullContent: 'Rust 1.75 版本发布的完整技术文档内容...',
    originalUrl: 'https://blog.rust-lang.org/2025/01/15/Rust-1.75.0.html',
    publishedAt: '2025年7月6日',
    tags: ['Rust', '异步编程', '版本发布', '编程语言', '性能优化'],
    feedId: 2,
    feedName: 'Rust Blog',
    noteCount: 1,
    sourceMaterialCount: 2,
    relatedSummaryCount: 1,
    createdAt: '2025-07-06T10:30:00Z',
    updatedAt: '2025-07-06T15:20:00Z',
    notesList: [
      {
        content: '需要测试现有代码的兼容性',
        createdAt: '2025-01-15T14:20:00Z'
      }
    ],
    sourceMaterials: [
      {
        id: "source-3-1",
        title: 'Rust 1.75.0 Release Notes',
        url: 'https://blog.rust-lang.org/2025/01/15/Rust-1.75.0.html',
        publishedAt: '2025-01-15T16:00:00Z',
        author: 'Rust Team',
        source: 'Rust Blog',
        excerpt: 'Rust 1.75版本的详细发布说明和新特性介绍...',
        wordCount: 6200,
        readingTime: 10,
        language: 'en',
        contentType: 'article'
      },
      {
        id: "source-3-2",
        title: 'Async Programming Improvements in Rust 1.75',
        url: 'https://github.com/rust-lang/rfcs/pull/3016',
        publishedAt: '2025-01-10T09:30:00Z',
        author: 'Rust RFC Team',
        source: 'GitHub RFC',
        excerpt: '关于Rust异步编程改进的RFC详细讨论...',
        wordCount: 4100,
        readingTime: 7,
        language: 'en',
        contentType: 'article'
      }
    ],
    relatedSummaries: [
      {
        id: 1,
        title: '大型语言模型在代码生成领域的最新进展',
        relevanceScore: 0.75,
        relationType: 'content',
        sharedTags: ['编程语言', '性能优化'],
        publishedAt: '2025年7月8日',
        excerpt: 'LLM在编程语言领域的技术突破...'
      }
    ]
  },
  {
    id: 4,
    title: 'Vue 3.5 新特性：组合式 API 的性能优化',
    content: 'Vue 3.5 版本在组合式 API 方面带来了显著的性能提升，新增的响应式语法糖让代码更加简洁，改进的类型推导提供了更好的 TypeScript 支持...',
    fullContent: 'Vue 3.5 新特性的完整技术文档内容...',
    originalUrl: 'https://blog.vuejs.org/posts/vue-3-5.html',
    publishedAt: '2025年7月5日',
    tags: ['Vue.js', '前端', '性能优化', '组合式API'],
    feedId: 3,
    feedName: 'Vue.js Blog',
    noteCount: 0,
    sourceMaterialCount: 2,
    relatedSummaryCount: 0,
    createdAt: '2025-07-05T10:30:00Z',
    updatedAt: '2025-07-05T15:20:00Z',
    notesList: [],
    sourceMaterials: [
      {
        id: "source-7-1",
        title: 'Vue 3.5 Performance Improvements',
        url: 'https://blog.vuejs.org/posts/vue-3-5-performance.html',
        publishedAt: '2025-01-10T14:20:00Z',
        author: 'Vue Team',
        source: 'Vue.js Blog',
        excerpt: 'Vue 3.5版本的性能优化详细分析...',
        wordCount: 5300,
        readingTime: 9,
        language: 'en',
        contentType: 'article'
      },
      {
        id: "source-7-2",
        title: 'Composition API Best Practices',
        url: 'https://vuejs.org/guide/composition-api-best-practices.html',
        publishedAt: '2025-01-08T11:15:00Z',
        author: 'Vue Team',
        source: 'Vue.js Docs',
        excerpt: '组合式API的最佳实践指南...',
        wordCount: 3800,
        readingTime: 6,
        language: 'en',
        contentType: 'article'
      }
    ],
    relatedSummaries: []
  }
]

let mockChatSessions: ApiChatSession[] = [
  {
    id: 1,
    title: 'Rust 性能更新',
    createdAt: new Date('2025-07-29T10:00:00Z'),
    updatedAt: new Date('2025-07-29T10:30:00Z'),
    messageCount: 4,
    messages: [
      {
        id: '1',
        type: 'user',
        content: '最近 Rust 有哪些值得关注的性能更新?',
        timestamp: new Date().toISOString()
      },
      {
        id: '2',
        type: 'assistant',
        content: '根据你的知识库，Rust 在最新版本中发布了重要的异步编程改进...',
        sources: [
          {
            summaryId: 1,
            summaryTitle: 'Rust 1.75 版本发布：异步编程的重大改进'
          }
        ],
        timestamp: new Date().toISOString()
      }
    ]
  }
]

let mockScheduledTasks: ApiScheduledTask[] = [
  {
    id: 'task-daily-tech',
    name: '每日技术资讯推送',
    enabled: true,
    cronExpression: '0 9 * * *',
    cronDescription: '每天上午9点',
    nextRun: new Date('2025-07-30T09:00:00Z'),
    lastRun: new Date('2025-07-29T09:00:00Z'),
    lastRunStatus: 'success',
    lastRunDuration: 120,
    emailConfig: {
      enabled: true,
      recipientEmails: ['user@example.com'],
      senderName: '小天AI助手'
    },
    selectedFeeds: [1, 2], // 使用数字ID对应Hacker News和Rust Blog
    aiSummaryEnabled: true,
    summaryLength: 'medium',
    createdAt: '2025-07-01T10:00:00Z',
    updatedAt: '2025-07-29T09:00:00Z'
  }
]

// 模拟网络延迟
const delay = (ms: number = 300) => new Promise(resolve => setTimeout(resolve, ms))


// 生成分页响应
function createPaginatedResponse<T>(
  items: T[],
  page: number = 1,
  pageSize: number = 20
): PaginationData<T> {
  const start = (page - 1) * pageSize
  const end = start + pageSize
  const paginatedItems = items.slice(start, end)

  return {
    items: paginatedItems,
    pagination: {
      page,
      pageSize,
      total: items.length,
      totalPages: Math.ceil(items.length / pageSize)
    }
  }
}

// ============ Mock API 实现 ============

export const mockApi = {
  // 订阅源管理
  feeds: {
    async getFeeds(params?: FeedListParams): Promise<PaginationData<ApiFeed>> {
      await delay()
      let filteredFeeds = [...mockFeeds]

      if (params?.category) {
        filteredFeeds = filteredFeeds.filter(feed => feed.category === params.category)
      }
      if (params?.status) {
        filteredFeeds = filteredFeeds.filter(feed => feed.status === params.status)
      }

      return createPaginatedResponse(filteredFeeds, params?.page, params?.pageSize)
    },

    async getFeed(id: string | number): Promise<ApiFeed> {
      await delay()
      const numId = typeof id === 'string' ? parseInt(id, 10) : id
      const feed = mockFeeds.find(f => f.id === numId)
      if (!feed) {
        throw new Error('订阅源不存在')
      }
      return feed
    },

    async createFeed(data: CreateFeedRequest): Promise<ApiFeed> {
      await delay(1000)
      const newFeed: ApiFeed = {
        id: Date.now(),
        ...data,
        description: data.description || '新添加的订阅源', // 确保description有默认值
        category: data.category || '其他', // 确保category有默认值
        icon: data.icon || '📰', // 确保icon有默认值
        status: 'loading',
        createdAt: new Date(),
        lastUpdated: new Date(),
        count: 0
      }
      mockFeeds.unshift(newFeed)

      // 模拟加载过程
      setTimeout(() => {
        const feed = mockFeeds.find(f => f.id === newFeed.id)
        if (feed) {
          feed.status = 'active'
          feed.lastUpdated = new Date()
        }
      }, 2000)

      return newFeed
    },

    async updateFeed(id: string | number, data: UpdateFeedRequest): Promise<ApiFeed> {
      await delay()
      const numId = typeof id === 'string' ? parseInt(id, 10) : id
      const feedIndex = mockFeeds.findIndex(f => f.id === numId)
      if (feedIndex === -1) {
        throw new Error('订阅源不存在')
      }

      mockFeeds[feedIndex] = { ...mockFeeds[feedIndex], ...data }
      return mockFeeds[feedIndex]
    },

    async deleteFeed(id: number, cascade: boolean = false): Promise<null> {
      await delay()
      const feedIndex = mockFeeds.findIndex(f => f.id === id)
      if (feedIndex === -1) {
        throw new Error('订阅源不存在')
      }

      // 如果设置了cascade，则删除相关摘要
      if (cascade) {
        mockSummaries = mockSummaries.filter(s => s.feedId !== id)
      }

      // 删除订阅源
      mockFeeds.splice(feedIndex, 1)

      // 更新定时任务中的订阅源引用
      mockScheduledTasks.forEach(task => {
        task.selectedFeeds = task.selectedFeeds.filter(feedId => feedId !== id)
      })

      return null
    }
  },

  // 内容摘要管理
  summaries: {
    async getSummaries(params?: SummaryListParams): Promise<PaginationData<ApiSummary>> {
      await delay()
      let filteredSummaries = [...mockSummaries]

      if (params?.feedId) {
        filteredSummaries = filteredSummaries.filter(s => s.feedId === params.feedId)
      }
      if (params?.search) {
        const searchLower = params.search.toLowerCase()
        filteredSummaries = filteredSummaries.filter(s =>
          s.title.toLowerCase().includes(searchLower) ||
          s.content.toLowerCase().includes(searchLower)
        )
      }
      if (params?.tags) {
        const tags = params.tags.split(',').map(t => t.trim())
        filteredSummaries = filteredSummaries.filter(s =>
          tags.some(tag => s.tags.includes(tag))
        )
      }

      return createPaginatedResponse(filteredSummaries, params?.page, params?.pageSize)
    },

    async getSummary(id: string | number): Promise<ApiSummary> {
      await delay()
      const numId = typeof id === 'string' ? parseInt(id, 10) : id
      const summary = mockSummaries.find(s => s.id === numId)
      if (!summary) {
        throw new Error('摘要不存在')
      }
      return summary
    },

    async getRelatedSummaries(id: number, params?: RelatedSummaryParams): Promise<PaginationData<ApiSummary>> {
      await delay()
      const currentSummary = mockSummaries.find(s => s.id === id)
      if (!currentSummary) {
        throw new Error('摘要不存在')
      }

      // 增强的相关性算法：基于标签匹配、内容相似度和feed源
      const relatedSummaries = mockSummaries
        .filter(s => s.id !== id)
        .map(s => {
          let score = 0

          // 标签匹配得分 (权重 60%)
          const sharedTags = s.tags.filter(tag => currentSummary.tags.includes(tag))
          score += (sharedTags.length / Math.max(s.tags.length, currentSummary.tags.length)) * 0.6

          // 同源得分 (权重 30%)
          if (s.feedId === currentSummary.feedId) {
            score += 0.3
          }

          // 内容相似度得分 (权重 10%) - 简单的关键词匹配
          const currentWords = currentSummary.content.toLowerCase().split(' ')
          const summaryWords = s.content.toLowerCase().split(' ')
          const commonWords = currentWords.filter(word => summaryWords.includes(word) && word.length > 3)
          score += Math.min(commonWords.length / Math.max(currentWords.length, summaryWords.length), 0.1)

          return { ...s, relevanceScore: score }
        })
        .filter(s => s.relevanceScore >= (params?.minScore || 0.1))
        .sort((a, b) => b.relevanceScore - a.relevanceScore)
        .slice(0, params?.limit || 5)
        .map(s => {
          const { relevanceScore, ...summary } = s
          return summary
        })

      return createPaginatedResponse(relatedSummaries)
    },

    async addNote(summaryId: string | number, data: CreateNoteRequest): Promise<any> {
      await delay()
      const numId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
      const summary = mockSummaries.find(s => s.id === numId)
      if (!summary) {
        throw new Error('摘要不存在')
      }

      const newNote = {
        content: data.content,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      }

      summary.notesList = summary.notesList || []
      summary.notesList.push(newNote)
      summary.noteCount = summary.notesList.length

      return newNote
    },

    async updateNote(summaryId: string | number, noteId: string | number, data: UpdateNoteRequest): Promise<any> {
      await delay()
      const numSummaryId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
      const numNoteId = typeof noteId === 'string' ? parseInt(noteId, 10) : noteId
      const summary = mockSummaries.find(s => s.id === numSummaryId)
      if (!summary || !summary.notesList) {
        throw new Error('摘要或笔记不存在')
      }

      const noteIndex = numNoteId
      if (noteIndex >= 0 && noteIndex < summary.notesList.length) {
        summary.notesList[noteIndex] = {
          ...summary.notesList[noteIndex],
          content: data.content,
          updatedAt: new Date().toISOString()
        }
        return summary.notesList[noteIndex]
      }

      throw new Error('笔记不存在')
    },

    async deleteNote(summaryId: string | number, noteId: string | number): Promise<null> {
      await delay()
      const numSummaryId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
      const numNoteId = typeof noteId === 'string' ? parseInt(noteId, 10) : noteId
      const summary = mockSummaries.find(s => s.id === numSummaryId)
      if (!summary || !summary.notesList) {
        throw new Error('摘要或笔记不存在')
      }

      const noteIndex = numNoteId
      if (noteIndex >= 0 && noteIndex < summary.notesList.length) {
        summary.notesList.splice(noteIndex, 1)
        summary.noteCount = summary.notesList.length
        return null
      }

      throw new Error('笔记不存在')
    },

    async addTag(summaryId: string | number, data: AddTagRequest): Promise<{ tags: string[] }> {
      await delay()
      const numId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
      const summary = mockSummaries.find(s => s.id === numId)
      if (!summary) {
        throw new Error('摘要不存在')
      }

      if (!summary.tags.includes(data.tag)) {
        summary.tags.push(data.tag)
      }

      return { tags: summary.tags }
    },

    async removeTag(summaryId: string | number, tag: string): Promise<{ tags: string[] }> {
      await delay()
      const numId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
      const summary = mockSummaries.find(s => s.id === numId)
      if (!summary) {
        throw new Error('摘要不存在')
      }

      summary.tags = summary.tags.filter(t => t !== tag)
      return { tags: summary.tags }
    }
  },

  // 聊天/问答系统
  chat: {
    async getSessions(params?: ChatSessionListParams): Promise<PaginationData<ApiChatSession>> {
      await delay()
      console.log('📋 Mock API: 获取聊天会话列表', {
        totalSessions: mockChatSessions.length,
        sessions: mockChatSessions.map(s => ({ id: s.id, title: s.title }))
      })
      return createPaginatedResponse(mockChatSessions, params?.page, params?.pageSize)
    },

    async getSession(id: string | number): Promise<ApiChatSession> {
      await delay()
      const numId = typeof id === 'string' ? parseInt(id, 10) : id
      const session = mockChatSessions.find(s => s.id === numId)
      if (!session) {
        throw new Error('会话不存在')
      }
      return session
    },

    async createSession(data?: CreateChatSessionRequest): Promise<ApiChatSession> {
      await delay()
      const newSession: ApiChatSession = {
        id: Date.now(),
        title: data?.title || '新对话',
        createdAt: new Date(),
        updatedAt: new Date(),
        messageCount: 0,
        messages: []
      }

      if (data?.initialMessage) {
        const userMessage = {
          id: `msg-${Date.now()}-user`,
          type: 'user' as const,
          content: data.initialMessage,
          timestamp: new Date().toISOString()
        }
        newSession.messages?.push(userMessage)
        newSession.messageCount = 1
      }

      mockChatSessions.unshift(newSession)
      console.log('🆕 Mock API: 创建新会话成功', {
        sessionId: newSession.id,
        title: newSession.title,
        totalSessions: mockChatSessions.length
      })
      return newSession
    },

    async sendMessage(sessionId: string | number, data: SendMessageRequest): Promise<SendMessageResponse> {
      console.log('🗣️ Mock API: 接收到聊天消息', { sessionId, content: data.content })


      const numId = typeof sessionId === 'string' ? parseInt(sessionId, 10) : sessionId
      const session = mockChatSessions.find(s => s.id === numId)
      if (!session) {
        console.error('❌ Mock API: 会话不存在', { sessionId, numId, availableSessions: mockChatSessions.map(s => s.id) })
        throw new Error('会话不存在')
      }

      console.log('✅ Mock API: 找到会话', session.title)

      const userMessage = {
        id: `msg-${Date.now()}-user`,
        type: 'user' as const,
        content: data.content,
        timestamp: new Date().toISOString()
      }

      session.messages = session.messages || []
      session.messages.push(userMessage)
      session.messageCount = session.messages.length
      session.updatedAt = new Date()
      await delay(1500) // 模拟AI思考时间

      // 丰富的AI回复逻辑 - 基于真实摘要数据
      let aiContent = '这是一个很有趣的问题。基于你的知识库内容分析，我为你提供以下信息：'
      let referencedSummaries: { summaryId: number; summaryTitle: string }[] = []

      const lowerContent = data.content.toLowerCase()

      if (lowerContent.includes('rust')) {
        const rustSummary = mockSummaries.find(s => s.tags.includes('Rust'))
        if (rustSummary) {
          aiContent = '🦀 关于Rust编程语言，根据最新的资讯分析：\n\nRust 1.75版本带来了异步编程的重大改进，包括更好的错误处理和性能优化。新版本引入了更强大的并发原语，async/await语法更加直观，这使得Rust在系统编程和Web开发领域都表现出色。'
          referencedSummaries = [{ summaryId: rustSummary.id, summaryTitle: rustSummary.title }]
        }
      } else if (lowerContent.includes('vue') || lowerContent.includes('前端')) {
        const vueSummary = mockSummaries.find(s => s.tags.includes('Vue.js'))
        if (vueSummary) {
          aiContent = '💚 关于Vue.js和前端开发，基于你的订阅内容：\n\nVue 3.5版本进一步优化了组合式API的性能和易用性。新增的响应式语法糖让代码更加简洁，改进的类型推导提供了更好的TypeScript支持。这些改进使得Vue.js在大型项目中的表现更加出色。'
          referencedSummaries = [{ summaryId: vueSummary.id, summaryTitle: vueSummary.title }]
        }
      } else if (lowerContent.includes('ai') || lowerContent.includes('人工智能') || lowerContent.includes('代码')) {
        const aiSummary = mockSummaries.find(s => s.tags.includes('AI'))
        if (aiSummary) {
          aiContent = '🤖 关于AI和代码生成技术，从你的知识库分析：\n\n最新研究表明，结合静态分析工具的大型语言模型在代码生成任务上表现出了惊人的准确性。这种技术突破不仅提高了代码质量，还为自动化编程和开发效率提升带来了新的可能性。'
          referencedSummaries = [{ summaryId: aiSummary.id, summaryTitle: aiSummary.title }]
        }
      } else if (lowerContent.includes('webassembly') || lowerContent.includes('wasm')) {
        const wasmSummary = mockSummaries.find(s => s.tags.includes('WebAssembly'))
        if (wasmSummary) {
          aiContent = '⚡ 关于WebAssembly技术，根据订阅源的内容分析：\n\nWASM在实际应用中展现了强大的性能潜力，特别是在图像处理、游戏引擎、加密算法等计算密集型场景中显著提升了性能。它为Web平台带来了接近原生应用的执行效率。'
          referencedSummaries = [{ summaryId: wasmSummary.id, summaryTitle: wasmSummary.title }]
        }
      } else if (lowerContent.includes('hello') || lowerContent.includes('你好') || lowerContent.includes('hi')) {
        aiContent = '👋 你好！我是AI助手，很高兴为你服务！\n\n我可以基于你的知识库内容回答各种技术问题，比如编程语言（Rust、Vue.js）、AI技术、Web开发等。请随时向我提问，我会根据你订阅的最新资讯为你提供有价值的信息。'
        referencedSummaries = []
      } else {
        // 通用搜索 - 找到最相关的摘要
        const relatedSummaries = mockSummaries.filter(s =>
          s.title.toLowerCase().includes(lowerContent) ||
          s.content.toLowerCase().includes(lowerContent) ||
          s.tags.some(tag => tag.toLowerCase().includes(lowerContent))
        ).slice(0, 2)

        if (relatedSummaries.length > 0) {
          aiContent = `💡 关于"${data.content}"，我在你的知识库中找到了相关内容：\n\n${relatedSummaries[0].content.substring(0, 200)}...\n\n如果你需要更详细的信息，可以查看完整的摘要内容。`
          referencedSummaries = relatedSummaries.map(s => ({ summaryId: s.id, summaryTitle: s.title }))
        } else {
          aiContent = `💡 关于"${data.content}"，我正在基于你的知识库进行分析...\n\n根据你订阅的技术资讯，这个话题涉及多个方面。我建议你可以通过以下关键词进一步探索：编程语言发展、技术栈选择、最佳实践等。如果你有更具体的问题，我可以提供更详细的分析。`
          referencedSummaries = []
        }
      }

      // 更新会话
      const assistantMessage = {
        id: `msg-${Date.now()}-assistant`,
        type: 'assistant' as const,
        content: aiContent,
        sources: referencedSummaries,
        timestamp: new Date().toISOString()
      }
      session.messages.push(assistantMessage)
      session.messageCount = session.messages.length
      session.updatedAt = new Date()

      console.log('💬 Mock API: 消息发送完成', {
        sessionId: session.id,
        messageCount: session.messageCount,
        userMessage: userMessage.content,
        aiResponse: assistantMessage.content
      })

      return {
        userMessage,
        assistantMessage
      }
    },

    async updateSession(id: number, data: UpdateChatSessionRequest): Promise<Pick<ApiChatSession, 'id' | 'title' | 'updatedAt'>> {
      await delay()
      const session = mockChatSessions.find(s => s.id === id)
      if (!session) {
        throw new Error('会话不存在')
      }

      session.title = data.title
      session.updatedAt = new Date()

      return {
        id: session.id,
        title: session.title,
        updatedAt: session.updatedAt
      }
    },

    async deleteSession(id: number): Promise<null> {
      await delay()
      const sessionIndex = mockChatSessions.findIndex(s => s.id === id)
      if (sessionIndex === -1) {
        throw new Error('会话不存在')
      }

      mockChatSessions.splice(sessionIndex, 1)
      return null
    }
  },

  // 同步管理
  sync: {
    async manualSync(data?: ManualSyncRequest): Promise<SyncStartResponse> {
      await delay()
      const syncId = `sync-${Date.now()}`

      return {
        syncId,
        status: 'started',
        startTime: new Date().toISOString(),
        estimatedDuration: 120,
        feedCount: data?.feedIds?.length || mockFeeds.length
      }
    },

    async getSyncStatus(params?: SyncStatusParams): Promise<SyncStatusResponse> {
      await delay()

      return {
        isRunning: false,
        lastSyncTime: '2025-07-29T09:00:00Z',
        lastSyncDuration: 85,
        errors: []
      }
    },

    async getSyncHistory(params?: SyncHistoryParams): Promise<PaginationData<ApiSyncHistory>> {
      await delay()

      const mockHistory: ApiSyncHistory[] = [
        {
          syncId: 'sync-20250729-081500',
          startTime: '2025-07-29T08:15:00Z',
          endTime: '2025-07-29T08:16:25Z',
          duration: 85,
          status: 'completed',
          feedsProcessed: 3,
          itemsProcessed: 47,
          summariesGenerated: 12,
          emailSent: false,
          errors: 1
        }
      ]

      return createPaginatedResponse(mockHistory, params?.page, params?.pageSize)
    },

    async cancelSync(data: CancelSyncRequest): Promise<CancelSyncResponse> {
      await delay()

      return {
        syncId: data.syncId,
        status: 'cancelled',
        cancelledAt: new Date().toISOString()
      }
    }
  },

  // 定时任务管理
  tasks: {
    async getTasks(): Promise<{ items: ApiScheduledTask[] }> {
      await delay()
      return { items: mockScheduledTasks }
    },

    async createTask(data: CreateScheduledTaskRequest): Promise<ApiScheduledTask> {
      await delay()
      const newTask: ApiScheduledTask = {
        id: `task-${Date.now()}`,
        ...data,
        enabled: data.enabled ?? false,
        nextRun: new Date(Date.now() + 24 * 60 * 60 * 1000),
        lastRun: null,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      }

      mockScheduledTasks.push(newTask)
      return newTask
    },

    async updateTask(id: string, data: UpdateScheduledTaskRequest): Promise<Pick<ApiScheduledTask, 'id' | 'name' | 'enabled' | 'cronExpression' | 'nextRun' | 'updatedAt'>> {
      await delay()
      const task = mockScheduledTasks.find(t => t.id === id)
      if (!task) {
        throw new Error('任务不存在')
      }

      Object.assign(task, data)
      task.updatedAt = new Date().toISOString()

      return {
        id: task.id,
        name: task.name,
        enabled: task.enabled,
        cronExpression: task.cronExpression,
        nextRun: task.nextRun,
        updatedAt: task.updatedAt
      }
    },

    async deleteTask(id: string): Promise<null> {
      await delay()
      const taskIndex = mockScheduledTasks.findIndex(t => t.id === id)
      if (taskIndex === -1) {
        throw new Error('任务不存在')
      }

      mockScheduledTasks.splice(taskIndex, 1)
      return null
    },

    async executeTask(id: string): Promise<ExecuteTaskResponse> {
      await delay()
      const task = mockScheduledTasks.find(t => t.id === id)
      if (!task) {
        throw new Error('任务不存在')
      }

      return {
        taskId: id,
        executionId: `exec-${Date.now()}`,
        startTime: new Date().toISOString(),
        status: 'started'
      }
    },

    async getTaskExecutions(id: string, params?: TaskExecutionParams): Promise<PaginationData<ApiTaskExecution>> {
      await delay()

      const mockExecutions: ApiTaskExecution[] = [
        {
          executionId: 'exec-20250729-090000',
          startTime: '2025-07-29T09:00:00Z',
          endTime: '2025-07-29T09:02:15Z',
          duration: 135,
          status: 'success',
          feedsProcessed: 2,
          summariesGenerated: 8,
          emailSent: true,
          logs: [
            '开始同步 Hacker News',
            '获取到 12 条新内容',
            '生成 AI 摘要完成',
            '发送邮件成功'
          ]
        }
      ]

      return createPaginatedResponse(mockExecutions, params?.page, params?.pageSize)
    }
  },

  // 邮件配置
  email: {
    async getConfig(): Promise<ApiEmailFullConfig> {
      await delay()

      return {
        enabled: true,
        recipientEmails: ['user@example.com', 'team@example.com'],
        senderName: '小天AI助手',
        template: {
          subject: '每日技术资讯摘要 - {date}',
          headerText: '以下是今日为您精选的技术资讯摘要：',
          footerText: '感谢使用小天AI助手',
          includeOriginalLinks: true,
          groupByFeed: true
        },
        smtpConfig: {
          host: 'smtp.example.com',
          port: 587,
          username: 'xiaotian@example.com',
          authConfigured: true
        }
      }
    },

    async updateConfig(data: UpdateEmailConfigRequest): Promise<any> {
      await delay()

      return {
        enabled: data.enabled ?? true,
        recipientEmails: data.recipientEmails || ['user@example.com'],
        senderName: data.senderName || '小天AI助手',
        template: data.template || {},
        updatedAt: new Date().toISOString()
      }
    },

    async testConfig(data: TestEmailRequest): Promise<TestEmailResponse> {
      await delay(2000) // 模拟邮件发送时间

      return {
        messageId: `test-${Date.now()}`,
        sentAt: new Date().toISOString(),
        recipients: data.recipientEmails,
        deliveryStatus: 'sent'
      }
    },

    async updateSmtpConfig(data: UpdateSmtpConfigRequest): Promise<ApiSmtpConfig> {
      await delay()

      return {
        host: data.host,
        port: data.port,
        username: data.username,
        authConfigured: true,
        useTLS: data.useTLS ?? true,
        updatedAt: new Date().toISOString()
      }
    }
  },

  // 系统管理
  system: {
    async getStats(): Promise<ApiSystemStats> {
      await delay()

      // 计算基于时间的摘要统计
      const now = new Date()
      const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
      const weekAgo = new Date(today.getTime() - 7 * 24 * 60 * 60 * 1000)
      const monthAgo = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000)

      const todaySummaries = mockSummaries.filter(s => {
        const createdDate = new Date(s.createdAt)
        return createdDate >= today
      }).length

      const thisWeekSummaries = mockSummaries.filter(s => {
        const createdDate = new Date(s.createdAt)
        return createdDate >= weekAgo
      }).length

      const thisMonthSummaries = mockSummaries.filter(s => {
        const createdDate = new Date(s.createdAt)
        return createdDate >= monthAgo
      }).length

      return {
        feeds: {
          total: mockFeeds.length,
          active: mockFeeds.filter(f => f.status === 'active').length,
          error: mockFeeds.filter(f => f.status === 'error').length
        },
        summaries: {
          total: mockSummaries.length,
          today: todaySummaries,
          thisWeek: thisWeekSummaries,
          thisMonth: thisMonthSummaries
        },
        chatSessions: {
          total: mockChatSessions.length,
          active: mockChatSessions.filter(s => s.messages && s.messages.length > 0).length,
          messagesTotal: mockChatSessions.reduce((sum, s) => sum + s.messageCount, 0)
        },
        scheduledTasks: {
          total: mockScheduledTasks.length,
          enabled: mockScheduledTasks.filter(t => t.enabled).length,
          lastRunSuccess: mockScheduledTasks.length > 0 ? mockScheduledTasks[0].lastRunStatus === 'success' : true
        },
        sync: {
          totalRuns: 120,
          successRate: 0.95,
          avgDuration: 78,
          lastSync: mockScheduledTasks.length > 0 && mockScheduledTasks[0].lastRun ?
            mockScheduledTasks[0].lastRun.toISOString() : '2025-07-29T09:00:00Z'
        }
      }
    },

    async getHealth(): Promise<ApiSystemHealth> {
      await delay()

      return {
        status: 'healthy',
        timestamp: new Date().toISOString(),
        version: 'v0.0.4',
        uptime: 86400,
        checks: {
          database: {
            status: 'healthy',
            responseTime: 15
          },
          redis: {
            status: 'healthy',
            responseTime: 3
          },
          email: {
            status: 'healthy',
            lastTest: '2025-07-29T09:30:00Z'
          },
          feeds: {
            status: 'healthy',
            accessibleCount: 3,
            totalCount: 4
          }
        }
      }
    },

    async getLogs(params?: SystemLogsParams): Promise<PaginationData<ApiSystemLog>> {
      await delay()

      const mockLogs: ApiSystemLog[] = [
        {
          id: 'log-20250729-104500',
          level: 'error',
          message: '订阅源同步失败',
          module: 'sync',
          feedId: 4,
          error: 'Connection timeout after 30s',
          timestamp: '2025-07-29T10:45:00Z',
          details: {
            url: 'https://www.reddit.com/r/programming/.rss',
            retryCount: 3
          }
        }
      ]

      return createPaginatedResponse(mockLogs, params?.page, params?.pageSize)
    },

    async cleanup(data: SystemCleanupRequest): Promise<SystemCleanupResponse> {
      await delay(3000) // 模拟清理时间

      return {
        cleanupType: data.cleanupType,
        itemsDeleted: 250,
        spaceFreed: '15.2MB',
        executedAt: new Date().toISOString()
      }
    }
  }
}

// 检查是否使用Mock
export function shouldUseMock(): boolean {
  return API_CONFIG.USE_MOCK
}

import type {
  ChatMessage,
  ChatSession,
  Feed,
  NewFeedData,
  Note,
  QAReturnContext,
  RelatedSummary,
  ScheduledTaskConfig,
  Summary,
  SyncStatus,
  ViewType
} from '@/types'
import { marked } from 'marked'
import { defineStore } from 'pinia'
import { computed, reactive, ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  // 当前视图状态
  const currentView = ref<ViewType>('summary')
  const currentDetail = ref<Summary | null>(null)
  const currentQAQuery = ref<string>('')

  // 当前选中的订阅源
  const selectedFeed = ref<string>('Hacker News')

  // 反馈消息
  const feedbackMessage = ref<string>('')
  const showFeedback = ref<boolean>(false)

  // 同步状态
  const syncStatus = ref<SyncStatus>({
    isRunning: false,
    lastSyncTime: null,
    progress: 0,
    currentAction: '',
    errors: []
  })

  // 定时任务配置
  const scheduledTasks = ref<ScheduledTaskConfig[]>([
    {
      id: 'default-task',
      name: '每日技术资讯推送',
      enabled: false,
      cronExpression: '0 9 * * *',
      cronDescription: '每天上午9点',
      nextRun: new Date(Date.now() + 24 * 60 * 60 * 1000), // 明天
      lastRun: null,
      emailConfig: {
        enabled: false,
        recipientEmails: [],
        senderName: '小天AI助手'
      },
      selectedFeeds: [],
      aiSummaryEnabled: true,
      summaryLength: 'medium'
    }
  ])

  // QA 相关状态 - 多会话管理
  const qaChatSessions = ref<ChatSession[]>([
    {
      id: 'session-1',
      title: 'Rust 性能更新',
      createdAt: new Date(),
      updatedAt: new Date(),
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
          content: '根据你的知识库，Rust 在最新版本中发布了重要的异步编程改进。Rust 1.75 版本带来了期待已久的异步编程改进，包括更好的错误处理、性能优化和开发体验提升。新版本的 async/await 语法更加直观，同时引入了更强大的并发原语。这些改进使得 Rust 在构建高性能异步应用方面更加强大，特别是在网络服务和系统编程领域。',
          sources: ['Rust 1.75 版本发布：异步编程的重大改进'],
          timestamp: new Date().toISOString()
        }
      ]
    }
  ])

  // 当前活跃的会话ID
  const currentChatSessionId = ref<string>('session-1')

  // 记住从QA跳转前的状态，用于返回
  const qaReturnContext = ref<QAReturnContext | null>(null)

  // 当前会话的消息（计算属性）
  const currentChatMessages = computed<ChatMessage[]>(() => {
    const session = qaChatSessions.value.find(s => s.id === currentChatSessionId.value)
    return session ? session.messages : []
  })

  // 当前会话信息（计算属性）
  const currentChatSession = computed<ChatSession | undefined>(() => {
    return qaChatSessions.value.find(s => s.id === currentChatSessionId.value)
  })

  // 订阅源数据 - 移除硬编码的count，改为动态计算
  const feeds = reactive<Feed[]>([
    {
      name: 'Hacker News',
      description: '技术新闻和讨论社区，汇聚全球程序员的智慧和前沿科技趋势',
      feedUrl: 'https://hnrss.org/frontpage',
      icon: '🔥', // 添加icon
      id: 'hacker-news',
      category: '科技',
      lastUpdated: new Date(Date.now() - 2 * 60 * 60 * 1000), // 2小时前
      status: 'active' // active, error, loading
    },
    {
      name: 'Rust Blog',
      description: 'Rust 编程语言官方博客，最新版本发布、性能优化和社区动态',
      feedUrl: 'https://blog.rust-lang.org/feed.xml',
      icon: '🦀',
      id: 'rust-blog',
      category: '编程',
      lastUpdated: new Date(Date.now() - 24 * 60 * 60 * 1000), // 1天前
      status: 'active'
    },
    {
      name: 'Vue.js Blog',
      description: 'Vue.js 官方博客，框架更新、最佳实践和前端生态发展资讯',
      feedUrl: 'https://blog.vuejs.org/feed.rss',
      icon: '💚',
      id: 'vue-blog',
      category: '前端',
      lastUpdated: new Date(Date.now() - 6 * 60 * 60 * 1000), // 6小时前
      status: 'active'
    },
    {
      name: 'www.reddit.com',
      description: '全球最大的在线社区和讨论平台，涵盖科技、编程、设计等话题',
      feedUrl: 'https://www.reddit.com/r/programming/.rss',
      icon: '🤖',
      id: 'reddit',
      category: '社区',
      lastUpdated: new Date(Date.now() - 30 * 60 * 1000), // 30分钟前
      status: 'error' // 演示错误状态
    }
  ])

  // 按订阅源分组的摘要文章
  const feedSummaries = reactive<{ [key: string]: Summary[] }>({
    'hacker-news': [
      {
        id: '1',
        title: '大型语言模型在代码生成领域的最新进展',
        originalUrl: 'https://news.ycombinator.com/item?id=123456',
        publishedAt: '2025年7月8日',
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

## 🎯 实际应用场景

### 自动化测试生成
- 智能生成单元测试用例
- 覆盖边界条件和异常处理
- 确保测试的完整性和有效性

### 代码重构建议
- 识别重复代码模式
- 提供性能优化建议
- 保持代码的可维护性

### API设计优化
- 分析接口的一致性
- 检测设计模式的合理性
- 提供最佳实践建议

## 💡 未来发展方向

1. **多语言支持扩展** - 覆盖更多编程语言和框架
2. **实时协作集成** - 与IDE深度集成，提供实时代码建议
3. **领域特化模型** - 针对特定行业（金融、医疗、IoT）的专业化模型

## ⚠️ 注意事项与限制

- **人工审查必要性**：生成的代码仍需要人工代码审查
- **上下文依赖性**：在复杂业务逻辑中可能需要额外的上下文信息
- **安全性考量**：敏感代码的生成需要额外的安全验证

---

*这一技术突破不仅提高了代码生成的质量，更为软件开发工作流程的自动化开辟了新的可能性。*`,
        tags: ['AI', '代码生成', '静态分析', 'LLM'],
        notesList: [
          { content: '这个技术可能会改变整个编程行业', createdAt: '2025-01-15 10:30' },
          { content: '需要关注对传统开发流程的影响', createdAt: '2025-01-15 11:00' }
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
            id: '3',
            title: 'Rust 1.75 版本发布：异步编程的重大改进',
            relevanceScore: 0.75,
            relationType: 'content',
            sharedTags: ['编程语言', '性能优化'],
            publishedAt: '2025年7月6日',
            excerpt: 'Rust 1.75版本在编程语言演进方面的重要突破...'
          },
          {
            id: '4',
            title: 'Vue 3.5 带来的组合式 API 优化',
            relevanceScore: 0.68,
            relationType: 'content',
            sharedTags: ['代码生成', 'TypeScript'],
            publishedAt: '2025年7月5日',
            excerpt: 'Vue.js在开发体验优化方面的最新进展...'
          }
        ]
      },
      {
        id: '2',
        title: 'WebAssembly 在浏览器性能优化中的实际应用',
        originalUrl: 'https://news.ycombinator.com/item?id=789012',
        publishedAt: '2025年7月7日',
        content: 'WebAssembly (WASM) 作为新一代 Web 技术，在实际应用中展现了强大的性能潜力。本文通过多个真实案例，展示了 WASM 如何在图像处理、游戏引擎、加密算法等场景中显著提升性能...',
        fullContent: `# WebAssembly 性能优化实战指南

## 🌟 技术概述

**WebAssembly (WASM)** 作为新一代 Web 技术标准，正在重新定义浏览器应用的性能边界。通过将低级字节码引入 Web 环境，WASM 实现了**接近原生应用的执行速度**。

## 🎯 核心应用场景

### 图像与视频处理
- **实时滤镜效果**：Instagram 风格的实时图像处理
- **视频编解码**：在浏览器中进行 H.264/H.265 编解码
- **计算机视觉**：人脸识别、物体检测等 AI 应用

\`\`\`javascript
// WASM 模块调用示例
const wasmModule = await WebAssembly.instantiateStreaming(
  fetch('image-processor.wasm')
);

function processImage(imageData) {
  const result = wasmModule.instance.exports.applyFilter(
    imageData.data.buffer,
    imageData.width,
    imageData.height
  );
  return new ImageData(new Uint8ClampedArray(result), imageData.width);
}
\`\`\`

### 游戏引擎优化
- **物理引擎**：复杂的物理模拟计算
- **渲染管线**：3D 图形渲染优化
- **音频处理**：实时音频合成和效果处理

### 密码学与安全
- **加密算法**：AES、RSA 等加密运算
- **哈希计算**：大数据的哈希处理
- **数字签名**：区块链相关的签名验证

## 📈 性能对比分析

| 测试场景 | JavaScript | WebAssembly | 性能提升 |
|----------|------------|-------------|----------|
| **图像处理** | 2.3s | 0.8s | **187%** |
| **数学运算** | 1.5s | 0.4s | **275%** |
| **字符串操作** | 0.9s | 0.6s | **50%** |
| **内存操作** | 1.2s | 0.3s | **300%** |

> 💡 **性能提升关键**：WASM 在计算密集型任务中的优势最为明显

## 🛠️ 开发工具链

### 编译器支持
- **Emscripten**：C/C++ 到 WASM 的完整工具链
- **wasm-pack**：Rust 生态的 WASM 编译工具
- **AssemblyScript**：TypeScript-like 语法编写 WASM

### 调试与优化
- **Chrome DevTools**：WASM 调试支持
- **wabt**：WebAssembly 二进制工具包
- **Binaryen**：WASM 优化器

## 🚀 实施建议

### 1. 识别性能瓶颈
- 使用 **Performance API** 测量 JavaScript 性能
- 识别 CPU 密集型计算部分
- 评估 WASM 迁移的成本效益

### 2. 渐进式迁移
- 从**独立模块**开始迁移
- 保持 JavaScript 和 WASM 的清晰接口
- 建立完善的测试覆盖

### 3. 生产部署
- **GZIP 压缩**：减小 WASM 文件大小
- **CDN 缓存**：优化加载速度
- **懒加载**：按需加载 WASM 模块

## ⚡ 未来展望

随着 **WASI (WebAssembly System Interface)** 和 **Component Model** 的发展，WASM 将超越浏览器环境，成为：

- 🌐 **云原生应用**的轻量级运行时
- 🔧 **边缘计算**的标准执行环境
- 🏗️ **微服务架构**的高性能组件

---

*WebAssembly 正在成为现代 Web 应用不可或缺的性能优化工具，值得每个前端开发者深入学习。*`,
        tags: ['WebAssembly', '性能优化', '浏览器技术'],
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
          },
          {
            id: 'source-2-2',
            title: 'Real-world WASM Applications: Case Studies',
            url: 'https://developer.mozilla.org/en-US/blog/wasm-case-studies/',
            publishedAt: '2025-01-13T09:20:00Z',
            author: 'MDN Team',
            source: 'MDN Blog',
            excerpt: 'Mozilla开发者网络分享的WebAssembly实际应用案例研究...',
            wordCount: 6200,
            readingTime: 10,
            language: 'en',
            contentType: 'article'
          }
        ],
        relatedSummaries: [
          {
            id: '1',
            title: '大型语言模型在代码生成领域的最新进展',
            relevanceScore: 0.62,
            relationType: 'content',
            sharedTags: ['性能优化'],
            publishedAt: '2025年7月8日',
            excerpt: 'AI技术在提升开发效率和代码质量方面的突破...'
          },
          {
            id: '4',
            title: 'Vue 3.5 带来的组合式 API 优化',
            relevanceScore: 0.58,
            relationType: 'content',
            sharedTags: ['性能优化', '浏览器技术'],
            publishedAt: '2025年7月5日',
            excerpt: '前端框架在性能优化方面的持续改进...'
          }
        ]
      }
    ],
    'rust-blog': [
      {
        id: '3',
        title: 'Rust 1.75 版本发布：异步编程的重大改进',
        originalUrl: 'https://blog.rust-lang.org/2025/01/15/Rust-1.75.0.html',
        publishedAt: '2025年7月6日',
        content: 'Rust 1.75 版本带来了期待已久的异步编程改进，包括更好的错误处理、性能优化和开发体验提升。新版本的 async/await 语法更加直观，同时引入了更强大的并发原语...',
        fullContent: 'Rust 1.75 版本带来了期待已久的异步编程改进，包括更好的错误处理、性能优化和开发体验提升。新版本的 async/await 语法更加直观，同时引入了更强大的并发原语。这些改进使得 Rust 在构建高性能异步应用方面更加强大，特别是在网络服务和系统编程领域。',
        tags: ['Rust', '异步编程', '版本发布'],
        notesList: [
          { content: '需要测试现有代码的兼容性', createdAt: '2025-01-15 14:20' }
        ],
        sourceMaterials: [
          {
            id: 'source-3-1',
            title: 'Announcing Rust 1.75.0',
            url: 'https://blog.rust-lang.org/2025/01/15/Rust-1.75.0.html',
            publishedAt: '2025-01-15T18:00:00Z',
            author: 'Rust Team',
            source: 'Rust官方博客',
            excerpt: 'Rust 1.75.0版本正式发布，带来异步编程重大改进...',
            wordCount: 5500,
            readingTime: 9,
            language: 'en',
            contentType: 'article'
          },
          {
            id: 'source-3-2',
            title: 'Performance Improvements in Rust 1.75 Async Runtime',
            url: 'https://tokio.rs/blog/2025-01-rust-1-75-performance',
            publishedAt: '2025-01-16T10:30:00Z',
            author: 'Tokio Team',
            source: 'Tokio官方博客',
            excerpt: 'Tokio团队分析Rust 1.75异步运行时的性能提升...',
            wordCount: 3800,
            readingTime: 6,
            language: 'en',
            contentType: 'article'
          }
        ],
        relatedSummaries: [
          {
            id: '1',
            title: '大型语言模型在代码生成领域的最新进展',
            relevanceScore: 0.75,
            relationType: 'content',
            sharedTags: ['编程语言', '性能优化'],
            publishedAt: '2025年7月8日',
            excerpt: 'AI在编程语言生态系统中的应用和影响...'
          },
          {
            id: '2',
            title: 'WebAssembly 在浏览器性能优化中的实际应用',
            relevanceScore: 0.65,
            relationType: 'content',
            sharedTags: ['性能优化'],
            publishedAt: '2025年7月7日',
            excerpt: '高性能Web技术的发展趋势和应用实践...'
          }
        ]
      }
    ],
    'vue-blog': [
      {
        id: '4',
        title: 'Vue 3.5 带来的组合式 API 优化',
        originalUrl: 'https://blog.vuejs.org/posts/vue-3-5.html',
        publishedAt: '2025年7月5日',
        content: 'Vue 3.5 版本进一步优化了组合式 API 的性能和易用性。新增的响应式语法糖让代码更加简洁，同时改进的类型推导提供了更好的 TypeScript 支持...',
        fullContent: 'Vue 3.5 版本进一步优化了组合式 API 的性能和易用性。新增的响应式语法糖让代码更加简洁，同时改进的类型推导提供了更好的 TypeScript 支持。这些改进使得 Vue.js 在大型项目中的表现更加出色，开发体验也得到了显著提升。',
        tags: ['Vue.js', '组合式API', 'TypeScript'],
        notesList: [],
        sourceMaterials: [
          {
            id: 'source-4-1',
            title: 'Vue 3.5 Released: Performance and DX Improvements',
            url: 'https://blog.vuejs.org/posts/vue-3-5.html',
            publishedAt: '2025-01-10T14:00:00Z',
            author: 'Vue Team',
            source: 'Vue.js官方博客',
            excerpt: 'Vue 3.5正式发布，带来性能提升和开发体验改进...',
            wordCount: 4200,
            readingTime: 7,
            language: 'en',
            contentType: 'article'
          },
          {
            id: 'source-4-2',
            title: 'Deep Dive: Vue 3.5 Reactivity Improvements',
            url: 'https://vue-land.com/articles/vue-3-5-reactivity-deep-dive',
            publishedAt: '2025-01-11T11:15:00Z',
            author: 'Anthony Fu',
            source: 'Vue Land',
            excerpt: 'Vue.js核心团队成员深入解析3.5版本响应式系统改进...',
            wordCount: 6800,
            readingTime: 11,
            language: 'en',
            contentType: 'article'
          },
          {
            id: 'source-4-3',
            title: 'TypeScript Support in Vue 3.5: What\'s New',
            url: 'https://typescript.org/docs/vue-3-5-support',
            publishedAt: '2025-01-12T09:45:00Z',
            author: 'TypeScript Team',
            source: 'TypeScript文档',
            excerpt: 'TypeScript官方团队介绍对Vue 3.5的支持改进...',
            wordCount: 2900,
            readingTime: 5,
            language: 'en',
            contentType: 'article'
          }
        ],
        relatedSummaries: [
          {
            id: '1',
            title: '大型语言模型在代码生成领域的最新进展',
            relevanceScore: 0.68,
            relationType: 'content',
            sharedTags: ['TypeScript', '代码生成'],
            publishedAt: '2025年7月8日',
            excerpt: '前端开发工具和AI技术的融合趋势...'
          },
          {
            id: '2',
            title: 'WebAssembly 在浏览器性能优化中的实际应用',
            relevanceScore: 0.58,
            relationType: 'content',
            sharedTags: ['性能优化', '前端技术'],
            publishedAt: '2025年7月7日',
            excerpt: '现代前端技术栈中的性能优化策略...'
          },
          {
            id: '3',
            title: 'Rust 1.75 版本发布：异步编程的重大改进',
            relevanceScore: 0.45,
            relationType: 'temporal',
            publishedAt: '2025年7月6日',
            excerpt: '同期编程语言生态系统的发展动态...'
          }
        ]
      }
    ],
    'reddit': [] // Reddit 为空，模拟错误状态的订阅源
  })

  // 动态计算每个订阅源的文章数量
  const feedsWithCount = computed(() => {
    return feeds.map(feed => ({
      ...feed,
      count: feedSummaries[feed.id]?.length || 0
    }))
  })

  // 根据选中的订阅源过滤摘要
  const summaries = computed(() => {
    const feedId = feeds.find(f => f.name === selectedFeed.value)?.id
    return feedId ? feedSummaries[feedId] || [] : []
  })

  // 格式化时间的辅助函数
  function formatTimeAgo(date: Date | string | null) {
    if (!date) return '未知时间'

    const now = new Date()
    const diff = now.getTime() - new Date(date).getTime()
    const minutes = Math.floor(diff / 60000)
    const hours = Math.floor(diff / 3600000)
    const days = Math.floor(diff / 86400000)

    if (minutes < 60) {
      return `${minutes}分钟前`
    } else if (hours < 24) {
      return `${hours}小时前`
    } else {
      return `${days}天前`
    }
  }

  // 显示反馈消息
  function showFeedbackMessage(message: string, duration = 3000) {
    feedbackMessage.value = message
    showFeedback.value = true
    setTimeout(() => {
      showFeedback.value = false
    }, duration)
  }

  // 选择订阅源
  function selectFeed(feedName: string) {
    console.log('选择订阅源:', feedName)
    selectedFeed.value = feedName
  }

  // 添加新的订阅源
  function addFeed(feedData: NewFeedData) {
    console.log('添加订阅源:', feedData)

    // 验证必要字段
    if (!feedData.feedUrl || !feedData.feedUrl.trim()) {
      showFeedbackMessage('RSS链接不能为空')
      return false
    }

    // 检查是否已存在
    const exists = feeds.some(feed =>
      feed.feedUrl === feedData.feedUrl.trim() ||
      feed.name === feedData.name.trim()
    )

    if (exists) {
      showFeedbackMessage('该订阅源已存在')
      return false
    }

    // 生成新的订阅源
    const newFeed: Feed = {
      name: feedData.name.trim() || new URL(feedData.feedUrl).hostname,
      description: feedData.description.trim() || '新添加的订阅源',
      feedUrl: feedData.feedUrl.trim(),
      icon: '📰', // 默认图标
      id: `feed-${Date.now()}`,
      category: feedData.category || '其他',
      lastUpdated: new Date(),
      status: 'loading'
    }

    // 添加到列表顶部（最新的在前面）
    feeds.unshift(newFeed)
    feedSummaries[newFeed.id] = [] // 初始化为空数组

    // 模拟加载过程
    setTimeout(() => {
      const feed = feeds.find(f => f.id === newFeed.id)
      if (feed) {
        feed.status = 'active'
      }
    }, 2000)

    showFeedbackMessage('订阅源添加成功！')
    return true
  }

  // 视图切换函数
  function switchToSummaryView() {
    currentView.value = 'summary'
    currentDetail.value = null
  }

  function switchToQAView(query: string = '') {
    currentView.value = 'qa'
    currentQAQuery.value = query
  }

  function switchToDetailView(summary: Summary) {
    currentView.value = 'detail'
    currentDetail.value = summary
  }

  // 模拟 AI 回答生成
  function generateAIAnswer(question: string) {
    const responses = {
      'vue': {
        content: '关于 Vue.js 的问题，基于你的知识库：Vue 3.5 版本进一步优化了组合式 API 的性能和易用性。新增的响应式语法糖让代码更加简洁，同时改进的类型推导提供了更好的 TypeScript 支持。这些改进使得 Vue.js 在大型项目中的表现更加出色。',
        sources: ['Vue 3.5 带来的组合式 API 优化'] // 使用真实的文章标题
      },
      'rust': {
        content: '关于 Rust 的问题，根据最新资讯：Rust 1.75 版本带来了异步编程的重大改进，包括更好的错误处理和性能优化。新版本的 async/await 语法更加直观，引入了更强大的并发原语。',
        sources: ['Rust 1.75 版本发布：异步编程的重大改进'] // 使用真实的文章标题
      },
      'webassembly': {
        content: '关于 WebAssembly，从你的订阅内容来看：WASM 在实际应用中展现了强大的性能潜力，特别是在图像处理、游戏引擎、加密算法等场景中显著提升性能。',
        sources: ['WebAssembly 在浏览器性能优化中的实际应用'] // 使用真实的文章标题
      },
      'ai': {
        content: '关于人工智能和代码生成，根据最新研究：结合静态分析工具的大型语言模型在代码生成任务上表现出了惊人的准确性，这种技术突破为自动化编程带来了新的可能性。',
        sources: ['大型语言模型在代码生成领域的最新进展'] // 使用真实的文章标题
      }
    }

    const lowerQuestion = question.toLowerCase()
    if (lowerQuestion.includes('vue')) {
      return responses.vue
    } else if (lowerQuestion.includes('rust')) {
      return responses.rust
    } else if (lowerQuestion.includes('webassembly') || lowerQuestion.includes('wasm')) {
      return responses.webassembly
    } else if (lowerQuestion.includes('ai') || lowerQuestion.includes('人工智能') || lowerQuestion.includes('代码生成') || lowerQuestion.includes('llm')) {
      return responses.ai
    } else {
      // 对于其他问题，从现有文章中随机选择一些作为参考
      const allArticles = []
      for (const feedId in feedSummaries) {
        allArticles.push(...feedSummaries[feedId].map(s => s.title))
      }
      const randomSources = allArticles.slice(0, 2) // 取前两篇作为参考

      return {
        content: `关于"${question}"的问题，基于你的知识库内容分析，这是一个很有意思的技术话题。从订阅的文章中可以看出，相关技术正在快速发展，建议你关注最新的技术动态和最佳实践。`,
        sources: randomSources.length > 0 ? randomSources : ['综合技术资讯']
      }
    }
  }

  // 处理从QA页面跳转到文章详情
  function jumpToSourceFromQA(sourceName: string) {
    console.log('尝试跳转到文章:', sourceName) // 调试信息

    // 保存当前QA状态，以便返回
    qaReturnContext.value = {
      fromQA: true,
      sessionId: currentChatSessionId.value
    }

    // 查找对应的文章
    for (const feedId in feedSummaries) {
      console.log(`在 ${feedId} 中查找文章...`) // 调试信息
      const summary = feedSummaries[feedId].find(s => {
        console.log(`检查文章: ${s.title}`) // 调试信息
        return s.title === sourceName
      })
      if (summary) {
        console.log('找到匹配文章:', summary.title) // 调试信息
        switchToDetailView(summary)
        showFeedbackMessage(`已找到相关文章: ${sourceName}`)
        return
      }
    }

    // 如果精确匹配失败，尝试模糊匹配
    console.log('精确匹配失败，尝试模糊匹配...') // 调试信息
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => {
        // 检查标题中是否包含参考来源的关键词
        const sourceKeywords = sourceName.split(/[\s\-：:]+/).filter(word => word.length > 1)
        return sourceKeywords.some(keyword =>
          s.title.toLowerCase().includes(keyword.toLowerCase())
        )
      })
      if (summary) {
        console.log('模糊匹配成功:', summary.title) // 调试信息
        switchToDetailView(summary)
        showFeedbackMessage(`已找到相关文章: ${summary.title}`)
        return
      }
    }

    // 如果还是没找到，显示所有可用文章供参考
    console.log('未找到匹配文章，返回摘要视图') // 调试信息
    switchToSummaryView()
    showFeedbackMessage(`未找到具体文章"${sourceName}"，请在列表中查看相关内容`)
  }

  // 从文章详情返回到QA聊天
  function returnToQAChat() {
    if (qaReturnContext.value) {
      switchToQAView()
      qaReturnContext.value = null
      showFeedbackMessage('已返回聊天界面')
    }
  }

  // 笔记管理
  function addNoteToSummary(summaryId: string, note: Note) {
    // 查找对应的摘要并添加笔记
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        if (!summary.notesList) {
          summary.notesList = []
        }
        summary.notesList.push({
          content: note.content,
          createdAt: note.createdAt
        })
        return true
      }
    }
    return false
  }

  function updateNotesForSummary(summaryId: string, notesList: Note[]) {
    // 查找对应的摘要并更新笔记列表
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        summary.notesList = notesList
        return true
      }
    }
    return false
  }

  // 标签管理
  function addTag(summaryId: string, tag: string) {
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        if (!summary.tags) {
          summary.tags = []
        }
        if (!summary.tags.includes(tag)) {
          summary.tags.push(tag)
        }
        return true
      }
    }
    return false
  }

  function removeTag(summaryId: string, tagToRemove: string) {
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary && summary.tags) {
        summary.tags = summary.tags.filter(tag => tag !== tagToRemove)
        return true
      }
    }
    return false
  }

  // Markdown 渲染
  function renderMarkdown(content: string) {
    return marked(content)
  }

  // QA 会话管理
  function createNewChatSession(initialQuestion: string = '') {
    const sessionId = `session-${Date.now()}`
    const sessionTitle = initialQuestion.length > 20
      ? initialQuestion.substring(0, 20) + '...'
      : initialQuestion || '新对话'

    const newSession = {
      id: sessionId,
      title: sessionTitle,
      createdAt: new Date(),
      updatedAt: new Date(),
      messages: []
    }

    // 如果有初始问题，直接添加
    if (initialQuestion.trim()) {
      newSession.messages.push({
        id: Date.now().toString(),
        type: 'user',
        content: initialQuestion,
        timestamp: new Date().toISOString()
      })
    }

    qaChatSessions.value.unshift(newSession) // 添加到开头
    currentChatSessionId.value = sessionId

    // 如果有初始问题，生成AI回答
    if (initialQuestion.trim()) {
      setTimeout(() => {
        const answer = generateAIAnswer(initialQuestion)
        addMessageToCurrentSession(answer.content, 'assistant', answer.sources)
      }, 1000)
    }

    return sessionId
  }

  function switchChatSession(sessionId: string) {
    if (qaChatSessions.value.find(s => s.id === sessionId)) {
      currentChatSessionId.value = sessionId
      return true
    }
    return false
  }

  function deleteChatSession(sessionId: string) {
    const index = qaChatSessions.value.findIndex(s => s.id === sessionId)
    if (index > -1) {
      qaChatSessions.value.splice(index, 1)

      // 如果删除的是当前会话，切换到最新的会话
      if (currentChatSessionId.value === sessionId) {
        if (qaChatSessions.value.length > 0) {
          currentChatSessionId.value = qaChatSessions.value[0].id
        } else {
          // 如果没有会话了，创建一个新的
          createNewChatSession()
        }
      }
      return true
    }
    return false
  }

  function addMessageToCurrentSession(content: string, type: 'user' | 'assistant', sources: string[] = []) {
    const session = qaChatSessions.value.find(s => s.id === currentChatSessionId.value)
    if (session) {
      const newMessage = {
        id: Date.now().toString(),
        type,
        content,
        sources: sources || [],
        timestamp: new Date().toISOString()
      }
      session.messages.push(newMessage)
      session.updatedAt = new Date()

      // 如果是用户消息且消息较短，更新会话标题
      if (type === 'user' && session.messages.length <= 2 && content.length <= 30) {
        session.title = content
      }

      return newMessage
    }
    return null
  }

  function askQuestionInCurrentSession(question: string) {
    // 添加用户问题到当前会话
    addMessageToCurrentSession(question, 'user')

    // 模拟 AI 回答
    setTimeout(() => {
      const answer = generateAIAnswer(question)
      addMessageToCurrentSession(answer.content, 'assistant', answer.sources)
    }, 1000)
  }

  function startNewChatFromSidebar(question: string) {
    // 从侧边栏开始新聊天
    const sessionId = createNewChatSession(question)
    switchToQAView()
    return sessionId
  }

  // 手动同步所有订阅源
  const manualSync = async (options: { includeAI?: boolean; sendEmail?: boolean } = {}) => {
    if (syncStatus.value.isRunning) return false

    syncStatus.value.isRunning = true
    syncStatus.value.progress = 0
    syncStatus.value.errors = []
    syncStatus.value.currentAction = '开始同步...'

    try {
      const steps = [
        { action: '连接到订阅源服务器...', duration: 1000 },
        { action: '获取最新文章列表...', duration: 2000 },
        { action: '解析文章内容...', duration: 1500 },
        { action: 'AI内容分析与总结...', duration: options.includeAI ? 3000 : 500 },
        { action: '更新本地数据...', duration: 1000 },
        { action: '发送邮件通知...', duration: options.sendEmail ? 1000 : 0 }
      ].filter(step => step.duration > 0)

      for (let i = 0; i < steps.length; i++) {
        const step = steps[i]
        syncStatus.value.currentAction = step.action

        await new Promise(resolve => setTimeout(resolve, step.duration))
        syncStatus.value.progress = Math.round(((i + 1) / steps.length) * 100)
      }

      syncStatus.value.lastSyncTime = new Date()

      // 模拟更新订阅源状态
      feeds.forEach(feed => {
        if (feed.status === 'loading') {
          feed.status = 'active'
          feed.lastUpdated = new Date()
        }
      })

      showFeedbackMessage('同步完成！已获取最新内容并生成AI总结。')
      return true
    } catch (error) {
      console.error('同步失败:', error)
      syncStatus.value.errors.push({
        feedId: 'general',
        feedName: '系统',
        error: '同步过程中发生未知错误',
        timestamp: new Date()
      })
      showFeedbackMessage('同步失败，请检查网络连接后重试。')
      return false
    } finally {
      syncStatus.value.isRunning = false
      syncStatus.value.currentAction = ''
    }
  }

  // 获取定时任务
  const getScheduledTask = (id: string) => {
    return scheduledTasks.value.find(task => task.id === id)
  }

  // 更新定时任务
  const updateScheduledTask = (taskId: string, updates: Partial<ScheduledTaskConfig>) => {
    const taskIndex = scheduledTasks.value.findIndex(task => task.id === taskId)
    if (taskIndex !== -1) {
      scheduledTasks.value[taskIndex] = { ...scheduledTasks.value[taskIndex], ...updates }

      // 如果启用了任务，计算下次执行时间
      if (updates.enabled === true) {
        const task = scheduledTasks.value[taskIndex]
        // 这里可以根据 cron 表达式计算下次执行时间
        // 简化处理：默认24小时后
        const now = new Date()
        task.nextRun = new Date(now.getTime() + 24 * 60 * 60 * 1000)
      }
    }
  }

  // 创建新的定时任务
  const createScheduledTask = (task: Omit<ScheduledTaskConfig, 'id'>) => {
    const newTask: ScheduledTaskConfig = {
      ...task,
      id: `task-${Date.now()}`
    }
    scheduledTasks.value.push(newTask)
    return newTask.id
  }

  // 删除定时任务
  const deleteScheduledTask = (taskId: string) => {
    const index = scheduledTasks.value.findIndex(task => task.id === taskId)
    if (index !== -1) {
      scheduledTasks.value.splice(index, 1)
    }
  }

  // 执行定时任务
  const executeScheduledTask = async (taskId: string) => {
    const task = getScheduledTask(taskId)
    if (!task || !task.enabled) return false

    console.log(`执行定时任务: ${task.name}`)

    // 执行同步
    const success = await manualSync({
      includeAI: task.aiSummaryEnabled,
      sendEmail: task.emailConfig.enabled
    })

    if (success) {
      task.lastRun = new Date()
      // 计算下次执行时间
      updateScheduledTask(taskId, { lastRun: new Date() })
    }

    return success
  }

  // 测试邮件配置
  const testEmailConfig = async (emailConfig: any) => {
    // 模拟邮件测试
    await new Promise(resolve => setTimeout(resolve, 2000))

    if (!emailConfig.recipientEmail) {
      throw new Error('收件人邮箱不能为空')
    }

    showFeedbackMessage('邮件配置测试成功！')
    return true
  }

  // 计算关联摘要
  const calculateRelatedSummaries = (currentSummaryId: string): RelatedSummary[] => {
    // 获取当前摘要
    let currentSummary: Summary | null = null
    for (const feedId in feedSummaries) {
      const found = feedSummaries[feedId].find(s => s.id === currentSummaryId)
      if (found) {
        currentSummary = found
        break
      }
    }

    if (!currentSummary) return []

    const related: RelatedSummary[] = []
    const currentTags = new Set(currentSummary.tags)

    // 遍历所有摘要，计算相似度
    for (const feedId in feedSummaries) {
      for (const summary of feedSummaries[feedId]) {
        if (summary.id === currentSummaryId) continue

        let relevanceScore = 0
        const sharedTags: string[] = []
        let relationType: RelatedSummary['relationType'] = 'content'

        // 1. 标签相似度计算（权重 40%）
        const summaryTags = new Set(summary.tags)
        for (const tag of summary.tags) {
          if (currentTags.has(tag)) {
            sharedTags.push(tag)
            relevanceScore += 0.4 / currentSummary.tags.length
          }
        }

        // 2. 内容相似度计算（简化版 - 基于关键词匹配，权重 40%）
        const currentKeywords = extractKeywords(currentSummary.title + ' ' + currentSummary.content)
        const summaryKeywords = extractKeywords(summary.title + ' ' + summary.content)
        const keywordMatches = currentKeywords.filter(kw => summaryKeywords.includes(kw))
        relevanceScore += (keywordMatches.length / Math.max(currentKeywords.length, 1)) * 0.4

        // 3. 时间相关性（权重 20%）
        const currentDate = new Date(currentSummary.publishedAt)
        const summaryDate = new Date(summary.publishedAt)
        const daysDiff = Math.abs(currentDate.getTime() - summaryDate.getTime()) / (1000 * 60 * 60 * 24)
        const timeRelevance = Math.max(0, 1 - daysDiff / 30) // 30天内的内容有时间相关性
        relevanceScore += timeRelevance * 0.2

        // 确定关联类型
        if (sharedTags.length > 0) {
          relationType = 'tag'
        } else if (keywordMatches.length > 2) {
          relationType = 'content'
        } else if (timeRelevance > 0.7) {
          relationType = 'temporal'
        } else {
          relationType = 'source'
        }

        // 只保留相关度大于0.3的摘要
        if (relevanceScore > 0.3) {
          related.push({
            id: summary.id,
            title: summary.title,
            relevanceScore: Math.min(relevanceScore, 1), // 确保不超过1
            relationType,
            sharedTags: sharedTags.length > 0 ? sharedTags : undefined,
            publishedAt: summary.publishedAt,
            excerpt: summary.content.substring(0, 100) + '...'
          })
        }
      }
    }

    // 按相关度排序，返回前5个
    return related
      .sort((a, b) => b.relevanceScore - a.relevanceScore)
      .slice(0, 5)
  }

  // 提取关键词的简化函数
  const extractKeywords = (text: string): string[] => {
    // 简化的关键词提取：分词并过滤停用词
    const stopWords = new Set(['的', '是', '在', '有', '和', '了', '也', '就', '都', '而', '及', '与', '或', '但', '不', '没', '很', '更', '最', '这', '那', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十'])

    return text
      .toLowerCase()
      .replace(/[^\w\u4e00-\u9fff\s]/g, ' ') // 保留中英文字符
      .split(/\s+/)
      .filter(word => word.length > 1 && !stopWords.has(word))
      .slice(0, 20) // 限制关键词数量
  }


  return {
    // 状态
    currentView,
    currentDetail,
    currentQAQuery,
    selectedFeed,
    feedbackMessage,
    showFeedback,
    feeds: feedsWithCount, // 使用带计数的订阅源
    summaries,
    feedSummaries,
    // QA 多会话状态
    qaChatSessions,
    currentChatSessionId,
    currentChatMessages,
    currentChatSession,
    qaReturnContext,
    // 同步状态
    syncStatus,
    scheduledTasks,

    // 方法
    formatTimeAgo,
    showFeedbackMessage,
    selectFeed,
    addFeed,
    switchToSummaryView,
    switchToQAView,
    switchToDetailView,
    addNoteToSummary,
    updateNotesForSummary,
    addTag,
    removeTag,
    renderMarkdown,
    // QA 多会话方法
    createNewChatSession,
    switchChatSession,
    deleteChatSession,
    addMessageToCurrentSession,
    askQuestionInCurrentSession,
    startNewChatFromSidebar,
    jumpToSourceFromQA,
    returnToQAChat,
    // 同步和定时任务方法
    manualSync,
    getScheduledTask,
    updateScheduledTask,
    createScheduledTask,
    deleteScheduledTask,
    executeScheduledTask,
    testEmailConfig,
    // 关联摘要计算
    calculateRelatedSummaries
  }
})
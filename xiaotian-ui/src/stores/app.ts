import type {
    ChatMessage,
    ChatSession,
    Feed,
    NewFeedData,
    Note,
    QAReturnContext,
    Summary,
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
        id: 1,
        title: '大型语言模型在代码生成领域的最新进展',
        source: 'Hacker News',
        date: '2025年7月8日',
        content: '近期研究表明，结合了静态分析工具的 LLM 在代码生成任务上表现出了惊人的准确性。模型不再是盲目生成代码，而是能够理解代码的上下文、依赖关系和潜在的空指针风险...',
        fullContent: '近期研究表明，结合了静态分析工具的 LLM 在代码生成任务上表现出了惊人的准确性。模型不再是盲目生成代码，而是能够理解代码的上下文、依赖关系和潜在的空指针风险。这种技术的突破为自动化编程带来了新的可能性，同时也为代码质量的提升提供了新的工具。更重要的是，这种结合静态分析的方法能够在编码阶段就发现潜在的bug，大大提高了代码的可靠性。',
        link: 'https://news.ycombinator.com/item?id=123456',
        tags: ['AI', '代码生成', '静态分析', 'LLM'],
        notesList: [
          { content: '这个技术可能会改变整个编程行业', createdAt: '2025-01-15 10:30' },
          { content: '需要关注对传统开发流程的影响', createdAt: '2025-01-15 11:00' }
        ]
      },
      {
        id: 2,
        title: 'WebAssembly 在浏览器性能优化中的实际应用',
        source: 'Hacker News',
        date: '2025年7月7日',
        content: 'WebAssembly (WASM) 作为新一代 Web 技术，在实际应用中展现了强大的性能潜力。本文通过多个真实案例，展示了 WASM 如何在图像处理、游戏引擎、加密算法等场景中显著提升性能...',
        fullContent: 'WebAssembly (WASM) 作为新一代 Web 技术，在实际应用中展现了强大的性能潜力。本文通过多个真实案例，展示了 WASM 如何在图像处理、游戏引擎、加密算法等场景中显著提升性能。特别是在计算密集型任务中，WASM 的性能甚至接近原生应用。随着工具链的不断完善，WASM 正在成为构建高性能 Web 应用的重要选择。',
        link: 'https://news.ycombinator.com/item?id=789012',
        tags: ['WebAssembly', '性能优化', '浏览器技术'],
        notesList: []
      }
    ],
    'rust-blog': [
      {
        id: 3,
        title: 'Rust 1.75 版本发布：异步编程的重大改进',
        source: 'Rust Blog',
        date: '2025年7月6日',
        content: 'Rust 1.75 版本带来了期待已久的异步编程改进，包括更好的错误处理、性能优化和开发体验提升。新版本的 async/await 语法更加直观，同时引入了更强大的并发原语...',
        fullContent: 'Rust 1.75 版本带来了期待已久的异步编程改进，包括更好的错误处理、性能优化和开发体验提升。新版本的 async/await 语法更加直观，同时引入了更强大的并发原语。这些改进使得 Rust 在构建高性能异步应用方面更加强大，特别是在网络服务和系统编程领域。',
        link: 'https://blog.rust-lang.org/2025/01/15/Rust-1.75.0.html',
        tags: ['Rust', '异步编程', '版本发布'],
        notesList: [
          { content: '需要测试现有代码的兼容性', createdAt: '2025-01-15 14:20' }
        ]
      }
    ],
    'vue-blog': [
      {
        id: 4,
        title: 'Vue 3.5 带来的组合式 API 优化',
        source: 'Vue.js Blog',
        date: '2025年7月5日',
        content: 'Vue 3.5 版本进一步优化了组合式 API 的性能和易用性。新增的响应式语法糖让代码更加简洁，同时改进的类型推导提供了更好的 TypeScript 支持...',
        fullContent: 'Vue 3.5 版本进一步优化了组合式 API 的性能和易用性。新增的响应式语法糖让代码更加简洁，同时改进的类型推导提供了更好的 TypeScript 支持。这些改进使得 Vue.js 在大型项目中的表现更加出色，开发体验也得到了显著提升。',
        link: 'https://blog.vuejs.org/posts/vue-3-5.html',
        tags: ['Vue.js', '组合式API', 'TypeScript'],
        notesList: []
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
    const diff = now - new Date(date)
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
    const newFeed = {
      name: feedData.name.trim() || new URL(feedData.feedUrl).hostname,
      description: feedData.description.trim() || '新添加的订阅源',
      feedUrl: feedData.feedUrl.trim(),
      icon: '📰', // 默认图标
      id: `feed-${Date.now()}`,
      category: feedData.category || '其他',
      lastUpdated: new Date(),
      status: 'loading'
    }

    // 添加到列表
    feeds.push(newFeed)
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
      view: 'qa',
      timestamp: new Date()
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
    returnToQAChat
  }
})
import { marked } from 'marked'
import { defineStore } from 'pinia'
import { computed, reactive, ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  // 当前视图状态
  const currentView = ref('summary') // 'summary', 'qa', 'detail'
  const currentDetail = ref(null)
  const currentQAQuery = ref('')

  // 当前选中的订阅源
  const selectedFeed = ref('Hacker News')

  // 反馈消息
  const feedbackMessage = ref('')
  const showFeedback = ref(false)

  // 订阅源数据 - 移除硬编码的count，改为动态计算
  const feeds = reactive([
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
  const feedSummaries = reactive({
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
  function formatTimeAgo(date) {
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
  function showFeedbackMessage(message, duration = 3000) {
    feedbackMessage.value = message
    showFeedback.value = true
    setTimeout(() => {
      showFeedback.value = false
    }, duration)
  }

  // 选择订阅源
  function selectFeed(feedName) {
    console.log('选择订阅源:', feedName)
    selectedFeed.value = feedName
  }

  // 添加新的订阅源
  function addFeed(feedData) {
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

  function switchToQAView(query = '') {
    currentView.value = 'qa'
    currentQAQuery.value = query
  }

  function switchToDetailView(summary) {
    currentView.value = 'detail'
    currentDetail.value = summary
  }

  // 笔记管理
  function addNoteToSummary(summaryId, note) {
    // 查找对应的摘要并添加笔记
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        if (!summary.notesList) {
          summary.notesList = []
        }
        summary.notesList.push({
          content: note,
          createdAt: new Date().toLocaleString()
        })
        return true
      }
    }
    return false
  }

  function updateNotesForSummary(summaryId, notesList) {
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
  function addTag(summaryId, tag) {
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

  function removeTag(summaryId, tagToRemove) {
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
  function renderMarkdown(content) {
    return marked(content)
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
    renderMarkdown
  }
})
import { marked } from 'marked'
import { defineStore } from 'pinia'
import { computed, reactive, ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  // 当前活跃的视图
  const currentView = ref('summary') // 'summary', 'qa', 'detail'

  // 当前选中的订阅源
  const selectedFeed = ref('Hacker News')

  // 反馈消息
  const feedbackMessage = ref('')
  const showFeedback = ref(false)

  // RSS 订阅源列表
  const feeds = reactive([
    { name: 'Hacker News', count: 12, id: 'hacker-news' },
    { name: 'Rust Blog', count: 5, id: 'rust-blog' },
    { name: 'Vue.js Blog', count: 8, id: 'vue-blog' }
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
        notes: '',
        notesList: [
          {
            content: '这篇文章很有启发性，静态分析结合LLM的思路很新颖。',
            createdAt: '2025年7月8日 14:30'
          }
        ],
        tags: ['AI', '代码生成', 'LLM', '静态分析']
      },
      {
        id: 2,
        title: 'WebAssembly在前端性能优化中的实践',
        source: 'Hacker News',
        date: '2025年7月7日',
        content: 'WebAssembly 正在成为前端性能优化的重要工具。通过将计算密集型任务移植到 WASM，可以获得接近原生的性能表现...',
        fullContent: 'WebAssembly 正在成为前端性能优化的重要工具。通过将计算密集型任务移植到 WASM，可以获得接近原生的性能表现。特别是在图像处理、数据分析和游戏开发等领域，WASM 的优势尤为明显。本文将分享一些实际项目中使用 WASM 优化性能的经验和最佳实践。',
        link: 'https://news.ycombinator.com/item?id=123457',
        notes: '',
        notesList: [],
        tags: ['WebAssembly', '性能优化', '前端']
      }
    ],
    'rust-blog': [
      {
        id: 3,
        title: 'Rust 1.79.0 发布：关键性能优化与新特性',
        source: 'Rust Blog',
        date: '2025年7月8日',
        content: 'Rust 1.79.0 版本正式发布。本次更新的核心是针对编译器和标准库的性能优化，编译速度在部分场景下提升了 15%。同时，引入了新的 `#[must_use]` 属性扩展...',
        fullContent: 'Rust 1.79.0 版本正式发布。本次更新的核心是针对编译器和标准库的性能优化，编译速度在部分场景下提升了 15%。同时，引入了新的 `#[must_use]` 属性扩展，帮助开发者更好地处理返回值。这次更新还包括了对异步编程的进一步优化，特别是在 tokio 生态系统的集成方面有了显著改进。此外，标准库新增了多个实用的 API，进一步提升了开发体验。',
        link: 'https://blog.rust-lang.org/2025/07/08/Rust-1.79.0.html',
        notes: '',
        notesList: [],
        tags: ['Rust', '性能优化', '编译器', '异步编程']
      }
    ],
    'vue-blog': [
      {
        id: 4,
        title: 'Vue 3.5 新特性深度解析',
        source: 'Vue.js Blog',
        date: '2025年7月6日',
        content: 'Vue 3.5 带来了多项重要更新，包括改进的响应式系统、更好的 TypeScript 支持和新的组合式 API 特性...',
        fullContent: 'Vue 3.5 带来了多项重要更新，包括改进的响应式系统、更好的 TypeScript 支持和新的组合式 API 特性。其中最值得关注的是新的 defineModel 宏，它简化了自定义组件的 v-model 实现。此外，Suspense 组件也得到了显著改进，现在支持更复杂的异步场景。',
        link: 'https://blog.vuejs.org/posts/vue-3-5.html',
        notes: '',
        notesList: [],
        tags: ['Vue.js', 'TypeScript', '响应式', 'Composition API']
      }
    ]
  })

  // 计算当前选中订阅源的摘要列表
  const summaries = computed(() => {
    const feedId = feeds.find(f => f.name === selectedFeed.value)?.id
    return feedSummaries[feedId] || []
  })

  // 当前查看的详情
  const currentDetail = ref(null)

  // 问答相关状态
  const qaQuestion = ref('')
  const qaAnswer = ref({
    content: '根据你的知识库，Rust 在 1.79.0 版本中发布了重要的性能更新，主要体现在编译器速度提升和标准库优化两个方面。',
    sources: ['Rust 1.79.0 发布：关键性能优化与新特性']
  })

  // 显示反馈消息
  function showFeedbackMessage(message, duration = 3000) {
    feedbackMessage.value = message
    showFeedback.value = true
    setTimeout(() => {
      showFeedback.value = false
    }, duration)
  }

  // 视图切换方法
  function switchToSummaryView() {
    currentView.value = 'summary'
  }

  function switchToQAView(question) {
    if (question) {
      qaQuestion.value = question
    }
    currentView.value = 'qa'
  }

  function switchToDetailView(summary) {
    console.log('Store: 切换到详情视图', summary) // 调试日志
    console.log('Store: 当前视图状态:', currentView.value) // 调试日志
    currentDetail.value = summary
    currentView.value = 'detail'
    console.log('Store: 新视图状态:', currentView.value) // 调试日志
    console.log('Store: 详情数据:', currentDetail.value) // 调试日志
  }

  // 选择订阅源
  function selectFeed(feedName) {
    selectedFeed.value = feedName
  }

  // 添加RSS源
  function addFeed(feedUrl) {
    if (feedUrl && feedUrl.trim()) {
      try {
        const url = new URL(feedUrl)
        const feedName = url.hostname
        const feedId = feedName.toLowerCase().replace(/\./g, '-')

        // 检查是否已存在
        if (feeds.find(f => f.name === feedName)) {
          showFeedbackMessage(`订阅源 "${feedName}" 已存在`, 2000)
          return false
        }

        // 添加新的订阅源
        feeds.push({
          name: feedName,
          count: 0,
          id: feedId
        })

        // 初始化空的摘要列表
        feedSummaries[feedId] = []

        showFeedbackMessage(`成功添加订阅源 "${feedName}"`, 2000)
        return true
      } catch (error) {
        showFeedbackMessage('请输入有效的 URL', 2000)
        return false
      }
    }
    return false
  }

  // 保存笔记（支持markdown）
  function saveNotes(summaryId, notes) {
    // 在所有订阅源中查找对应的摘要
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        summary.notes = notes
        return true
      }
    }
    return false
  }

  // 添加标签
  function addTag(summaryId, tag) {
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary && !summary.tags.includes(tag)) {
        summary.tags.push(tag)
        return true
      }
    }
    return false
  }

  // 移除标签
  function removeTag(summaryId, tag) {
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        const index = summary.tags.indexOf(tag)
        if (index > -1) {
          summary.tags.splice(index, 1)
          return true
        }
      }
    }
    return false
  }

  // 渲染markdown
  function renderMarkdown(text) {
    return marked(text)
  }

  // 为摘要添加笔记
  function addNoteToSummary(summaryId, note) {
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        if (!summary.notesList) {
          summary.notesList = []
        }
        summary.notesList.push(note)
        return true
      }
    }
    return false
  }

  // 更新摘要的笔记列表
  function updateNotesForSummary(summaryId, notesList) {
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => s.id === summaryId)
      if (summary) {
        summary.notesList = notesList
        return true
      }
    }
    return false
  }

  return {
    // 状态
    currentView,
    selectedFeed,
    feeds,
    summaries,
    currentDetail,
    qaQuestion,
    qaAnswer,
    feedbackMessage,
    showFeedback,

    // 方法
    switchToSummaryView,
    switchToQAView,
    switchToDetailView,
    selectFeed,
    addFeed,
    saveNotes,
    addTag,
    removeTag,
    renderMarkdown,
    showFeedbackMessage,
    addNoteToSummary,
    updateNotesForSummary
  }
})
import { defineStore } from 'pinia'
import { reactive, ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  // 当前活跃的视图
  const currentView = ref('summary') // 'summary', 'qa', 'detail'

  // RSS 订阅源列表
  const feeds = reactive([
    { name: 'Hacker News', count: 12 }
  ])

  // 摘要文章列表
  const summaries = reactive([
    {
      id: 1,
      title: 'Rust 1.79.0 发布：关键性能优化与新特性',
      source: 'Rust Blog',
      date: '2025年7月8日',
      content: 'Rust 1.79.0 版本正式发布。本次更新的核心是针对编译器和标准库的性能优化，编译速度在部分场景下提升了 15%。同时，引入了新的 `#[must_use]` 属性扩展...',
      fullContent: 'Rust 1.79.0 版本正式发布。本次更新的核心是针对编译器和标准库的性能优化，编译速度在部分场景下提升了 15%。同时，引入了新的 `#[must_use]` 属性扩展，帮助开发者更好地处理返回值。这次更新还包括了对异步编程的进一步优化，特别是在 tokio 生态系统的集成方面有了显著改进。',
      link: '#',
      notes: ''
    },
    {
      id: 2,
      title: '大型语言模型在代码生成领域的最新进展',
      source: 'Hacker News',
      date: '2025年7月8日',
      content: '近期研究表明，结合了静态分析工具的 LLM 在代码生成任务上表现出了惊人的准确性。模型不再是盲目生成代码，而是能够理解代码的上下文、依赖关系和潜在的空指针风险...',
      fullContent: '近期研究表明，结合了静态分析工具的 LLM 在代码生成任务上表现出了惊人的准确性。模型不再是盲目生成代码，而是能够理解代码的上下文、依赖关系和潜在的空指针风险。这种技术的突破为自动化编程带来了新的可能性，同时也为代码质量的提升提供了新的工具。',
      link: '#',
      notes: ''
    }
  ])

  // 当前查看的详情
  const currentDetail = ref(null)

  // 问答相关状态
  const qaQuestion = ref('')
  const qaAnswer = ref({
    content: '根据你的知识库，Rust 在 1.79.0 版本中发布了重要的性能更新，主要体现在编译器速度提升和标准库优化两个方面。',
    sources: ['Rust 1.79.0 发布：关键性能优化与新特性']
  })

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
    currentDetail.value = summary
    currentView.value = 'detail'
  }

  // 添加RSS源
  function addFeed(feedUrl) {
    if (feedUrl && feedUrl.trim()) {
      feeds.push({
        name: new URL(feedUrl).hostname,
        count: 0
      })
    }
  }

  // 保存笔记
  function saveNotes(summaryId, notes) {
    const summary = summaries.find(s => s.id === summaryId)
    if (summary) {
      summary.notes = notes
    }
  }

  return {
    // 状态
    currentView,
    feeds,
    summaries,
    currentDetail,
    qaQuestion,
    qaAnswer,

    // 方法
    switchToSummaryView,
    switchToQAView,
    switchToDetailView,
    addFeed,
    saveNotes
  }
})
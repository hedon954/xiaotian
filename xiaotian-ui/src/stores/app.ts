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
import { useApiStore } from './api'

export const useAppStore = defineStore('app', () => {
  // 获取API store实例
  const apiStore = useApiStore()

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


  const scheduledTasks = ref<ScheduledTaskConfig[]>([])


  const qaChatSessions = ref<ChatSession[]>([])


  const currentChatSessionId = ref<number>(0)

  // 记住从QA跳转前的状态，用于返回
  const qaReturnContext = ref<QAReturnContext | null>(null)


  const currentChatMessages = computed<ChatMessage[]>(() => {
    const apiSessions = apiStore.chatSessionsCache
    if (apiSessions.length > 0) {
      const session = apiSessions.find(s => s.id === currentChatSessionId.value)
      if (session && session.messages) {
        return session.messages
      }
    }

    const session = qaChatSessions.value.find(s => s.id === currentChatSessionId.value)
    return session ? session.messages : []
  })


  const currentChatSession = computed<ChatSession | undefined>(() => {
    const apiSessions = apiStore.chatSessionsCache
    if (apiSessions.length > 0) {
      const session = apiSessions.find(s => s.id === currentChatSessionId.value)
      if (session) {
        return session
      }
    }

    return qaChatSessions.value.find(s => s.id === currentChatSessionId.value)
  })


  const feeds = reactive<Feed[]>([])


  const feedSummaries = reactive<{ [key: number]: Summary[] }>({})

  // 动态计算每个订阅源的文章数量
  const feedsWithCount = computed(() => {
    return feeds.map(feed => ({
      ...feed,
      count: feedSummaries[feed.id]?.length || 0
    }))
  })


  const summaries = computed(() => {
    const apiSummaries = apiStore.summariesCache
    if (apiSummaries.length > 0) {
      const selectedFeedData = apiStore.feedsCache.find(f => f.name === selectedFeed.value)
      if (selectedFeedData) {
        return apiSummaries.filter(s => s.feedId === selectedFeedData.id)
      }
    }

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

  // 添加新的订阅源 - 现在使用API服务
  async function addFeed(feedData: NewFeedData) {
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

    try {
      // 使用API服务创建订阅源
      const result = await apiStore.createFeed({
        name: feedData.name.trim() || new URL(feedData.feedUrl).hostname,
        type: 'rss',
        feedUrl: feedData.feedUrl.trim(),
        description: feedData.description?.trim() || '新添加的订阅源',
        category: feedData.category || '其他',
        icon: '📰'
      })

      if (result) {
        // 同步更新本地数据（用于向后兼容）
        const newFeed: Feed = {
          name: result.name,
          description: result.description,
          feedUrl: result.feedUrl,
          icon: result.icon,
          id: result.id,
          category: result.category,
          lastUpdated: result.lastUpdated ? new Date(result.lastUpdated) : new Date(),
          status: result.status as 'active' | 'loading' | 'error'
        }

        feeds.unshift(newFeed)
        feedSummaries[newFeed.id] = []

        showFeedbackMessage('订阅源添加成功！')
        return true
      } else {
        showFeedbackMessage(apiStore.error || '添加订阅源失败')
        return false
      }
    } catch (error) {
      console.error('添加订阅源失败:', error)
      showFeedbackMessage('添加订阅源失败，请稍后重试')
      return false
    }
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
  function jumpToSourceFromQA(summaryId: string | number) {
    console.log('尝试跳转到文章:', summaryId, typeof summaryId) // 调试信息

    // 保存当前QA状态，以便返回
    qaReturnContext.value = {
      fromQA: true,
      sessionId: currentChatSessionId.value
    }

    // 标准化summaryId为数字类型进行比较
    const targetId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
    console.log('标准化后的ID:', targetId) // 调试信息

    // 优先从API缓存中查找
    const apiSummaries = apiStore.summariesCache
    if (apiSummaries.length > 0) {
      const summary = apiSummaries.find(s => s.id === targetId)
      if (summary) {
        console.log('从API缓存找到匹配文章:', summary.title) // 调试信息
        switchToDetailView(summary)
        showFeedbackMessage(`已找到相关文章: ${summary.title}`)
        return
      }
    }

    // 查找对应的文章（兼容旧数据结构）
    for (const feedId in feedSummaries) {
      console.log(`在 ${feedId} 中查找文章...`) // 调试信息
      const summary = feedSummaries[feedId].find(s => {
        console.log(`检查文章: ${s.title}, ID: ${s.id}`) // 调试信息
        return Number(s.id) === targetId
      })
      if (summary) {
        console.log('找到匹配文章:', summary.title) // 调试信息
        switchToDetailView(summary)
        showFeedbackMessage(`已找到相关文章: ${summary.title}`)
        return
      }
    }


    // 如果还是没找到，显示所有可用文章供参考
    console.log(`未找到匹配文章${summaryId}，返回摘要视图`) // 调试信息
    switchToSummaryView()
    showFeedbackMessage(`未找到具体文章"${summaryId}"，请在列表中查看相关内容`)
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
  function addNoteToSummary(summaryId: string | number, note: Note) {
    // 标准化ID为数字类型
    const targetId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
    // 查找对应的摘要并添加笔记
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => Number(s.id) === targetId)
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

  function updateNotesForSummary(summaryId: string | number, notesList: Note[]) {
    // 标准化ID为数字类型
    const targetId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
    // 查找对应的摘要并更新笔记列表
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => Number(s.id) === targetId)
      if (summary) {
        summary.notesList = notesList
        return true
      }
    }
    return false
  }

  // 标签管理
  function addTag(summaryId: string | number, tag: string) {
    // 标准化ID为数字类型
    const targetId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => Number(s.id) === targetId)
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

  function removeTag(summaryId: string | number, tagToRemove: string) {
    // 标准化ID为数字类型
    const targetId = typeof summaryId === 'string' ? parseInt(summaryId, 10) : summaryId
    for (const feedId in feedSummaries) {
      const summary = feedSummaries[feedId].find(s => Number(s.id) === targetId)
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
    const sessionTitle = initialQuestion.length > 20
      ? initialQuestion.substring(0, 20) + '...'
      : initialQuestion || '新对话'

    const newSession = {
      id: Date.now(), // 添加唯一ID
      title: sessionTitle,
      createdAt: new Date(),
      updatedAt: new Date(),
      messages: [],
      messageCount: 0
    }

    // 如果有初始问题，直接添加
    if (initialQuestion.trim()) {
      newSession.messages.push({
        id: Date.now().toString(),
        type: 'user',
        content: initialQuestion,
        timestamp: new Date().toISOString()
      })
      newSession.messageCount = 1
    }

    // 添加到会话列表
    qaChatSessions.value.unshift(newSession)
    currentChatSessionId.value = newSession.id

    console.log('🆕 App Store: 创建本地会话', {
      sessionId: newSession.id,
      title: newSession.title,
      totalSessions: qaChatSessions.value.length
    })

    // 如果有初始问题，生成AI回答
    if (initialQuestion.trim()) {
      setTimeout(() => {
        const answer = generateAIAnswer(initialQuestion)
        addMessageToCurrentSession(answer.content, 'assistant', answer.sources)
      }, 1000)
    }

    return newSession.id
  }

  function switchChatSession(sessionId: number) {
    console.log('🔄 App Store: 切换会话', {
      sessionId,
      apiSessions: apiStore.chatSessionsCache.length,
      localSessions: qaChatSessions.value.length
    })

    // 优先检查API Store的会话
    if (apiStore.chatSessionsCache.find(s => s.id === sessionId)) {
      currentChatSessionId.value = sessionId
      console.log('✅ 切换到API会话:', sessionId)
      return true
    }

    // 降级到本地会话
    if (qaChatSessions.value.find(s => s.id === sessionId)) {
      currentChatSessionId.value = sessionId
      console.log('✅ 切换到本地会话:', sessionId)
      return true
    }

    console.warn('⚠️ 会话不存在:', sessionId)
    return false
  }

  function deleteChatSession(sessionId: number) {
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

  function addMessageToCurrentSession(content: string, type: 'user' | 'assistant', sources: { summaryId: number; summaryTitle: string }[] = []) {
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

      console.log('💬 App Store: 添加消息到会话', {
        sessionId: session.id,
        type,
      })

      return newMessage
    }

    console.warn('⚠️ App Store: 当前会话不存在', currentChatSessionId.value)
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

  // 手动同步所有订阅源 - 现在使用API服务
  const manualSync = async (options: { includeAI?: boolean; sendEmail?: boolean } = {}) => {
    if (syncStatus.value.isRunning) return false

    syncStatus.value.isRunning = true
    syncStatus.value.progress = 0
    syncStatus.value.errors = []
    syncStatus.value.currentAction = '开始同步...'

    try {
      // 尝试使用API服务进行同步
      const result = await apiStore.manualSync({ options })

      if (result) {
        // 模拟进度更新（真实API会通过WebSocket或轮询来更新进度）
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

        // 重新加载订阅源数据
        await apiStore.loadFeeds()
        await apiStore.loadSummaries()

        showFeedbackMessage('同步完成！已获取最新内容并生成AI总结。')
        return true
      } else {
        throw new Error('同步启动失败')
      }
    } catch (error) {
      console.error('同步失败:', error)
      syncStatus.value.errors.push({
        feedId: 'general',
        feedName: '系统',
        error: apiStore.error || '同步过程中发生未知错误',
        timestamp: new Date()
      })
      showFeedbackMessage(apiStore.error || '同步失败，请检查网络连接后重试。')
      return false
    } finally {
      syncStatus.value.isRunning = false
      syncStatus.value.currentAction = ''
    }
  }


  const getScheduledTask = (id: string) => {
    const apiTasks = apiStore.tasksCache
    if (apiTasks.length > 0) {
      const task = apiTasks.find(t => t.id === id)
      if (task) {
        return task
      }
    }

    return scheduledTasks.value.find(task => task.id === id)
  }


  const updateScheduledTask = async (taskId: string, updates: Partial<ScheduledTaskConfig>) => {
    try {
      const apiUpdates: any = {}

      if (updates.name !== undefined) apiUpdates.name = updates.name
      if (updates.enabled !== undefined) apiUpdates.enabled = updates.enabled
      if (updates.cronExpression !== undefined) apiUpdates.cronExpression = updates.cronExpression
      if (updates.emailConfig !== undefined) {
        apiUpdates.emailConfig = {
          enabled: updates.emailConfig.enabled,
          recipientEmails: updates.emailConfig.recipientEmails,
          senderName: updates.emailConfig.senderName || '小天AI助手'
        }
      }
      if (updates.selectedFeeds !== undefined) apiUpdates.selectedFeeds = updates.selectedFeeds
      if (updates.aiSummaryEnabled !== undefined) apiUpdates.aiSummaryEnabled = updates.aiSummaryEnabled
      if (updates.summaryLength !== undefined) apiUpdates.summaryLength = updates.summaryLength

      await apiStore.updateTask(taskId, apiUpdates)
    } catch (error) {
      // 回退到本地更新
      const taskIndex = scheduledTasks.value.findIndex(task => task.id === taskId)
      if (taskIndex !== -1) {
        scheduledTasks.value[taskIndex] = { ...scheduledTasks.value[taskIndex], ...updates }

        // 如果启用了任务，计算下次执行时间
        if (updates.enabled === true) {
          const task = scheduledTasks.value[taskIndex]
          const now = new Date()
          task.nextRun = new Date(now.getTime() + 24 * 60 * 60 * 1000)
        }
      }
    }
  }

  // 创建新的定时任务 - 现在使用 API Store
  const createScheduledTask = async (task: Omit<ScheduledTaskConfig, 'id'>) => {
    try {
      const result = await apiStore.createTask({
        name: task.name,
        cronExpression: task.cronExpression,
        emailConfig: {
          enabled: task.emailConfig.enabled,
          recipientEmails: task.emailConfig.recipientEmails,
          senderName: task.emailConfig.senderName || '小天AI助手'
        },
        selectedFeeds: task.selectedFeeds,
        aiSummaryEnabled: task.aiSummaryEnabled,
        summaryLength: task.summaryLength,
        enabled: task.enabled
      })
      return result?.id
    } catch (error) {
      // 回退到本地创建
      const newTask: ScheduledTaskConfig = {
        ...task,
        id: `task-${Date.now()}`
      }
      scheduledTasks.value.push(newTask)
      return newTask.id
    }
  }

  // 删除定时任务 - 现在使用 API Store
  const deleteScheduledTask = async (taskId: string) => {
    try {
      await apiStore.deleteTask(taskId)
    } catch (error) {
      // 回退到本地删除
      const index = scheduledTasks.value.findIndex(task => task.id === taskId)
      if (index !== -1) {
        scheduledTasks.value.splice(index, 1)
      }
    }
  }

  // 执行定时任务 - 现在使用 API Store
  const executeScheduledTask = async (taskId: string) => {
    try {
      const result = await apiStore.executeTask(taskId)
      if (result) {
        console.log(`任务执行已启动: ${result.executionId}`)
        return true
      }
    } catch (error) {
      console.error('执行定时任务失败:', error)
    }

    // 回退到原有逻辑
    const task = getScheduledTask(taskId)
    if (!task || !task.enabled) return false

    console.log(`执行定时任务: ${task.name}`)

    const success = await manualSync({
      includeAI: task.aiSummaryEnabled,
      sendEmail: task.emailConfig.enabled
    })

    if (success) {
      await updateScheduledTask(taskId, { lastRun: new Date() })
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
  const calculateRelatedSummaries = (currentSummaryId: string | number): RelatedSummary[] => {
    // 标准化ID为数字类型
    const targetId = typeof currentSummaryId === 'string' ? parseInt(currentSummaryId, 10) : currentSummaryId

    // 获取当前摘要
    let currentSummary: Summary | null = null

    // 优先从API缓存中查找
    const apiSummaries = apiStore.summariesCache
    if (apiSummaries.length > 0) {
      currentSummary = apiSummaries.find(s => s.id === targetId) || null
    }

    // 如果API缓存中没有，则在feedSummaries中查找（兼容旧数据）
    if (!currentSummary) {
      for (const feedId in feedSummaries) {
        const found = feedSummaries[feedId].find(s => Number(s.id) === targetId)
        if (found) {
          currentSummary = found
          break
        }
      }
    }

    if (!currentSummary) return []

    const related: RelatedSummary[] = []
    const currentTags = new Set(currentSummary.tags)

    // 遍历所有摘要，计算相似度
    const allSummaries = apiSummaries.length > 0 ? apiSummaries :
      Object.values(feedSummaries).flat()

    for (const summary of allSummaries) {
      if (summary.id === targetId) continue

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
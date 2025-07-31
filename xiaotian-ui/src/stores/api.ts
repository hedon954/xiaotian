// API Store - 管理所有API状态和缓存
import apiService from '@/services'
import type {
  ApiChatSession,
  ApiFeed,
  ApiScheduledTask,
  ApiSummary,
  ChatSessionListParams,
  CreateChatSessionRequest,
  CreateFeedRequest,
  CreateScheduledTaskRequest,
  FeedListParams,
  ManualSyncRequest,
  SendMessageRequest,
  SummaryListParams,
  UpdateChatSessionRequest,
  UpdateFeedRequest,
  UpdateScheduledTaskRequest
} from '@/types/api'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useApiStore = defineStore('api', () => {
  // 缓存数据
  const feedsCache = ref<ApiFeed[]>([])
  const summariesCache = ref<ApiSummary[]>([])
  const chatSessionsCache = ref<ApiChatSession[]>([])
  const tasksCache = ref<ApiScheduledTask[]>([])

  // 加载状态
  const isLoading = ref(false)
  const error = ref<string>('')

  // 清除错误
  function clearError() {
    error.value = ''
  }

  // 错误处理
  function handleError(err: any) {
    error.value = err?.message || '操作失败'
    console.error('API Error:', err)
  }

  // 订阅源管理
  async function loadFeeds(params?: FeedListParams) {
    try {
      isLoading.value = true
      clearError()
      const response = await apiService.feeds.getFeeds(params)
      feedsCache.value = response.items
      return response
    } catch (err) {
      handleError(err)
      return { items: [], pagination: { page: 1, pageSize: 20, total: 0, totalPages: 0 } }
    } finally {
      isLoading.value = false
    }
  }

  async function getFeed(id: number) {
    try {
      isLoading.value = true
      clearError()
      const feed = await apiService.feeds.getFeed(String(id))
      const index = feedsCache.value.findIndex(f => f.id === id)
      if (index !== -1) {
        feedsCache.value[index] = feed
      } else {
        feedsCache.value.push(feed)
      }
      return feed
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function createFeed(data: CreateFeedRequest) {
    try {
      isLoading.value = true
      clearError()
      const newFeed = await apiService.feeds.createFeed(data)
      feedsCache.value.unshift(newFeed)
      return newFeed
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function updateFeed(id: number, data: UpdateFeedRequest) {
    try {
      isLoading.value = true
      clearError()
      const updatedFeed = await apiService.feeds.updateFeed(String(id), data)
      const index = feedsCache.value.findIndex(f => f.id === id)
      if (index !== -1) {
        feedsCache.value[index] = updatedFeed
      }
      return updatedFeed
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function deleteFeed(id: number, cascade?: boolean) {
    try {
      isLoading.value = true
      clearError()
      await apiService.feeds.deleteFeed(id, cascade)
      const index = feedsCache.value.findIndex(f => f.id === id)
      if (index !== -1) {
        feedsCache.value.splice(index, 1)
      }
      return true
    } catch (err) {
      handleError(err)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 摘要管理
  async function loadSummaries(params?: SummaryListParams) {
    try {
      isLoading.value = true
      clearError()
      const response = await apiService.summaries.getSummaries(params)
      summariesCache.value = response.items
      return response
    } catch (err) {
      handleError(err)
      return { items: [], pagination: { page: 1, pageSize: 20, total: 0, totalPages: 0 } }
    } finally {
      isLoading.value = false
    }
  }

  async function getSummary(id: number) {
    try {
      isLoading.value = true
      clearError()
      const summary = await apiService.summaries.getSummary(String(id))
      const index = summariesCache.value.findIndex(s => s.id === id)
      if (index !== -1) {
        summariesCache.value[index] = summary
      } else {
        summariesCache.value.push(summary)
      }
      return summary
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function addNote(summaryId: number, content: string) {
    try {
      isLoading.value = true
      clearError()
      const note = await apiService.summaries.addNote(summaryId, { content })
      const summary = summariesCache.value.find(s => s.id === summaryId)
      if (summary) {
        summary.notesList = summary.notesList || []
        summary.notesList.push(note)
        summary.noteCount = summary.notesList.length
      }
      return note
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function updateNote(summaryId: number, noteId: number, content: string) {
    try {
      isLoading.value = true
      clearError()
      const updatedNote = await apiService.summaries.updateNote(summaryId, String(noteId), { content })
      const summary = summariesCache.value.find(s => s.id === summaryId)
      if (summary && summary.notesList) {
        const noteIndex = summary.notesList.findIndex((_, index) => index === noteId)
        if (noteIndex !== -1) {
          summary.notesList[noteIndex] = updatedNote
        }
      }
      return updatedNote
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function deleteNote(summaryId: number, noteId: number) {
    try {
      isLoading.value = true
      clearError()
      await apiService.summaries.deleteNote(summaryId, String(noteId))
      const summary = summariesCache.value.find(s => s.id === summaryId)
      if (summary && summary.notesList) {
        const noteIndex = summary.notesList.findIndex((_, index) => index === noteId)
        if (noteIndex !== -1) {
          summary.notesList.splice(noteIndex, 1)
          summary.noteCount = summary.notesList.length
        }
      }
      return true
    } catch (err) {
      handleError(err)
      return false
    } finally {
      isLoading.value = false
    }
  }

  async function addTag(summaryId: number, tag: string) {
    try {
      isLoading.value = true
      clearError()
      const result = await apiService.summaries.addTag(summaryId, { tag })
      const summary = summariesCache.value.find(s => s.id === summaryId)
      if (summary) {
        summary.tags = result.tags
      }
      return result
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function removeTag(summaryId: number, tag: string) {
    try {
      isLoading.value = true
      clearError()
      const result = await apiService.summaries.removeTag(String(summaryId), tag)
      const summary = summariesCache.value.find(s => s.id === summaryId)
      if (summary) {
        summary.tags = result.tags
      }
      return result
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // 聊天管理
  async function loadChatSessions(params?: ChatSessionListParams) {
    try {
      isLoading.value = true
      clearError()
      console.log('📋 API Store: 加载聊天会话列表')

      const response = await apiService.chat.getSessions(params)
      chatSessionsCache.value = response.items

      console.log('✅ API Store: 聊天会话加载完成', {
        totalSessions: response.items.length,
        sessions: response.items.map(s => ({ id: s.id, title: s.title, messageCount: s.messageCount }))
      })

      return response
    } catch (err) {
      console.error('❌ API Store: 加载聊天会话失败', err)
      handleError(err)
      return { items: [], pagination: { page: 1, pageSize: 20, total: 0, totalPages: 0 } }
    } finally {
      isLoading.value = false
    }
  }

  async function getChatSession(id: number) {
    try {
      isLoading.value = true
      clearError()
      const session = await apiService.chat.getSession(String(id))
      const index = chatSessionsCache.value.findIndex(s => s.id === id)
      if (index !== -1) {
        chatSessionsCache.value[index] = session
      } else {
        chatSessionsCache.value.push(session)
      }
      return session
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function createChatSession(data?: CreateChatSessionRequest) {
    try {
      isLoading.value = true
      clearError()
      console.log('🆕 API Store: 创建聊天会话', data)

      const session = await apiService.chat.createSession(data)
      chatSessionsCache.value.unshift(session)

      console.log('✅ API Store: 聊天会话创建成功', {
        sessionId: session.id,
        title: session.title,
        totalSessions: chatSessionsCache.value.length
      })

      return session
    } catch (err) {
      console.error('❌ API Store: 创建聊天会话失败', err)
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function sendChatMessage(sessionId: number, data: SendMessageRequest) {
    try {
      isLoading.value = true
      clearError()
      console.log('📤 API Store: 发送聊天消息', { sessionId, content: data.content })

      const response = await apiService.chat.sendMessage(String(sessionId), data)
      console.log('📥 API Store: 收到响应', response)

      const session = chatSessionsCache.value.find(s => s.id === sessionId)
      if (session) {
        // 确保messages数组存在
        if (!session.messages) {
          session.messages = []
        }

        session.messages.push(response.userMessage)
        session.messages.push(response.assistantMessage)
        session.messageCount = session.messages.length
        session.updatedAt = new Date()

        console.log('✅ API Store: 会话更新完成', {
          sessionId: session.id,
          messageCount: session.messageCount,
          messagesLength: session.messages.length,
          lastUserMessage: response.userMessage.content,
          lastAiMessage: response.assistantMessage.content.substring(0, 100) + '...',
          totalSessions: chatSessionsCache.value.length
        })

        // 强制触发响应式更新
        const index = chatSessionsCache.value.findIndex(s => s.id === sessionId)
        if (index !== -1) {
          chatSessionsCache.value[index] = { ...session }
        }
      } else {
        console.warn('⚠️ API Store: 未找到对应会话', { sessionId, availableSessions: chatSessionsCache.value.map(s => s.id) })
      }
      return response
    } catch (err) {
      console.error('❌ API Store: 发送消息失败', err)
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function updateChatSession(id: number, data: UpdateChatSessionRequest) {
    try {
      isLoading.value = true
      clearError()
      const updatedSession = await apiService.chat.updateSession(id, data)
      const session = chatSessionsCache.value.find(s => s.id === id)
      if (session) {
        session.title = updatedSession.title
        session.updatedAt = updatedSession.updatedAt
      }
      return updatedSession
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function deleteChatSession(id: number) {
    try {
      isLoading.value = true
      clearError()
      await apiService.chat.deleteSession(id)
      const index = chatSessionsCache.value.findIndex(s => s.id === id)
      if (index !== -1) {
        chatSessionsCache.value.splice(index, 1)
      }
      return true
    } catch (err) {
      handleError(err)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 同步管理
  async function manualSync(options?: ManualSyncRequest) {
    try {
      isLoading.value = true
      clearError()
      const result = await apiService.sync.manualSync({
        feedIds: options?.feedIds,
        options: options?.options
      })
      return result
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function getSyncStatus() {
    try {
      clearError()
      return await apiService.sync.getSyncStatus()
    } catch (err) {
      handleError(err)
      return null
    }
  }

  async function cancelSync(syncId: string) {
    try {
      clearError()
      return await apiService.sync.cancelSync({ syncId })
    } catch (err) {
      handleError(err)
      return null
    }
  }

  // 定时任务管理
  async function loadTasks() {
    try {
      isLoading.value = true
      clearError()
      const response = await apiService.tasks.getTasks()
      tasksCache.value = response.items
      return response
    } catch (err) {
      handleError(err)
      return { items: [] }
    } finally {
      isLoading.value = false
    }
  }

  async function createTask(data: CreateScheduledTaskRequest) {
    try {
      isLoading.value = true
      clearError()
      const task = await apiService.tasks.createTask(data)
      tasksCache.value.unshift(task)
      return task
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function updateTask(id: string, data: UpdateScheduledTaskRequest) {
    try {
      isLoading.value = true
      clearError()
      const updatedTask = await apiService.tasks.updateTask(id, data)
      const index = tasksCache.value.findIndex(t => t.id === id)
      if (index !== -1) {
        Object.assign(tasksCache.value[index], updatedTask)
      }
      return tasksCache.value[index]
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function deleteTask(id: string) {
    try {
      isLoading.value = true
      clearError()
      await apiService.tasks.deleteTask(id)
      const index = tasksCache.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tasksCache.value.splice(index, 1)
      }
      return true
    } catch (err) {
      handleError(err)
      return false
    } finally {
      isLoading.value = false
    }
  }

  async function executeTask(id: string) {
    try {
      isLoading.value = true
      clearError()
      return await apiService.tasks.executeTask(id)
    } catch (err) {
      handleError(err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // 清除所有缓存
  function clearCache() {
    feedsCache.value = []
    summariesCache.value = []
    chatSessionsCache.value = []
    tasksCache.value = []
    clearError()
  }

  // 计算属性
  const feedsCount = computed(() => feedsCache.value.length)
  const summariesCount = computed(() => summariesCache.value.length)
  const chatSessionsCount = computed(() => chatSessionsCache.value.length)
  const tasksCount = computed(() => tasksCache.value.length)

  return {
    // 缓存数据
    feedsCache,
    summariesCache,
    chatSessionsCache,
    tasksCache,

    // 状态
    isLoading,
    error,

    // 计算属性
    feedsCount,
    summariesCount,
    chatSessionsCount,
    tasksCount,

    // 方法
    clearError,
    clearCache,

    // 订阅源管理
    loadFeeds,
    getFeed,
    createFeed,
    updateFeed,
    deleteFeed,

    // 摘要管理
    loadSummaries,
    getSummary,
    addNote,
    updateNote,
    deleteNote,
    addTag,
    removeTag,

    // 聊天管理
    loadChatSessions,
    getChatSession,
    createChatSession,
    sendChatMessage,
    updateChatSession,
    deleteChatSession,

    // 同步管理
    manualSync,
    getSyncStatus,
    cancelSync,

    // 定时任务管理
    loadTasks,
    createTask,
    updateTask,
    deleteTask,
    executeTask
  }
})
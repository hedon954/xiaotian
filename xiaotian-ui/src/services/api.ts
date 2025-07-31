// API 服务层
import { API_CONFIG, API_ENDPOINTS, buildApiUrl, DEFAULT_HEADERS } from '@/config/api'
import type {
  AddTagRequest,
  ApiChatSession,
  ApiEmailFullConfig,
  ApiFeed,
  ApiResponse,
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

// HTTP 请求方法
type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE'

// 请求选项
interface RequestOptions {
  method?: HttpMethod
  headers?: Record<string, string>
  body?: any
  timeout?: number
}

// 网络错误类
export class NetworkError extends Error {
  constructor(
    message: string,
    public status?: number,
    public code?: number
  ) {
    super(message)
    this.name = 'NetworkError'
  }
}

// API 客户端类
class ApiClient {
  private baseUrl: string
  private defaultHeaders: Record<string, string>

  constructor() {
    this.baseUrl = API_CONFIG.BASE_URL
    this.defaultHeaders = { ...DEFAULT_HEADERS }
  }

  /**
   * 发送 HTTP 请求
   */
  private async request<T>(
    endpoint: string,
    options: RequestOptions = {}
  ): Promise<T> {
    const {
      method = 'GET',
      headers = {},
      body,
      timeout = API_CONFIG.TIMEOUT
    } = options

    const url = buildApiUrl(endpoint)
    const config: RequestInit = {
      method,
      headers: {
        ...this.defaultHeaders,
        ...headers
      },
      signal: AbortSignal.timeout(timeout)
    }

    if (body && method !== 'GET') {
      config.body = JSON.stringify(body)
    }

    try {
      const response = await fetch(url, config)

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}))
        throw new NetworkError(
          errorData.message || `HTTP ${response.status}: ${response.statusText}`,
          response.status,
          errorData.code
        )
      }

      const result: ApiResponse<T> = await response.json()

      if (result.code !== 0) {
        throw new NetworkError(result.message, undefined, result.code)
      }

      return result.data
    } catch (error) {
      if (error instanceof NetworkError) {
        throw error
      }

      if (error instanceof DOMException && error.name === 'TimeoutError') {
        throw new NetworkError('请求超时，请稍后重试')
      }

      throw new NetworkError('网络连接失败，请检查网络设置')
    }
  }

  /**
   * GET 请求
   */
  async get<T>(endpoint: string, params?: Record<string, any>): Promise<T> {
    let url = endpoint
    if (params) {
      const searchParams = new URLSearchParams()
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          searchParams.append(key, String(value))
        }
      })
      const queryString = searchParams.toString()
      if (queryString) {
        url += (url.includes('?') ? '&' : '?') + queryString
      }
    }

    return this.request<T>(url, { method: 'GET' })
  }

  /**
   * POST 请求
   */
  async post<T>(endpoint: string, data?: any): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: data
    })
  }

  /**
   * PUT 请求
   */
  async put<T>(endpoint: string, data?: any): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'PUT',
      body: data
    })
  }

  /**
   * DELETE 请求
   */
  async delete<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'DELETE' })
  }
}

// 创建 API 客户端实例
const apiClient = new ApiClient()

// ============ 订阅源管理 API ============

export const feedsApi = {
  /**
   * 获取订阅源列表
   */
  async getFeeds(params?: FeedListParams): Promise<PaginationData<ApiFeed>> {
    return apiClient.get(API_ENDPOINTS.FEEDS, params)
  },

  /**
   * 获取订阅源详情
   */
  async getFeed(id: string): Promise<ApiFeed> {
    return apiClient.get(API_ENDPOINTS.FEED_DETAIL(id))
  },

  /**
   * 添加订阅源
   */
  async createFeed(data: CreateFeedRequest): Promise<ApiFeed> {
    return apiClient.post(API_ENDPOINTS.FEEDS, data)
  },

  /**
   * 更新订阅源
   */
  async updateFeed(id: string, data: UpdateFeedRequest): Promise<ApiFeed> {
    return apiClient.put(API_ENDPOINTS.FEED_DETAIL(id), data)
  },

  /**
   * 删除订阅源
   */
  async deleteFeed(id: number, cascade?: boolean): Promise<null> {
    const params = cascade ? { cascade: true } : undefined
    return apiClient.delete(API_ENDPOINTS.FEED_DETAIL(id) + (params ? '?cascade=true' : ''))
  }
}

// ============ 内容摘要管理 API ============

export const summariesApi = {
  /**
   * 获取摘要列表
   */
  async getSummaries(params?: SummaryListParams): Promise<PaginationData<ApiSummary>> {
    return apiClient.get(API_ENDPOINTS.SUMMARIES, params)
  },

  /**
   * 获取摘要详情
   */
      async getSummary(id: string): Promise<ApiSummary> {
    return apiClient.get(API_ENDPOINTS.SUMMARY_DETAIL(id))
  },

  /**
   * 获取相关摘要
   */
  async getRelatedSummaries(id: number, params?: RelatedSummaryParams): Promise<PaginationData<ApiSummary>> {
    return apiClient.get(API_ENDPOINTS.SUMMARY_RELATED(id), params)
  },

  /**
   * 添加摘要笔记
   */
  async addNote(summaryId: number, data: CreateNoteRequest): Promise<any> {
    return apiClient.post(API_ENDPOINTS.SUMMARY_NOTES(summaryId), data)
  },

  /**
   * 更新摘要笔记
   */
  async updateNote(summaryId: number, noteId: string, data: UpdateNoteRequest): Promise<any> {
    return apiClient.put(API_ENDPOINTS.SUMMARY_NOTE_DETAIL(summaryId, noteId), data)
  },

  /**
   * 删除摘要笔记
   */
  async deleteNote(summaryId: number, noteId: string): Promise<null> {
    return apiClient.delete(API_ENDPOINTS.SUMMARY_NOTE_DETAIL(summaryId, noteId))
  },

  /**
   * 添加摘要标签
   */
  async addTag(summaryId: string | number, data: AddTagRequest): Promise<{ tags: string[] }> {
    return apiClient.post(API_ENDPOINTS.SUMMARY_TAGS(summaryId), data)
  },

  /**
   * 删除摘要标签
   */
  async removeTag(summaryId: string, tag: string): Promise<{ tags: string[] }> {
    return apiClient.delete(API_ENDPOINTS.SUMMARY_TAG_DETAIL(summaryId, tag))
  }
}

// ============ 聊天/问答系统 API ============

export const chatApi = {
  /**
   * 获取聊天会话列表
   */
  async getSessions(params?: ChatSessionListParams): Promise<PaginationData<ApiChatSession>> {
    return apiClient.get(API_ENDPOINTS.CHAT_SESSIONS, params)
  },

  /**
   * 获取聊天会话详情
   */
  async getSession(id: string): Promise<ApiChatSession> {
    return apiClient.get(API_ENDPOINTS.CHAT_SESSION_DETAIL(id))
  },

  /**
   * 创建聊天会话
   */
  async createSession(data?: CreateChatSessionRequest): Promise<ApiChatSession> {
    return apiClient.post(API_ENDPOINTS.CHAT_SESSIONS, data)
  },

  /**
   * 发送聊天消息
   */
  async sendMessage(sessionId: string, data: SendMessageRequest): Promise<SendMessageResponse> {
    return apiClient.post(API_ENDPOINTS.CHAT_MESSAGES(sessionId), data)
  },

  /**
   * 更新聊天会话标题
   */
  async updateSession(id: number, data: UpdateChatSessionRequest): Promise<Pick<ApiChatSession, 'id' | 'title' | 'updatedAt'>> {
    return apiClient.put(API_ENDPOINTS.CHAT_SESSION_DETAIL(id), data)
  },

  /**
   * 删除聊天会话
   */
  async deleteSession(id: number): Promise<null> {
    return apiClient.delete(API_ENDPOINTS.CHAT_SESSION_DETAIL(id))
  }
}

// ============ 同步管理 API ============

export const syncApi = {
  /**
   * 执行手动同步
   */
  async manualSync(data?: ManualSyncRequest): Promise<SyncStartResponse> {
    return apiClient.post(API_ENDPOINTS.SYNC_MANUAL, data)
  },

  /**
   * 获取同步状态
   */
  async getSyncStatus(params?: SyncStatusParams): Promise<SyncStatusResponse> {
    return apiClient.get(API_ENDPOINTS.SYNC_STATUS, params)
  },

  /**
   * 获取同步历史
   */
  async getSyncHistory(params?: SyncHistoryParams): Promise<PaginationData<ApiSyncHistory>> {
    return apiClient.get(API_ENDPOINTS.SYNC_HISTORY, params)
  },

  /**
   * 取消同步任务
   */
  async cancelSync(data: CancelSyncRequest): Promise<CancelSyncResponse> {
    return apiClient.post(API_ENDPOINTS.SYNC_CANCEL, data)
  }
}

// ============ 定时任务管理 API ============

export const tasksApi = {
  /**
   * 获取定时任务列表
   */
  async getTasks(): Promise<{ items: ApiScheduledTask[] }> {
    return apiClient.get(API_ENDPOINTS.SCHEDULED_TASKS)
  },

  /**
   * 创建定时任务
   */
  async createTask(data: CreateScheduledTaskRequest): Promise<ApiScheduledTask> {
    return apiClient.post(API_ENDPOINTS.SCHEDULED_TASKS, data)
  },

  /**
   * 更新定时任务
   */
  async updateTask(id: string, data: UpdateScheduledTaskRequest): Promise<Pick<ApiScheduledTask, 'id' | 'name' | 'enabled' | 'cronExpression' | 'nextRun' | 'updatedAt'>> {
    return apiClient.put(API_ENDPOINTS.SCHEDULED_TASK_DETAIL(id), data)
  },

  /**
   * 删除定时任务
   */
  async deleteTask(id: string): Promise<null> {
    return apiClient.delete(API_ENDPOINTS.SCHEDULED_TASK_DETAIL(id))
  },

  /**
   * 手动执行定时任务
   */
  async executeTask(id: string): Promise<ExecuteTaskResponse> {
    return apiClient.post(API_ENDPOINTS.SCHEDULED_TASK_EXECUTE(id))
  },

  /**
   * 获取任务执行历史
   */
  async getTaskExecutions(id: string, params?: TaskExecutionParams): Promise<PaginationData<ApiTaskExecution>> {
    return apiClient.get(API_ENDPOINTS.SCHEDULED_TASK_EXECUTIONS(id), params)
  }
}

// ============ 邮件配置 API ============

export const emailApi = {
  /**
   * 获取邮件配置
   */
  async getConfig(): Promise<ApiEmailFullConfig> {
    return apiClient.get(API_ENDPOINTS.EMAIL_CONFIG)
  },

  /**
   * 更新邮件配置
   */
  async updateConfig(data: UpdateEmailConfigRequest): Promise<Omit<ApiEmailFullConfig, 'smtpConfig'> & { updatedAt: string }> {
    return apiClient.put(API_ENDPOINTS.EMAIL_CONFIG, data)
  },

  /**
   * 测试邮件配置
   */
  async testConfig(data: TestEmailRequest): Promise<TestEmailResponse> {
    return apiClient.post(API_ENDPOINTS.EMAIL_TEST, data)
  },

  /**
   * 更新SMTP配置
   */
  async updateSmtpConfig(data: UpdateSmtpConfigRequest): Promise<ApiSmtpConfig> {
    return apiClient.put(API_ENDPOINTS.EMAIL_SMTP, data)
  }
}

// ============ 系统管理 API ============

export const systemApi = {
  /**
   * 获取系统统计
   */
  async getStats(): Promise<ApiSystemStats> {
    return apiClient.get(API_ENDPOINTS.SYSTEM_STATS)
  },

  /**
   * 系统健康检查
   */
  async getHealth(): Promise<ApiSystemHealth> {
    return apiClient.get(API_ENDPOINTS.SYSTEM_HEALTH)
  },

  /**
   * 获取错误日志
   */
  async getLogs(params?: SystemLogsParams): Promise<PaginationData<ApiSystemLog>> {
    return apiClient.get(API_ENDPOINTS.SYSTEM_LOGS, params)
  },

  /**
   * 清理系统数据
   */
  async cleanup(data: SystemCleanupRequest): Promise<SystemCleanupResponse> {
    return apiClient.post(API_ENDPOINTS.SYSTEM_CLEANUP, data)
  }
}

// 导出默认的 API 服务
export const api = {
  feeds: feedsApi,
  summaries: summariesApi,
  chat: chatApi,
  sync: syncApi,
  tasks: tasksApi,
  email: emailApi,
  system: systemApi
}

export default api
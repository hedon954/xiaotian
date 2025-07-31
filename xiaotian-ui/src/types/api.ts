// API 相关类型定义
// 基于 API v0.0.4 文档

// 通用响应格式
export interface ApiResponse<T = any> {
  code: number
  data: T
  message: string
  trace_id: string
  request_id: string
}

// 分页数据格式
export interface PaginationData<T = any> {
  items: T[]
  pagination: {
    page: number
    pageSize: number
    total: number
    totalPages: number
  }
}

// 分页参数
export interface PaginationParams {
  page?: number
  pageSize?: number
}

// 错误响应
export interface ApiError {
  code: number
  message: string
  trace_id: string
  request_id: string
}

// ============ 订阅源管理 ============

export interface ApiFeed {
  id: number
  name: string
  type: 'rss'
  description: string
  feedUrl: string
  category: string
  status: 'active' | 'loading' | 'error'
  icon: string
  createdAt: Date
  lastUpdated: Date
  count?: number
}

export interface CreateFeedRequest {
  name: string
  type: 'rss'
  feedUrl: string
  description?: string
  category?: string
  icon?: string
}

export interface UpdateFeedRequest {
  name?: string
  type?: 'rss'
  description?: string
  category?: string
  icon?: string
}

export interface FeedListParams extends PaginationParams {
  category?: string
  status?: 'active' | 'loading' | 'error'
}

// ============ 内容摘要管理 ============

export interface ApiSummary {
  id: number
  title: string
  content: string
  fullContent: string
  originalUrl: string
  publishedAt: string
  tags: string[]
  feedId: number
  feedName: string
  noteCount: number
  sourceMaterialCount: number
  relatedSummaryCount: number
  createdAt: string
  updatedAt: string
  notesList: ApiNote[]
  sourceMaterials: ApiSourceMaterial[]
  relatedSummaries?: ApiRelatedSummary[]
}

export interface ApiNote {
  content: string
  createdAt: string
  updatedAt?: string
}

export interface ApiSourceMaterial {
  id: string
  title: string
  url: string
  publishedAt: string
  author?: string
  source: string
  excerpt?: string
  wordCount?: number
  readingTime?: number
  language?: string
  contentType: 'article' | 'video' | 'podcast' | 'document'
}

export interface ApiRelatedSummary {
  id: number
  title: string
  relevanceScore: number
  relationType: 'content' | 'temporal' | 'source' | 'tag'
  sharedTags?: string[]
  publishedAt: string
  excerpt: string
}

export interface SummaryListParams extends PaginationParams {
  feedId?: number
  startDate?: string
  endDate?: string
  tags?: string
  search?: string
}

export interface RelatedSummaryParams {
  limit?: number
  minScore?: number
}

export interface CreateNoteRequest {
  content: string
}

export interface UpdateNoteRequest {
  content: string
}

export interface AddTagRequest {
  tag: string
}

// ============ 聊天/问答系统 ============

export interface ApiChatSession {
  id: number
  title: string
  createdAt: Date
  updatedAt: Date
  messageCount?: number
  messages: ApiChatMessage[]
}

export interface ApiChatMessage {
  id: string // UUID格式 msg-{uuid}
  type: 'user' | 'assistant'
  content: string
  sources?: ApiChatMessageSource[]
  timestamp: string
}

export interface ApiChatMessageSource {
  summaryId: number
  summaryTitle: string
}

export interface CreateChatSessionRequest {
  title?: string
  initialMessage?: string
}

export interface SendMessageRequest {
  content: string
  context?: {
    summaryId?: string
    feedId?: string
  }
}

export interface SendMessageResponse {
  userMessage: ApiChatMessage
  assistantMessage: ApiChatMessage
}

export interface UpdateChatSessionRequest {
  title: string
}

export interface ChatSessionListParams extends PaginationParams {}

// ============ 同步管理 ============

export interface ManualSyncRequest {
  feedIds?: string[]
  options?: {
    includeAI?: boolean
    sendEmail?: boolean
    maxItems?: number
    summaryLength?: 'short' | 'medium' | 'long'
  }
}

export interface SyncStartResponse {
  syncId: string
  status: 'started'
  startTime: string
  estimatedDuration: number
  feedCount: number
}

export interface SyncStatusResponse {
  isRunning: boolean
  currentSync?: {
    syncId: string
    startTime: string
    progress: number
    currentAction: string
    feedsProcessed: number
    feedsTotal: number
    itemsProcessed: number
    itemsTotal: number
  }
  lastSyncTime: string | null
  lastSyncDuration: number | null
  errors: ApiSyncError[]
}

export interface ApiSyncError {
  feedId: string  // 改为string
  feedName: string
  error: string
  timestamp: Date
}

export interface SyncHistoryParams extends PaginationParams {
  startDate?: string
  endDate?: string
}

export interface ApiSyncHistory {
  syncId: string
  startTime: string
  endTime: string
  duration: number
  status: 'completed' | 'failed' | 'cancelled'
  feedsProcessed: number
  itemsProcessed: number
  summariesGenerated: number
  emailSent: boolean
  errors: number
}

export interface CancelSyncRequest {
  syncId: string
}

export interface CancelSyncResponse {
  syncId: string
  status: 'cancelled'
  cancelledAt: string
}

// ============ 定时任务管理 ============

export interface ApiScheduledTask {
  id: string
  name: string
  enabled: boolean
  cronExpression: string
  cronDescription?: string
  nextRun: Date
  lastRun: Date | null
  lastRunStatus?: 'success' | 'failed'
  lastRunDuration?: number
  emailConfig: ApiEmailConfig
  selectedFeeds: number[]
  aiSummaryEnabled: boolean
  summaryLength: 'short' | 'medium' | 'long'
  createdAt?: string
  updatedAt?: string
}

export interface CreateScheduledTaskRequest {
  name: string
  cronExpression: string
  emailConfig: ApiEmailConfig
  selectedFeeds: number[]
  aiSummaryEnabled: boolean
  summaryLength: 'short' | 'medium' | 'long'
  enabled?: boolean
}

export interface UpdateScheduledTaskRequest {
  name?: string
  enabled?: boolean
  cronExpression?: string
  emailConfig?: ApiEmailConfig
  selectedFeeds?: string[]
  aiSummaryEnabled?: boolean
  summaryLength?: 'short' | 'medium' | 'long'
}

export interface ExecuteTaskResponse {
  taskId: string
  executionId: string
  startTime: string
  status: 'started'
}

export interface ApiTaskExecution {
  executionId: string
  startTime: string
  endTime: string
  duration: number
  status: 'success' | 'failed'
  feedsProcessed: number
  summariesGenerated: number
  emailSent: boolean
  logs: string[]
}

export interface TaskExecutionParams extends PaginationParams {}

// ============ 邮件配置 ============

export interface ApiEmailConfig {
  enabled: boolean
  recipientEmails: string[]
  senderName: string
}

export interface ApiEmailFullConfig {
  enabled: boolean
  recipientEmails: string[]
  senderName: string
  template: {
    subject: string
    headerText: string
    footerText: string
    includeOriginalLinks: boolean
    groupByFeed: boolean
  }
  smtpConfig: {
    host: string
    port: number
    username: string
    authConfigured: boolean
    useTLS?: boolean
  }
}

export interface UpdateEmailConfigRequest {
  enabled?: boolean
  recipientEmails?: string[]
  senderName?: string
  template?: {
    subject?: string
    headerText?: string
    footerText?: string
    includeOriginalLinks?: boolean
    groupByFeed?: boolean
  }
}

export interface TestEmailRequest {
  recipientEmails: string[]
  testContent?: string
}

export interface TestEmailResponse {
  messageId: string
  sentAt: string
  recipients: string[]
  deliveryStatus: 'sent' | 'failed'
}

export interface UpdateSmtpConfigRequest {
  host: string
  port: number
  username: string
  password: string
  useTLS?: boolean
}

export interface ApiSmtpConfig {
  host: string
  port: number
  username: string
  authConfigured: boolean
  useTLS: boolean
  updatedAt: string
}

// ============ 系统管理 ============

export interface ApiSystemStats {
  feeds: {
    total: number
    active: number
    error: number
  }
  summaries: {
    total: number
    today: number
    thisWeek: number
    thisMonth: number
  }
  chatSessions: {
    total: number
    active: number
    messagesTotal: number
  }
  scheduledTasks: {
    total: number
    enabled: number
    lastRunSuccess: boolean
  }
  sync: {
    totalRuns: number
    successRate: number
    avgDuration: number
    lastSync: string | null
  }
}

export interface ApiSystemHealth {
  status: 'healthy' | 'degraded' | 'unhealthy'
  timestamp: string
  version: string
  uptime: number
  checks: {
    database: {
      status: 'healthy' | 'degraded' | 'unhealthy'
      responseTime: number
    }
    redis: {
      status: 'healthy' | 'degraded' | 'unhealthy'
      responseTime: number
    }
    email: {
      status: 'healthy' | 'degraded' | 'unhealthy'
      lastTest: string | null
    }
    feeds: {
      status: 'healthy' | 'degraded' | 'unhealthy'
      accessibleCount: number
      totalCount: number
    }
  }
}

export interface SystemLogsParams extends PaginationParams {
  level?: 'error' | 'warn' | 'info'
  startDate?: string
  endDate?: string
}

export interface ApiSystemLog {
  id: string
  level: 'error' | 'warn' | 'info'
  message: string
  module: string
  feedId?: number
  error?: string
  timestamp: string
  details?: Record<string, any>
}

export interface SystemCleanupRequest {
  cleanupType: 'old_logs' | 'old_summaries' | 'chat_sessions'
  retentionDays: number
  dryRun?: boolean
}

export interface SystemCleanupResponse {
  cleanupType: string
  itemsDeleted: number
  spaceFreed: string
  executedAt: string
}

// ============ 状态查询参数 ============

export interface SyncStatusParams {
  syncId?: string
}
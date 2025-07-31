// API 配置文件
import { getApiBaseUrl, getMockStatus } from '@/utils/env'

export const API_CONFIG = {
  // 基础 URL 配置
  BASE_URL: getApiBaseUrl(),

  // Mock 开关 - 可通过环境变量控制
  USE_MOCK: getMockStatus(),

  // 请求超时时间（毫秒）
  TIMEOUT: 10000,

  // 分页默认配置
  DEFAULT_PAGE_SIZE: 20,
  MAX_PAGE_SIZE: 100,

  // 重试配置
  MAX_RETRIES: 3,
  RETRY_DELAY: 1000,
}

// API 端点路径
export const API_ENDPOINTS = {
  // 订阅源管理
  FEEDS: '/feeds',
  FEED_DETAIL: (id: string | number) => `/feeds/${id}`,

  // 内容摘要管理
  SUMMARIES: '/summaries',
  SUMMARY_DETAIL: (id: string | number) => `/summaries/${id}`,
  SUMMARY_RELATED: (id: string | number) => `/summaries/${id}/related`,
  SUMMARY_NOTES: (id: string | number) => `/summaries/${id}/notes`,
  SUMMARY_NOTE_DETAIL: (summaryId: string | number, noteId: string | number) => `/summaries/${summaryId}/notes/${noteId}`,
  SUMMARY_TAGS: (id: string | number) => `/summaries/${id}/tags`,
  SUMMARY_TAG_DETAIL: (summaryId: string | number, tag: string) => `/summaries/${summaryId}/tags/${encodeURIComponent(tag)}`,

  // 聊天/问答系统
  CHAT_SESSIONS: '/chat/sessions',
  CHAT_SESSION_DETAIL: (id: string | number) => `/chat/sessions/${id}`,
  CHAT_MESSAGES: (sessionId: string | number) => `/chat/sessions/${sessionId}/messages`,

  // 同步管理
  SYNC_MANUAL: '/sync/manual',
  SYNC_STATUS: '/sync/status',
  SYNC_HISTORY: '/sync/history',
  SYNC_CANCEL: '/sync/cancel',

  // 定时任务管理
  SCHEDULED_TASKS: '/tasks/scheduled',
  SCHEDULED_TASK_DETAIL: (id: string) => `/tasks/scheduled/${id}`,
  SCHEDULED_TASK_EXECUTE: (id: string) => `/tasks/scheduled/${id}/execute`,
  SCHEDULED_TASK_EXECUTIONS: (id: string) => `/tasks/scheduled/${id}/executions`,

  // 邮件配置
  EMAIL_CONFIG: '/email/config',
  EMAIL_TEST: '/email/test',
  EMAIL_SMTP: '/email/smtp',

  // 系统管理
  SYSTEM_STATS: '/system/stats',
  SYSTEM_HEALTH: '/system/health',
  SYSTEM_LOGS: '/system/logs',
  SYSTEM_CLEANUP: '/system/cleanup',
} as const

// 完整URL构建函数
export function buildApiUrl(endpoint: string): string {
  return `${API_CONFIG.BASE_URL}${endpoint}`
}

// 请求头配置
export const DEFAULT_HEADERS = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
} as const

// 环境变量类型声明
declare global {
  interface ImportMetaEnv {
    readonly VITE_API_BASE_URL: string
    readonly VITE_USE_MOCK: string
  }
}
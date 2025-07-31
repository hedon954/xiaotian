// 应用视图类型
export type ViewType = 'summary' | 'qa' | 'detail'

// 导出API相关类型
export * from './api'

// 订阅源接口
export interface Feed {
  id: number
  name: string
  description: string
  feedUrl: string
  category: string
  lastUpdated: Date
  status: 'active' | 'loading' | 'error'
  icon: string
}

// 原始材料接口
export interface SourceMaterial {
  id: string
  title: string           // 原始标题
  url: string            // 原始链接
  publishedAt: string    // 发布时间
  author?: string        // 作者
  source: string         // 来源网站/平台
  excerpt?: string       // 摘要/简介
  wordCount?: number     // 字数
  readingTime?: number   // 预估阅读时间（分钟）
  language?: string      // 语言
  contentType: 'article' | 'video' | 'podcast' | 'document' // 内容类型
}

// 关联摘要接口
export interface RelatedSummary {
  id: number
  title: string
  relevanceScore: number    // 关联度评分 0-1
  relationType: 'content' | 'temporal' | 'source' | 'tag'  // 关联类型
  sharedTags?: string[]     // 共同标签
  publishedAt: string
  excerpt: string          // 简短摘要
}

// 摘要接口
export interface Summary {
  id: number
  title: string
  content: string
  originalUrl: string           // 保留兼容性，指向主要来源
  publishedAt: string
  tags: string[]
  notesList: Note[]
  fullContent: string
  sourceMaterials: SourceMaterial[]  // 多个原始材料
  relatedSummaries?: RelatedSummary[] // 关联摘要
}

// 笔记接口
export interface Note {
  content: string
  createdAt: string
}

// 聊天消息接口
export interface ChatMessage {
  id: string
  content: string
  type: 'user' | 'assistant'
  timestamp: string
  sources?: {
    summaryId: number
    summaryTitle: string
  }[]
}

// 聊天会话接口
export interface ChatSession {
  id: number
  title: string
  createdAt: Date
  updatedAt: Date
  messages: ChatMessage[]
}

// 新增订阅源数据接口
export interface NewFeedData {
  name: string
  feedUrl: string
  description?: string
  category?: string
}

// QA返回上下文接口
export interface QAReturnContext {
  fromQA: boolean
  sessionId?: number
}

// Cron表达式解析结果
export interface CronParseResult {
  isValid: boolean
  description: string
  nextRun?: Date
  error?: string
}

// 邮件配置接口（简化版，移除SMTP设置）
export interface EmailConfig {
  enabled: boolean
  recipientEmails: string[] // 改为多个邮件地址的数组
  senderName?: string
}

// 定时任务配置接口（使用cron表达式）
export interface ScheduledTaskConfig {
  id: string
  name: string
  enabled: boolean
  cronExpression: string // 改为cron表达式
  cronDescription?: string // cron表达式的人类可读描述
  nextRun: Date
  lastRun: Date | null
  emailConfig: EmailConfig
  selectedFeeds: number[]
  aiSummaryEnabled: boolean // 是否启用AI总结
  summaryLength: 'short' | 'medium' | 'long' // 总结长度
}

// 手动同步状态
export interface SyncStatus {
  isRunning: boolean
  lastSyncTime: Date | null
  progress: number // 0-100
  currentAction: string // 当前正在执行的操作描述
  errors: SyncError[]
}

// 同步错误信息
export interface SyncError {
  feedId: string
  feedName: string
  error: string
  timestamp: Date
}

// AI总结生成配置
export interface AISummaryConfig {
  enabled: boolean
  summaryType: 'highlights' | 'comprehensive' | 'technical' // 总结类型
  maxItems: number // 最大处理条目数
  includeKeywords: string[] // 关键词过滤
  excludeKeywords: string[] // 排除关键词
}

// 邮件模板配置
export interface EmailTemplate {
  subject: string
  headerText: string
  footerText: string
  includeOriginalLinks: boolean
  groupByFeed: boolean
}
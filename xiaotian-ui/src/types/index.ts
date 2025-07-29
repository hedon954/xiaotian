// 应用视图类型
export type ViewType = 'summary' | 'qa' | 'detail'

// 订阅源接口
export interface Feed {
  id: string
  name: string
  description: string
  feedUrl: string
  category: string
  lastUpdated: Date
  status: 'active' | 'loading' | 'error'
  icon: string
}

// 摘要接口
export interface Summary {
  id: string
  title: string
  content: string
  originalUrl: string
  publishedAt: string
  tags: string[]
  notesList: Note[]
  fullContent: string
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
  sources?: string[]
}

// 聊天会话接口
export interface ChatSession {
  id: string
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
  sessionId?: string
}
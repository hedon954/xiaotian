// 统一的服务入口
import { api as realApi } from './api'
import { mockApi, shouldUseMock } from './mock'

// 根据配置选择使用真实API还是Mock API
const useMock = shouldUseMock()
console.log(`🔧 API服务模式: ${useMock ? 'Mock' : 'Real'}`)
export const apiService = useMock ? mockApi : realApi

// 导出所有API服务
export const {
  feeds: feedsService,
  summaries: summariesService,
  chat: chatService,
  sync: syncService,
  tasks: tasksService,
  email: emailService,
  system: systemService
} = apiService

// 导出API相关工具
export { API_CONFIG } from '@/config/api'
export { NetworkError } from './api'
export { shouldUseMock } from './mock'

// 默认导出
export default apiService
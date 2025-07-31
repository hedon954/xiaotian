// 环境变量工具函数
export function getMockStatus(): boolean {
  const mockEnv = import.meta.env.VITE_USE_MOCK

  // 只有明确设置为 'false' 时才关闭 mock
  return mockEnv !== 'false'
}

export function getApiBaseUrl(): string {
  return import.meta.env.VITE_API_BASE_URL || '/api/v1'
}

// 调试用：输出当前环境变量状态
export function debugEnvVars(): void {
  console.log('🔧 环境变量调试信息：')
  console.log('  VITE_API_BASE_URL:', import.meta.env.VITE_API_BASE_URL)
  console.log('  VITE_USE_MOCK:', import.meta.env.VITE_USE_MOCK)
  console.log('  实际 USE_MOCK 值:', getMockStatus())
  console.log('  实际 BASE_URL 值:', getApiBaseUrl())
}
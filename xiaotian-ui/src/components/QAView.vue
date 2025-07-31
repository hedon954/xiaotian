<template>
  <main class="flex-1 flex flex-col bg-gray-50 dark:bg-gray-900 overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between p-6 lg:p-8 pb-4 flex-shrink-0">
      <div class="flex items-center space-x-4">
        <button
          @click="appStore.switchToSummaryView()"
          class="p-3 rounded-xl hover:bg-white dark:hover:bg-gray-800 transition-colors shadow-sm border border-gray-200 dark:border-gray-700"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-gray-600 dark:text-gray-300">
            <line x1="19" y1="12" x2="5" y2="12"></line>
            <polyline points="12 19 5 12 12 5"></polyline>
          </svg>
        </button>
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{{ currentChatSession?.title || '知识库问答' }}</h1>
          <p class="text-gray-500 dark:text-gray-400 mt-1">
            <span v-if="currentChatSession">
              创建于 {{ formatTime(currentChatSession.createdAt) }}
            </span>
            <span v-else>基于你的订阅内容进行智能问答</span>
          </p>
        </div>
      </div>

      <!-- Status and Session Info -->
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
          <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
          <span>AI 助手在线</span>
        </div>
        <div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
          <span>{{ currentChatMessages.length }} 条消息</span>
        </div>
        <button
          @click="createNewSession"
          class="flex items-center space-x-2 px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition-colors"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
          <span>新对话</span>
        </button>
      </div>
    </div>

    <!-- Chat Container -->
    <div class="flex-1 flex flex-col mx-6 lg:mx-8 mb-6 lg:mb-8 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <!-- Chat Messages -->
      <div ref="messagesContainer" class="flex-1 overflow-y-auto p-6 space-y-6" @scroll="handleScroll">
        <!-- Empty State -->
        <div v-if="currentChatMessages.length === 0" class="flex flex-col items-center justify-center h-full text-center">
          <div class="w-20 h-20 bg-gray-100 dark:bg-gray-700 rounded-2xl flex items-center justify-center mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="text-gray-400 dark:text-gray-500">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="7 10 12 15 17 10"/>
              <line x1="12" y1="15" x2="12" y2="3"/>
            </svg>
          </div>
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">开始新对话</h3>
          <p class="text-gray-500 dark:text-gray-400 max-w-md mb-4">
            向AI助手提问关于你订阅内容的任何问题，比如技术更新、最佳实践等。
          </p>
                  <div class="space-y-3">
          <button
            @click="focusInput"
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
          >
            开始提问
          </button>
          <div class="flex gap-2">
            <button
              @click="testChat('Rust有什么新功能？')"
              class="px-3 py-1 text-sm bg-gray-600 hover:bg-gray-700 text-white rounded transition-colors"
            >
              测试Rust问题
            </button>
            <button
              @click="testChat('Vue.js怎么样？')"
              class="px-3 py-1 text-sm bg-green-600 hover:bg-green-700 text-white rounded transition-colors"
            >
              测试Vue问题
            </button>
            <button
              @click="testChat('你好')"
              class="px-3 py-1 text-sm bg-purple-600 hover:bg-purple-700 text-white rounded transition-colors"
            >
              测试打招呼
            </button>
            <button
              @click="debugSessionState"
              class="px-3 py-1 text-sm bg-red-600 hover:bg-red-700 text-white rounded transition-colors"
            >
              调试状态
            </button>
          </div>
        </div>
        </div>

        <!-- Messages -->
        <div
          v-for="message in currentChatMessages"
          :key="message.id"
          :class="message.type === 'user' ? 'flex justify-end' : 'flex justify-start'"
        >
          <!-- User Message -->
          <div v-if="message.type === 'user'" class="bg-gradient-to-r from-blue-600 to-blue-700 text-white p-4 rounded-2xl max-w-2xl shadow-sm">
            <div class="flex items-start space-x-2">
              <div class="w-6 h-6 bg-blue-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
                  <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                  <circle cx="12" cy="7" r="4"/>
                </svg>
              </div>
              <div>
                <p class="font-medium text-sm text-blue-100 mb-1">你的问题</p>
                <p class="leading-relaxed">{{ message.content }}</p>
                <p class="text-xs text-blue-200 mt-2 opacity-75">{{ formatTime(message.timestamp) }}</p>
              </div>
            </div>
          </div>

          <!-- AI Message -->
          <div v-else class="bg-gray-50 dark:bg-gray-700 p-4 rounded-2xl max-w-2xl shadow-sm border border-gray-200 dark:border-gray-600">
            <div class="flex items-start space-x-3">
              <div class="w-6 h-6 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
                  <path d="M12 8V4H8"/>
                  <rect width="16" height="12" x="4" y="8" rx="2"/>
                  <path d="M2 14h2"/>
                  <path d="M20 14h2"/>
                  <path d="M15 13v2"/>
                  <path d="M9 13v2"/>
                </svg>
              </div>
              <div class="flex-1">
                <p class="font-semibold text-sm text-purple-600 dark:text-purple-400 mb-2">AI 助手回答</p>
                <div class="text-gray-700 dark:text-gray-300 leading-relaxed space-y-3">
                  <p>{{ message.content }}</p>

                  <!-- Sources -->
                  <div v-if="message.sources && message.sources.length > 0" class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-600">
                    <p class="font-semibold text-sm text-gray-600 dark:text-gray-400 mb-2 flex items-center space-x-1">
                      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                        <polyline points="14,2 14,8 20,8"/>
                        <line x1="16" y1="13" x2="8" y2="13"/>
                        <line x1="16" y1="17" x2="8" y2="17"/>
                        <polyline points="10,9 9,9 8,9"/>
                      </svg>
                      <span>参考来源</span>
                    </p>
                    <div class="space-y-1">
                      <button
                        v-for="source in message.sources"
                        :key="source.summaryId"
                        @click="jumpToSource(source.summaryId)"
                        class="block text-sm text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 hover:underline transition-colors"
                      >
                        {{ source.summaryTitle }}
                      </button>
                    </div>
                  </div>

                  <p class="text-xs text-gray-500 dark:text-gray-400 mt-3">{{ formatTime(message.timestamp) }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Loading Message -->
        <div v-if="isLoading" class="flex justify-start">
          <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-2xl shadow-sm border border-gray-200 dark:border-gray-600">
            <div class="flex items-start space-x-3">
              <div class="w-6 h-6 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white animate-spin">
                  <path d="M12 8V4H8"/>
                  <rect width="16" height="12" x="4" y="8" rx="2"/>
                  <path d="M2 14h2"/>
                  <path d="M20 14h2"/>
                  <path d="M15 13v2"/>
                  <path d="M9 13v2"/>
                </svg>
              </div>
              <div class="flex-1">
                <p class="font-semibold text-sm text-purple-600 dark:text-purple-400 mb-2">AI 助手正在思考...</p>
                <div class="flex space-x-1">
                  <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                  <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                  <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Input Area -->
      <div class="flex-shrink-0 p-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800">
        <div class="flex space-x-3">
          <div class="flex-1">
            <textarea
              ref="questionInput"
              v-model="newQuestion"
              rows="2"
              placeholder="在当前对话中继续提问..."
              class="w-full bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-xl p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors resize-none"
              @keyup.ctrl.enter="askQuestion"
              :disabled="isLoading"
            ></textarea>
          </div>
          <button
            @click="askQuestion"
            :disabled="!newQuestion.trim() || isLoading"
            class="px-6 py-2 bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 disabled:from-gray-400 disabled:to-gray-500 disabled:cursor-not-allowed text-white font-medium rounded-xl transition-all duration-200 shadow-sm flex items-center space-x-2"
          >
            <svg v-if="isLoading" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="animate-spin">
              <line x1="22" y1="12" x2="18" y2="12"></line>
              <line x1="6" y1="12" x2="2" y2="12"></line>
              <line x1="12" y1="6" x2="12" y2="2"></line>
              <line x1="12" y1="22" x2="12" y2="18"></line>
              <line x1="19" y1="19" x2="16" y2="16"></line>
              <line x1="8" y1="8" x2="5" y2="5"></line>
              <line x1="19" y1="5" x2="16" y2="8"></line>
              <line x1="8" y1="16" x2="5" y2="19"></line>
            </svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="22" y1="2" x2="11" y2="13"></line>
              <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
            </svg>
            <span>{{ isLoading ? '思考中...' : '发送' }}</span>
          </button>
        </div>
        <div class="flex items-center justify-between mt-2">
          <p class="text-xs text-gray-500 dark:text-gray-400">按 Ctrl + Enter 快速发送 • 在当前会话中继续对话</p>
          <button
            v-if="!showScrollToBottom"
            @click="scrollToBottom"
            class="text-xs text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
          >
            滚动到最新消息
          </button>
        </div>
      </div>
    </div>
  </main>
</template>

<script setup>
import { useApiStore } from '@/stores/api'
import { useAppStore } from '@/stores/app'
import { storeToRefs } from 'pinia'
import { computed, nextTick, onMounted, ref, watch } from 'vue'

const appStore = useAppStore()
const apiStore = useApiStore()
const { currentChatSession, currentChatSessionId } = storeToRefs(appStore)

// 使用API Store或App Store的聊天消息
const currentChatMessages = computed(() => {
  const sessionId = currentChatSessionId.value

  if (!sessionId) {
    console.log('⚠️ 没有当前会话ID')
    return []
  }

  // 优先检查API Store的会话
  const apiSessions = apiStore.chatSessionsCache
  if (apiSessions.length > 0) {
    const session = apiSessions.find(s => s.id === sessionId)
    if (session && session.messages) {
      console.log('✅ 使用API会话消息:', {
        sessionId,
        messageCount: session.messages.length,
        messages: session.messages.map(m => ({ type: m.type, content: m.content.substring(0, 50) + '...' }))
      })
      return session.messages
    }
  }

  // 降级到App Store的本地消息
  const localSessions = appStore.qaChatSessions
  if (localSessions.length > 0) {
    const session = localSessions.find(s => s.id === sessionId)
    if (session && session.messages) {
      console.log('✅ 使用本地会话消息:', {
        sessionId,
        messageCount: session.messages.length,
        messages: session.messages.map(m => ({ type: m.type, content: m.content.substring(0, 50) + '...' }))
      })
      return session.messages
    }
  }

  console.log('⚠️ 没有找到任何消息', {
    sessionId,
    apiSessions: apiSessions.length,
    localSessions: localSessions.length
  })
  return []
})

const newQuestion = ref('')
const isLoading = ref(false)
const messagesContainer = ref(null)
const questionInput = ref(null)
const showScrollToBottom = ref(true)

// 格式化时间
function formatTime(timestamp) {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 在当前会话中提问
async function askQuestion() {
  if (!newQuestion.value.trim() || isLoading.value) return

  const question = newQuestion.value.trim()
  newQuestion.value = ''
  isLoading.value = true

  console.log('💬 QAView发送消息:', question)

  try {
    // 确保有活跃的会话
    let sessionId = currentChatSessionId.value

    if (!sessionId || !apiStore.chatSessionsCache.find(s => s.id === sessionId)) {
      console.log('🆕 创建新的API会话')
      const session = await apiStore.createChatSession({ title: '新对话' })
      if (session) {
        sessionId = session.id
        appStore.switchChatSession(sessionId)
      }
    }

    if (sessionId) {
      // 使用API Store发送消息
      await apiStore.sendChatMessage(sessionId, { content: question })
      console.log('✅ API消息发送成功')
    } else {
      throw new Error('无法创建会话')
    }
  } catch (error) {
    console.error('❌ API消息发送失败，使用本地方法:', error)
    // 降级到App Store的方法
    appStore.askQuestionInCurrentSession(question)
  } finally {
    isLoading.value = false

    // 滚动到底部
    await nextTick()
    scrollToBottom()
    questionInput.value?.focus()
  }
}

// 创建新会话
async function createNewSession() {
  console.log('🆕 QAView创建新会话')
  try {
    // 优先使用API Store创建会话
    const session = await apiStore.createChatSession({ title: '新对话' })
    if (session) {
      appStore.switchChatSession(session.id)
      console.log('✅ API会话创建成功:', session)
      return
    }
  } catch (error) {
    console.error('❌ API会话创建失败，使用本地会话:', error)
  }

  // 降级到App Store的方法
  appStore.createNewChatSession()
}

// 聚焦输入框
function focusInput() {
  questionInput.value?.focus()
}

// 测试聊天功能
async function testChat(question) {
  console.log('🧪 测试聊天:', question)
  newQuestion.value = question
  await askQuestion()
}

// 调试会话状态
function debugSessionState() {
  console.log('🔍 调试会话状态:')
  console.log('当前会话ID:', currentChatSessionId.value)
  console.log('API会话数量:', apiStore.chatSessionsCache.length)
  console.log('API会话列表:', apiStore.chatSessionsCache.map(s => ({
    id: s.id,
    title: s.title,
    messageCount: s.messageCount,
    messagesLength: s.messages?.length || 0
  })))
  console.log('本地会话数量:', appStore.qaChatSessions.length)
  console.log('本地会话列表:', appStore.qaChatSessions.map(s => ({
    id: s.id,
    title: s.title,
    messagesLength: s.messages?.length || 0
  })))
  console.log('当前消息数量:', currentChatMessages.value.length)
  console.log('当前消息:', currentChatMessages.value)
}

// 跳转到参考文章
function jumpToSource(summaryId) {
  appStore.jumpToSourceFromQA(summaryId)
}

// 滚动到底部
function scrollToBottom() {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

// 处理滚动事件
function handleScroll() {
  if (messagesContainer.value) {
    const { scrollTop, scrollHeight, clientHeight } = messagesContainer.value
    showScrollToBottom.value = scrollTop + clientHeight < scrollHeight - 100
  }
}

// 监听消息变化，自动滚动到底部
watch(currentChatMessages, async (newMessages, oldMessages) => {
  console.log('👀 消息变化监听:', {
    oldCount: oldMessages?.length || 0,
    newCount: newMessages?.length || 0
  })

  await nextTick()
  if (showScrollToBottom.value) {
    scrollToBottom()
  }
}, { deep: true })

// 组件挂载时滚动到底部并聚焦输入框
onMounted(async () => {
  nextTick(() => {
    scrollToBottom()
    questionInput.value?.focus()
  })

  // 调试信息：检查当前聊天状态
  console.log('🔍 QAView挂载 - 当前状态检查:', {
    currentChatSessionId: currentChatSessionId.value,
    apiSessions: apiStore.chatSessionsCache.length,
    appSessions: appStore.qaChatSessions.length,
    currentMessages: currentChatMessages.value.length
  })

  // 如果没有当前会话ID，尝试设置一个默认会话
  if (!currentChatSessionId.value) {
    if (apiStore.chatSessionsCache.length > 0) {
      const firstSession = apiStore.chatSessionsCache[0]
      appStore.switchChatSession(firstSession.id)
      console.log('🎯 QAView: 设置默认API会话:', firstSession.id)
    } else if (appStore.qaChatSessions.length > 0) {
      const firstSession = appStore.qaChatSessions[0]
      appStore.switchChatSession(firstSession.id)
      console.log('🎯 QAView: 设置默认本地会话:', firstSession.id)
    } else {
      console.log('🆕 QAView: 没有会话，创建新会话')
      await createNewSession()
    }
  }
})
</script>
<template>
  <aside class="w-80 flex-shrink-0 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
      <h1 class="text-lg font-bold text-gray-900 dark:text-white flex items-center space-x-2">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
        </svg>
        <span>AI 助手</span>
      </h1>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">智能问答聊天</p>
    </div>

    <!-- Chat History -->
    <div class="flex-grow p-4 overflow-y-auto">
      <div class="mb-4">
        <div class="flex items-center justify-between mb-3">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-300 flex items-center space-x-2">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 3h18v18H3zM9 9h6v6H9z"/>
            </svg>
            <span>聊天历史</span>
          </h3>
          <button
            @click="createNewChat"
            class="text-xs text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 transition-colors flex items-center space-x-1"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"></line>
              <line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
            <span>新对话</span>
          </button>
        </div>

        <!-- Chat Sessions List -->
        <div class="space-y-1 max-h-48 overflow-y-auto">
          <div
            v-for="session in qaChatSessions"
            :key="session.id"
            @click="switchToChat(session.id)"
            class="group flex items-center justify-between p-3 rounded-lg cursor-pointer transition-all duration-200 hover:scale-[1.01]"
            :class="currentChatSessionId === session.id
              ? 'bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-900/30 dark:to-indigo-900/30 border border-blue-200 dark:border-blue-600 shadow-sm'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700/70 border border-transparent'"
          >
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium truncate text-gray-900 dark:text-white">{{ session.title }}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">{{ appStore.formatTimeAgo(session.updatedAt) }}</p>
            </div>
            <button
              v-if="qaChatSessions.length > 1"
              @click.stop="deleteChat(session.id)"
              class="opacity-0 group-hover:opacity-100 p-1.5 text-red-500 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/30 rounded-md transition-all"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M3 6h18l-2 13H5L3 6zM8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- New Chat Input -->
    <div class="p-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center space-x-2">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
          <circle cx="12" cy="11" r="1"/>
          <circle cx="8" cy="11" r="1"/>
          <circle cx="16" cy="11" r="1"/>
        </svg>
        <span>新对话</span>
      </h2>
      <textarea
        v-model="qaInput"
        rows="3"
        placeholder="有什么问题想要问我？"
        class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors resize-none"
        @keydown.ctrl.enter="handleQASubmit"
        @keydown.meta.enter="handleQASubmit"
      ></textarea>
      <div class="flex items-center justify-between mt-3">
        <p class="text-xs text-gray-500 dark:text-gray-400">
          Ctrl/Cmd + Enter 快速发送
        </p>
        <button
          @click="handleQASubmit"
          :disabled="!qaInput.trim()"
          class="bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 disabled:from-gray-400 disabled:to-gray-500 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-lg transition-all duration-200 shadow-sm disabled:shadow-none"
        >
          发送
        </button>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useApiStore } from '@/stores/api'
import { useAppStore } from '@/stores/app'
import { storeToRefs } from 'pinia'
import { computed, onMounted, ref } from 'vue'

const appStore = useAppStore()
const apiStore = useApiStore()
const { currentChatSessionId } = storeToRefs(appStore)

// 使用API Store的聊天会话数据
const qaChatSessions = computed(() => {
  return apiStore.chatSessionsCache.length > 0
    ? apiStore.chatSessionsCache
    : appStore.qaChatSessions
})

// QA 输入
const qaInput = ref<string>('')

// QA相关函数
const handleQASubmit = async () => {
  if (qaInput.value.trim()) {
    console.log('🗣️ 发送聊天消息:', qaInput.value.trim())

    // 检查是否有活跃的会话，如果没有则创建一个
    if (qaChatSessions.value.length === 0) {
      await createNewChat()
    }

    // 使用API Store发送消息
    const currentSessionId = currentChatSessionId.value || qaChatSessions.value[0]?.id
    if (currentSessionId) {
      try {
        await apiStore.sendChatMessage(currentSessionId, { content: qaInput.value.trim() })
        console.log('✅ 消息发送成功')
        qaInput.value = ''
        appStore.switchToQAView()
      } catch (error) {
        console.error('❌ 发送消息失败:', error)
        // 降级到App Store的方法
        appStore.startNewChatFromSidebar(qaInput.value.trim())
        qaInput.value = ''
      }
    }
  }
}

const createNewChat = async () => {
  console.log('🆕 创建新的聊天会话')
  try {
    // 优先使用API Store创建会话
    const session = await apiStore.createChatSession({ title: '新对话' })
    if (session) {
      appStore.switchChatSession(session.id)
      appStore.switchToQAView()
      console.log('✅ API会话创建成功:', session)
      return session.id
    }
  } catch (error) {
    console.error('❌ API会话创建失败，使用本地会话:', error)
  }

  // 降级到App Store的方法
  const sessionId = appStore.createNewChatSession()
  appStore.switchChatSession(sessionId)
  appStore.switchToQAView()
  return sessionId
}

const switchToChat = (sessionId: number) => {
  console.log('🔄 切换到聊天会话:', sessionId)
  appStore.switchChatSession(sessionId)
  appStore.switchToQAView()
}

const deleteChat = async (sessionId: number) => {
  console.log('🗑️ 删除聊天会话:', sessionId)
  try {
    // 优先使用API Store删除
    await apiStore.deleteChatSession(sessionId)
    console.log('✅ API会话删除成功')
  } catch (error) {
    console.error('❌ API会话删除失败，使用本地删除:', error)
    // 降级到App Store的方法
    appStore.deleteChatSession(sessionId)
  }
}

// 组件挂载时加载聊天会话
onMounted(async () => {
  console.log('💬 ChatPanel挂载，加载聊天会话...')
  try {
    await apiStore.loadChatSessions()
    console.log('✅ 聊天会话加载完成:', apiStore.chatSessionsCache)

    // 如果有API会话，设置第一个为当前会话
    if (apiStore.chatSessionsCache.length > 0) {
      const firstSession = apiStore.chatSessionsCache[0]
      appStore.switchChatSession(firstSession.id)
      console.log('🎯 设置默认会话:', firstSession.id)
    } else {
      console.log('🔧 没有API聊天会话，创建默认会话...')
      await createNewChat()
    }
  } catch (error) {
    console.error('❌ 聊天会话加载失败:', error)
  }
})
</script>

<style scoped>
/* Transitions for smooth interactions */
.group:hover {
  transform: translateY(-1px);
}</style>
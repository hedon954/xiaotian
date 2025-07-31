<template>
  <div
    v-if="visible"
    class="fixed inset-0 z-50 overflow-y-auto"
    @click.self="$emit('close')"
  >
    <div class="min-h-screen px-4 text-center">
      <!-- 背景遮罩 -->
      <div class="fixed inset-0 bg-black bg-opacity-50 transition-opacity"></div>

      <!-- 居中容器 -->
      <span class="inline-block h-screen align-middle" aria-hidden="true">&#8203;</span>

      <!-- 设置面板 -->
      <div class="inline-block w-full max-w-4xl p-6 my-8 text-left align-middle transition-all transform bg-white dark:bg-gray-800 shadow-xl rounded-2xl">
        <!-- 头部 -->
        <div class="flex items-center justify-between mb-6 pb-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-2xl font-bold text-gray-900 dark:text-white flex items-center space-x-3">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="3"/>
              <path d="M12 1v6m0 6v6"/>
              <path d="m15.5 3.5-3 3 3 3"/>
              <path d="m8.5 20.5 3-3-3-3"/>
            </svg>
            <span>系统设置</span>
          </h3>
          <button
            @click="$emit('close')"
            class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>

        <!-- 标签导航 -->
        <div class="flex space-x-1 mb-6">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="activeTab = tab.id"
            class="px-4 py-2 text-sm font-medium rounded-lg transition-colors"
            :class="activeTab === tab.id
              ? 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300'
              : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700'"
          >
            {{ tab.name }}
          </button>
        </div>

        <!-- 手动同步标签页 -->
        <div v-if="activeTab === 'sync'" class="space-y-6">
          <div>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center space-x-2">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/>
                <path d="M21 3v5h-5"/>
                <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/>
                <path d="M8 16H3v5"/>
              </svg>
              <span>手动同步</span>
            </h4>
            <p class="text-gray-600 dark:text-gray-400 mb-4">立即拉取所有订阅源的最新内容并生成AI总结</p>

            <!-- 同步状态 -->
            <div v-if="syncStatus.isRunning" class="mb-4 p-4 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-200 dark:border-blue-700">
              <div class="flex items-center space-x-3 mb-2">
                <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-blue-600"></div>
                <span class="text-blue-800 dark:text-blue-200 font-medium">正在同步...</span>
              </div>
              <p class="text-sm text-blue-600 dark:text-blue-300 mb-2">{{ syncStatus.currentAction }}</p>
              <div class="w-full bg-blue-200 dark:bg-blue-800 rounded-full h-2">
                <div
                  class="bg-blue-600 h-2 rounded-full transition-all duration-300"
                  :style="{ width: `${syncStatus.progress}%` }"
                ></div>
              </div>
            </div>

            <!-- 同步历史 -->
            <div v-if="syncStatus.lastSyncTime && !syncStatus.isRunning" class="mb-4 p-3 bg-green-50 dark:bg-green-900/30 rounded-lg border border-green-200 dark:border-green-700">
              <p class="text-sm text-green-800 dark:text-green-200">
                上次同步: {{ appStore.formatTimeAgo(syncStatus.lastSyncTime) }}
              </p>
            </div>

            <!-- 同步错误 -->
            <div v-if="syncStatus.errors.length > 0" class="mb-4 p-3 bg-red-50 dark:bg-red-900/30 rounded-lg border border-red-200 dark:border-red-700">
              <h5 class="text-sm font-medium text-red-800 dark:text-red-200 mb-2">同步错误:</h5>
              <ul class="text-sm text-red-600 dark:text-red-300 space-y-1">
                <li v-for="error in syncStatus.errors" :key="error.feedId">
                  {{ error.feedName }}: {{ error.error }}
                </li>
              </ul>
            </div>

            <!-- 同步选项 -->
            <div class="space-y-3 mb-4">
              <label class="flex items-center space-x-3">
                <input
                  v-model="manualSyncConfig.includeAISummary"
                  type="checkbox"
                  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                >
                <span class="text-sm text-gray-700 dark:text-gray-300">生成AI总结</span>
              </label>
              <label class="flex items-center space-x-3">
                <input
                  v-model="manualSyncConfig.sendEmailNotification"
                  type="checkbox"
                  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                >
                <span class="text-sm text-gray-700 dark:text-gray-300">发送邮件通知</span>
              </label>
            </div>

            <!-- 同步按钮 -->
            <button
              @click="startManualSync"
              :disabled="syncStatus.isRunning"
              class="w-full bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 disabled:from-gray-400 disabled:to-gray-500 disabled:cursor-not-allowed text-white font-medium py-3 px-6 rounded-lg transition-all duration-200 shadow-sm flex items-center justify-center space-x-2"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/>
                <path d="M21 3v5h-5"/>
                <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/>
                <path d="M8 16H3v5"/>
              </svg>
              <span>{{ syncStatus.isRunning ? '同步中...' : '立即同步' }}</span>
            </button>
          </div>
        </div>

        <!-- 定时任务标签页 -->
        <div v-if="activeTab === 'schedule'" class="space-y-6">
          <div>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center space-x-2">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"/>
                <polyline points="12,6 12,12 16,14"/>
              </svg>
              <span>定时任务配置</span>
            </h4>

            <!-- 启用定时任务 -->
            <div class="mb-6 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
              <label class="flex items-center space-x-3">
                <input
                  v-model="scheduledTask.enabled"
                  type="checkbox"
                  class="w-5 h-5 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                >
                <div>
                  <span class="text-base font-medium text-gray-700 dark:text-gray-300">启用定时任务</span>
                  <p class="text-sm text-gray-500 dark:text-gray-400">自动定期同步订阅源并发送邮件总结</p>
                </div>
              </label>
            </div>

            <!-- 任务配置 -->
            <div v-if="scheduledTask.enabled" class="space-y-4">
              <!-- 任务名称 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">任务名称</label>
                <input
                  v-model="scheduledTask.name"
                  type="text"
                  placeholder="例如: 每日技术资讯推送"
                  class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
                >
              </div>

              <!-- Cron表达式配置 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  执行计划 (Cron表达式)
                </label>

                <!-- 预设选项 -->
                <div class="mb-3">
                  <p class="text-xs text-gray-500 dark:text-gray-400 mb-2">常用预设：</p>
                  <div class="flex flex-wrap gap-2">
                    <button
                      v-for="preset in presetCronExpressions"
                      :key="preset.expression"
                      @click="selectPresetCron(preset.expression)"
                      class="text-xs px-3 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                    >
                      {{ preset.description }}
                    </button>
                  </div>
                </div>

                <!-- Cron表达式输入 -->
                <input
                  v-model="scheduledTask.cronExpression"
                  @input="parseCron"
                  type="text"
                  placeholder="0 9 * * * (每天上午9点)"
                  class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
                  :class="cronParseResult.isValid ? 'border-green-300 dark:border-green-600' : cronParseResult.error ? 'border-red-300 dark:border-red-600' : ''"
                >

                <!-- Cron表达式解析结果 -->
                <div v-if="cronParseResult.isValid" class="mt-2 p-3 bg-green-50 dark:bg-green-900/30 rounded-lg border border-green-200 dark:border-green-700">
                  <p class="text-sm text-green-800 dark:text-green-200">
                    <span class="font-medium">执行计划：</span>{{ cronParseResult.description }}
                  </p>
                  <p v-if="cronParseResult.nextRun" class="text-xs text-green-600 dark:text-green-300 mt-1">
                    下次执行：{{ formatDateTime(cronParseResult.nextRun) }}
                  </p>
                </div>

                <!-- Cron表达式错误 -->
                <div v-if="cronParseResult.error" class="mt-2 p-3 bg-red-50 dark:bg-red-900/30 rounded-lg border border-red-200 dark:border-red-700">
                  <p class="text-sm text-red-800 dark:text-red-200">{{ cronParseResult.error }}</p>
                  <p class="text-xs text-red-600 dark:text-red-300 mt-1">
                    Cron格式：分 时 日 月 周 (例如: 0 9 * * * 表示每天上午9点)
                  </p>
                </div>
              </div>

              <!-- 选择订阅源 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">选择订阅源</label>
                <div class="space-y-2 max-h-40 overflow-y-auto border border-gray-300 dark:border-gray-600 rounded-lg p-3">
                  <label v-for="feed in feeds" :key="feed.id" class="flex items-center space-x-3">
                    <input
                      v-model="scheduledTask.selectedFeeds"
                      :value="feed.id"
                      type="checkbox"
                      class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    >
                    <span class="text-sm">{{ feed.icon }} {{ feed.name }}</span>
                  </label>
                </div>
              </div>

              <!-- AI总结配置 -->
              <div>
                <label class="flex items-center space-x-3 mb-3">
                  <input
                    v-model="scheduledTask.aiSummaryEnabled"
                    type="checkbox"
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                  >
                  <span class="text-sm font-medium text-gray-700 dark:text-gray-300">启用AI总结</span>
                </label>

                <div v-if="scheduledTask.aiSummaryEnabled" class="ml-7">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">总结长度</label>
                  <select
                    v-model="scheduledTask.summaryLength"
                    class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
                  >
                    <option value="short">简短 (1-2段)</option>
                    <option value="medium">中等 (3-5段)</option>
                    <option value="long">详细 (完整总结)</option>
                  </select>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 邮件设置标签页 -->
        <div v-if="activeTab === 'email'" class="space-y-6">
          <div>
            <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center space-x-2">
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
                <polyline points="22,6 12,13 2,6"/>
              </svg>
              <span>邮件配置</span>
            </h4>

            <!-- 启用邮件 -->
            <div class="mb-6 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
              <label class="flex items-center space-x-3">
                <input
                  v-model="scheduledTask.emailConfig.enabled"
                  type="checkbox"
                  class="w-5 h-5 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                >
                <div>
                  <span class="text-base font-medium text-gray-700 dark:text-gray-300">启用邮件通知</span>
                  <p class="text-sm text-gray-500 dark:text-gray-400">将AI生成的总结发送到指定邮箱</p>
                </div>
              </label>
            </div>

            <!-- 邮件配置 -->
            <div v-if="scheduledTask.emailConfig.enabled" class="space-y-4">
              <!-- 收件人邮箱 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  收件人邮箱 *
                </label>
                <EmailTagsInput
                  v-model="scheduledTask.emailConfig.recipientEmails"
                  placeholder="输入邮件地址，按回车添加..."
                  :maxEmails="5"
                />
              </div>

              <!-- 发件人名称 -->
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">发件人名称</label>
                <input
                  v-model="scheduledTask.emailConfig.senderName"
                  type="text"
                  placeholder="小天AI助手"
                  class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
                >
              </div>
            </div>
          </div>
        </div>

        <!-- 底部按钮 -->
        <div class="flex items-center justify-end space-x-3 mt-8 pt-4 border-t border-gray-200 dark:border-gray-700">
          <button
            @click="$emit('close')"
            class="px-6 py-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 border border-gray-300 dark:border-gray-600 rounded-lg transition-colors hover:bg-gray-50 dark:hover:bg-gray-700"
          >
            取消
          </button>
          <button
            @click="saveSettings"
            class="px-6 py-2 text-sm bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-white font-medium rounded-lg transition-all duration-200 shadow-sm"
          >
            保存设置
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import EmailTagsInput from '@/components/EmailTagsInput.vue'
import { useApiStore } from '@/stores/api'
import { useAppStore } from '@/stores/app'
import type { CronParseResult, ScheduledTaskConfig, SyncStatus } from '@/types'
import { PRESET_CRON_EXPRESSIONS, parseCronExpression } from '@/utils/cronParser'
import { computed, onMounted, reactive, ref, watch } from 'vue'

// Props
interface Props {
  visible: boolean
}

defineProps<Props>()

// Emits
defineEmits<{
  close: []
}>()

const appStore = useAppStore()
const apiStore = useApiStore()


const feeds = computed(() => {
  return apiStore.feedsCache
})

// 标签页
const activeTab = ref<'sync' | 'schedule' | 'email'>('sync')
const tabs = [
  { id: 'sync' as const, name: '手动同步' },
  { id: 'schedule' as const, name: '定时任务' },
  { id: 'email' as const, name: '邮件设置' }
]

// 手动同步配置
const manualSyncConfig = reactive({
  includeAISummary: true,
  sendEmailNotification: false
})

// 同步状态
const syncStatus = ref<SyncStatus>({
  isRunning: false,
  lastSyncTime: null,
  progress: 0,
  currentAction: '',
  errors: []
})

// 定时任务配置
const scheduledTask = ref<ScheduledTaskConfig>({
  id: 'default-task',
  name: '每日技术资讯推送',
  enabled: false,
  cronExpression: '0 9 * * *',
  cronDescription: '每天上午9点',
  nextRun: new Date(),
  lastRun: null,
  emailConfig: {
    enabled: false,
    recipientEmails: [],
    senderName: '小天AI助手'
  },
  selectedFeeds: [],
  aiSummaryEnabled: true,
  summaryLength: 'medium'
})

// Cron表达式解析结果
const cronParseResult = ref<CronParseResult>({
  isValid: true,
  description: '每天上午9点'
})

// 预设的Cron表达式
const presetCronExpressions = PRESET_CRON_EXPRESSIONS

// 解析Cron表达式
const parseCron = () => {
  const result = parseCronExpression(scheduledTask.value.cronExpression)
  cronParseResult.value = result

  if (result.isValid) {
    scheduledTask.value.cronDescription = result.description
    if (result.nextRun) {
      scheduledTask.value.nextRun = result.nextRun
    }
  }
}

// 选择预设Cron表达式
const selectPresetCron = (expression: string) => {
  scheduledTask.value.cronExpression = expression
  parseCron()
}

// 格式化日期时间
const formatDateTime = (date: Date) => {
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 初始解析Cron表达式
parseCron()

// 监听Cron表达式变化
watch(() => scheduledTask.value.cronExpression, () => {
  parseCron()
})

// 开始手动同步 - 现在使用 API Store
const startManualSync = async () => {
  if (syncStatus.value.isRunning) return

  // 使用 App Store 的同步方法
  await appStore.manualSync({
    includeAI: manualSyncConfig.includeAISummary,
    sendEmail: manualSyncConfig.sendEmailNotification
  })
}

// 保存设置 - 现在使用 API Store
const saveSettings = async () => {
  try {
    // 如果是新任务则创建，否则更新
    if (scheduledTask.value.id === 'default-task') {
      await appStore.createScheduledTask(scheduledTask.value)
    } else {
      await appStore.updateScheduledTask(scheduledTask.value.id, scheduledTask.value)
    }

    appStore.showFeedbackMessage('设置已保存')
  } catch (error) {
    console.error('保存设置失败:', error)
    appStore.showFeedbackMessage('保存设置失败，请稍后重试')
  }
}

// 组件挂载时加载数据
onMounted(() => {
  apiStore.loadFeeds()
  apiStore.loadTasks()
})
</script>

<style scoped>
.rotate-90 {
  transform: rotate(90deg);
  transition: transform 0.2s ease-in-out;
}
</style>
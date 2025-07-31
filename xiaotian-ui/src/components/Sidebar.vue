<template>
  <aside class="w-80 flex-shrink-0 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center space-x-3">
      <div class="p-2 bg-gradient-to-r from-blue-600 to-blue-700 rounded-lg shadow-sm">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
          <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"/>
          <path d="m9 12 2 2 4-4"/>
        </svg>
      </div>
      <div>
        <h1 class="text-lg font-bold text-gray-900 dark:text-white">AI Info Tracker</h1>
        <p class="text-xs text-gray-500 dark:text-gray-400">智能信息聚合平台</p>
      </div>
    </div>

    <!-- Feed List -->
    <div class="flex-grow p-4 overflow-y-auto">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-300">订阅源</h2>
        <div class="flex items-center space-x-2">
          <!-- 快速同步按钮 -->
          <button
            @click="triggerQuickSync"
            :disabled="isQuickSyncing"
            class="flex items-center space-x-1 text-xs text-green-600 hover:text-green-700 dark:text-green-400 dark:hover:text-green-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed p-1.5 rounded-md hover:bg-green-50 dark:hover:bg-green-900/30"
            title="快速同步所有订阅源"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="14"
              height="14"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              :class="{'animate-spin': isQuickSyncing}"
            >
              <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/>
              <path d="M21 3v5h-5"/>
              <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/>
              <path d="M8 16H3v5"/>
            </svg>
          </button>

          <!-- 设置按钮 -->
          <button
            @click="showSettings = true"
            class="flex items-center space-x-1 text-xs text-gray-600 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300 transition-colors p-1.5 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700"
            title="系统设置"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="3"/>
              <path d="M12 1v6m0 6v6"/>
              <path d="m15.5 3.5-3 3 3 3"/>
              <path d="m8.5 20.5 3-3-3-3"/>
            </svg>
          </button>

          <!-- 添加订阅源按钮 -->
          <button
            @click="showAddFeed = !showAddFeed"
            class="flex items-center space-x-1 text-xs text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 transition-colors p-1.5 rounded-md hover:bg-blue-50 dark:hover:bg-blue-900/30"
            title="添加订阅源"
          >
            <svg v-if="!showAddFeed" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"></line>
              <line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
      </div>

      <!-- Add Feed Form (moved to top) -->
      <Transition name="slide-down">
        <div v-if="showAddFeed" class="mb-4 p-4 bg-gradient-to-r from-gray-50 to-gray-100 dark:from-gray-800 dark:to-gray-700 rounded-xl border border-gray-200 dark:border-gray-600">
          <h3 class="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-4 flex items-center space-x-2">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M4 11a9 9 0 0 1 9-9"/>
              <path d="M4 4a16 16 0 0 1 16 16"/>
              <circle cx="5" cy="19" r="1"/>
            </svg>
            <span>添加新订阅源</span>
          </h3>
          <div class="space-y-3">
            <div>
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">订阅源名称</label>
              <input
                v-model="newFeed.name"
                type="text"
                placeholder="例如: TechCrunch"
                class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-2.5 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
              >
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">RSS链接</label>
              <input
                v-model="newFeed.feedUrl"
                type="url"
                placeholder="https://example.com/feed.xml"
                class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-2.5 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
              >
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">简要描述</label>
              <textarea
                v-model="newFeed.description"
                rows="3"
                placeholder="描述这个订阅源的内容..."
                class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-2.5 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors resize-none"
              ></textarea>
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">分类</label>
              <select
                v-model="newFeed.category"
                class="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg p-2.5 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-colors"
              >
                <option value="科技">科技</option>
                <option value="编程">编程</option>
                <option value="前端">前端</option>
                <option value="设计">设计</option>
                <option value="产品">产品</option>
                <option value="其他">其他</option>
              </select>
            </div>
            <div class="flex space-x-2 pt-2">
              <button
                @click="handleAddFeed"
                :disabled="!newFeed.feedUrl.trim()"
                class="flex-1 bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 disabled:from-gray-400 disabled:to-gray-500 disabled:cursor-not-allowed text-white text-sm font-medium py-2.5 px-4 rounded-lg transition-all duration-200 shadow-sm"
              >
                添加订阅源
              </button>
              <button
                @click="resetAddForm"
                class="px-4 py-2.5 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200 border border-gray-300 dark:border-gray-600 rounded-lg transition-colors hover:bg-gray-50 dark:hover:bg-gray-700"
              >
                重置
              </button>
            </div>
          </div>

          <!-- Feedback Message -->
          <Transition name="feedback">
            <div
              v-if="showFeedback"
              class="mt-3 p-3 rounded-lg text-sm"
              :class="feedbackMessage.includes('成功')
                ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200 border border-green-200 dark:border-green-800'
                : 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200 border border-red-200 dark:border-red-800'"
            >
              {{ feedbackMessage }}
            </div>
          </Transition>
        </div>
      </Transition>

      <!-- Feed Cards -->
      <div class="space-y-3">
        <div
          v-for="feed in feeds"
          :key="feed.id"
          @click="handleFeedSelect(feed.name)"
          class="group relative p-4 rounded-xl border-2 transition-all duration-300 cursor-pointer overflow-hidden"
          :class="selectedFeed === feed.name
            ? 'bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-900/30 dark:to-indigo-900/30 border-blue-300 dark:border-blue-600 shadow-lg shadow-blue-200/50 dark:shadow-blue-900/30 scale-[1.02]'
            : 'bg-white dark:bg-gray-800/50 border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700/70 hover:border-gray-300 dark:hover:border-gray-600 hover:shadow-md hover:scale-[1.01]'"
        >
          <!-- 状态指示条 -->
          <div
            class="absolute top-0 left-0 w-full h-1 transition-all duration-300"
            :class="{
              'bg-gradient-to-r from-green-400 to-green-600': feed.status === 'active',
              'bg-gradient-to-r from-yellow-400 to-orange-500': feed.status === 'loading',
              'bg-gradient-to-r from-red-400 to-red-600': feed.status === 'error'
            }"
          ></div>

          <!-- Header Row -->
          <div class="flex items-start justify-between mb-3">
            <div class="flex items-center space-x-3 min-w-0 flex-1">
              <!-- Feed Icon -->
              <div class="w-10 h-10 bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-800 rounded-xl flex items-center justify-center text-lg flex-shrink-0 shadow-sm">
                {{ feed.icon }}
              </div>
              <!-- Feed Info -->
              <div class="min-w-0 flex-1">
                <h3 class="font-bold text-base text-gray-900 dark:text-white truncate leading-tight">
                  {{ feed.name }}
                </h3>
                <div class="flex items-center space-x-2 mt-1">
                  <span class="inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300">
                    {{ feed.category }}
                  </span>
                  <span class="text-xs text-gray-400 dark:text-gray-500">
                    {{ appStore.formatTimeAgo(feed.lastUpdated) }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Article Count Badge -->
            <div class="flex items-center space-x-2 flex-shrink-0">
              <div class="relative">
                <span class="inline-flex items-center px-3 py-1.5 rounded-full text-sm font-bold transition-all duration-200"
                      :class="selectedFeed === feed.name
                        ? 'bg-blue-500 text-white shadow-lg shadow-blue-500/30'
                        : feed.count > 0
                          ? 'bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300 group-hover:bg-blue-100 group-hover:text-blue-700 dark:group-hover:bg-blue-900 dark:group-hover:text-blue-300'
                          : 'bg-gray-50 text-gray-400 dark:bg-gray-800 dark:text-gray-500'">
                  {{ feed.count }}
                </span>
                <!-- 状态指示点 -->
                <div
                  class="absolute -top-1 -right-1 w-3 h-3 rounded-full border-2 border-white dark:border-gray-800 transition-all"
                  :class="{
                    'bg-green-500 shadow-lg shadow-green-500/50': feed.status === 'active',
                    'bg-yellow-500 shadow-lg shadow-yellow-500/50 animate-pulse': feed.status === 'loading',
                    'bg-red-500 shadow-lg shadow-red-500/50': feed.status === 'error'
                  }"
                ></div>
              </div>
            </div>
          </div>

          <!-- Description -->
          <p class="text-sm text-gray-600 dark:text-gray-400 line-clamp-2 mb-3 leading-relaxed">
            {{ feed.description }}
          </p>

          <!-- Footer Row -->
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-2">
              <!-- Status Text -->
              <span class="text-xs font-medium px-2 py-1 rounded-full transition-all"
                    :class="{
                      'bg-green-100 text-green-700 dark:bg-green-900/50 dark:text-green-300': feed.status === 'active',
                      'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/50 dark:text-yellow-300': feed.status === 'loading',
                      'bg-red-100 text-red-700 dark:bg-red-900/50 dark:text-red-300': feed.status === 'error'
                    }">
                {{ feed.status === 'active' ? '正常运行' : feed.status === 'loading' ? '加载中...' : '连接错误' }}
              </span>
            </div>

            <!-- Details Button -->
            <button
              @click.stop="showFeedDetails(feed)"
              class="opacity-0 group-hover:opacity-100 flex items-center space-x-1 text-xs text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 transition-all duration-200 bg-blue-50 dark:bg-blue-900/30 px-2 py-1 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/50"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="3"/>
                <path d="M12 1v6m0 6v6"/>
                <path d="m15.5 3.5-3 3 3 3"/>
                <path d="m8.5 20.5 3-3-3-3"/>
              </svg>
              <span>详情</span>
            </button>
          </div>
        </div>
      </div>


    </div>

    <!-- Settings Panel -->
    <SettingsPanel
      :visible="showSettings"
      @close="showSettings = false"
    />

    <!-- Feed Details Modal -->
    <Transition name="modal">
      <div v-if="selectedFeedDetails" class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4" @click="selectedFeedDetails = null">
        <div class="bg-white dark:bg-gray-800 rounded-3xl p-8 max-w-lg w-full shadow-2xl border border-gray-200 dark:border-gray-700" @click.stop>
          <!-- Header with Icon -->
          <div class="flex items-start justify-between mb-8">
            <div class="flex items-center space-x-4">
              <div class="w-14 h-14 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-2xl flex items-center justify-center text-2xl shadow-lg">
                {{ selectedFeedDetails.icon }}
              </div>
              <div>
                <h3 class="text-2xl font-bold text-gray-900 dark:text-white">订阅源详情</h3>
                <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">RSS 源信息概览</p>
              </div>
            </div>
            <button @click="selectedFeedDetails = null" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-xl">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>

          <div class="space-y-6">
            <!-- 基本信息卡片 -->
            <div class="bg-gradient-to-r from-gray-50 to-gray-100 dark:from-gray-700 dark:to-gray-800 p-5 rounded-2xl">
              <div class="flex items-center space-x-3 mb-3">
                <div
                  class="w-4 h-4 rounded-full flex-shrink-0 shadow-sm"
                  :class="{
                    'bg-green-500': selectedFeedDetails.status === 'active',
                    'bg-yellow-500 animate-pulse': selectedFeedDetails.status === 'loading',
                    'bg-red-500': selectedFeedDetails.status === 'error'
                  }"
                ></div>
                <h4 class="font-bold text-xl text-gray-900 dark:text-white">{{ selectedFeedDetails.name }}</h4>
              </div>

              <div class="flex items-center space-x-3 mb-3">
                <span class="inline-flex items-center px-3 py-1.5 rounded-xl text-sm font-semibold bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-200">
                  {{ selectedFeedDetails.category }}
                </span>
                <span class="inline-flex items-center px-3 py-1.5 rounded-xl text-sm font-semibold"
                      :class="{
                        'bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-200': selectedFeedDetails.status === 'active',
                        'bg-yellow-100 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200': selectedFeedDetails.status === 'loading',
                        'bg-red-100 text-red-800 dark:bg-red-800 dark:text-red-200': selectedFeedDetails.status === 'error'
                      }">
                  {{ selectedFeedDetails.status === 'active' ? '正常运行' : selectedFeedDetails.status === 'loading' ? '加载中' : '连接错误' }}
                </span>
              </div>

              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-600 dark:text-gray-400">文章数量</span>
                <span class="font-bold text-lg text-gray-900 dark:text-white">{{ selectedFeedDetails.count }} 篇</span>
              </div>
            </div>

            <!-- 描述 -->
            <div>
              <label class="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                  <polyline points="14,2 14,8 20,8"/>
                  <line x1="16" y1="13" x2="8" y2="13"/>
                  <line x1="16" y1="17" x2="8" y2="17"/>
                  <polyline points="10,9 9,9 8,9"/>
                </svg>
                <span>描述信息</span>
              </label>
              <p class="text-gray-600 dark:text-gray-400 leading-relaxed bg-gray-50 dark:bg-gray-700 p-4 rounded-xl border border-gray-200 dark:border-gray-600">
                {{ selectedFeedDetails.description }}
              </p>
            </div>

            <!-- RSS链接 -->
            <div>
              <label class="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3 flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M4 11a9 9 0 0 1 9-9"/>
                  <path d="M4 4a16 16 0 0 1 16 16"/>
                  <circle cx="5" cy="19" r="1"/>
                </svg>
                <span>RSS 订阅链接</span>
              </label>
              <div class="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-xl border border-blue-200 dark:border-blue-800">
                <a :href="selectedFeedDetails.feedUrl" target="_blank" class="group flex items-center space-x-2 text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 text-sm break-all transition-colors">
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="flex-shrink-0">
                    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                    <polyline points="15,3 21,3 21,9"></polyline>
                    <line x1="10" y1="14" x2="21" y2="3"></line>
                  </svg>
                  <span class="group-hover:underline">{{ selectedFeedDetails.feedUrl }}</span>
                </a>
              </div>
            </div>

            <!-- 底部信息 -->
            <div class="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-600">
              <div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="3"/>
                  <path d="M12 1v6m0 6v6"/>
                </svg>
                <span>最后更新: {{ appStore.formatTimeAgo(selectedFeedDetails.lastUpdated) }}</span>
              </div>

              <!-- 外部访问按钮 -->
              <a :href="selectedFeedDetails.feedUrl" target="_blank" class="inline-flex items-center space-x-2 bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-white text-sm font-medium py-2 px-4 rounded-xl transition-all duration-200 shadow-sm hover:shadow-md">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                  <polyline points="15,3 21,3 21,9"></polyline>
                  <line x1="10" y1="14" x2="21" y2="3"></line>
                </svg>
                <span>访问源站</span>
              </a>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </aside>
</template>

<script setup lang="ts">
import SettingsPanel from '@/components/SettingsPanel.vue'
import { useApiStore } from '@/stores/api'
import { useAppStore } from '@/stores/app'
import type { NewFeedData } from '@/types'
import { storeToRefs } from 'pinia'
import { computed, onMounted, reactive, ref } from 'vue'

const appStore = useAppStore()
const apiStore = useApiStore()
const { selectedFeed, feedbackMessage, showFeedback } = storeToRefs(appStore)


const feeds = computed(() => {
  return apiStore.feedsCache
})

// 新订阅源表单
const showAddFeed = ref<boolean>(false)
const newFeed = reactive<NewFeedData>({
  name: '',
  feedUrl: '',
  description: '',
  category: '科技'
})

// 设置面板
const showSettings = ref<boolean>(false)

// 快速同步状态
const isQuickSyncing = ref<boolean>(false)

// 选择的订阅源详情
const selectedFeedDetails = ref<any>(null)

// 处理订阅源选择
const handleFeedSelect = (feedName: string) => {
  console.log('点击订阅源:', feedName)
  appStore.selectFeed(feedName)
  // 如果当前不在摘要视图，切换到摘要视图
  if (appStore.currentView !== 'summary') {
    appStore.switchToSummaryView()
  }
}

// 处理添加订阅源
const handleAddFeed = async () => {
  if (!newFeed.name || !newFeed.feedUrl) {
    appStore.showFeedbackMessage('请填写必要的订阅源信息')
    return
  }

  await appStore.addFeed(newFeed)

  // 重置表单
  Object.assign(newFeed, {
    name: '',
    feedUrl: '',
    description: '',
    category: '科技'
  })
  showAddFeed.value = false
}



// 显示订阅源详情
const showFeedDetails = (feed: any) => {
  selectedFeedDetails.value = feed
}

// 重置添加订阅源表单
const resetAddForm = () => {
  Object.assign(newFeed, {
    name: '',
    feedUrl: '',
    description: '',
    category: '科技'
  })
}

// 快速同步功能
const triggerQuickSync = async () => {
  if (isQuickSyncing.value) return

  isQuickSyncing.value = true

  try {
    // 模拟快速同步过程
    await new Promise(resolve => setTimeout(resolve, 2000))
    appStore.showFeedbackMessage('快速同步完成！已获取最新内容并生成AI总结。')
  } catch (error) {
    console.error('快速同步失败:', error)
    appStore.showFeedbackMessage('快速同步失败，请检查网络连接后重试。')
  } finally {
    isQuickSyncing.value = false
  }
}

// 关闭订阅源详情
const closeFeedDetails = () => {
  selectedFeedDetails.value = null
}

// 组件挂载时加载API数据
onMounted(async () => {
  console.log('🔄 Sidebar挂载，开始加载数据...')
  try {
    await apiStore.loadFeeds()
    console.log('✅ 订阅源加载完成:', apiStore.feedsCache)

    // 也加载摘要数据
    await apiStore.loadSummaries()
    console.log('✅ 摘要数据加载完成:', apiStore.summariesCache)
  } catch (error) {
    console.error('❌ 数据加载失败:', error)
  }
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-clamp: 2;
  overflow: hidden;
}

/* Transitions */
.feedback-enter-active, .feedback-leave-active {
  transition: all 0.3s ease;
}
.feedback-enter-from, .feedback-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.slide-down-enter-active, .slide-down-leave-active {
  transition: all 0.3s ease;
}
.slide-down-enter-from, .slide-down-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

.modal-enter-active, .modal-leave-active {
  transition: all 0.3s ease;
}
.modal-enter-from, .modal-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
</style>
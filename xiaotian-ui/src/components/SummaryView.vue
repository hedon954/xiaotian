<template>
  <main class="flex-1 p-6 lg:p-8 overflow-y-auto bg-gray-50 dark:bg-gray-900">
    <!-- Header -->
    <div class="mb-6">
      <div class="flex items-center space-x-3 mb-2">
        <div class="w-8 h-8 bg-gradient-to-r from-blue-600 to-blue-700 rounded-lg flex items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
            <path d="M4 11a9 9 0 0 1 9-9"/>
            <path d="M4 4a16 16 0 0 1 16 16"/>
            <circle cx="5" cy="19" r="1"/>
          </svg>
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">{{ selectedFeed }}</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">最新摘要内容</p>
        </div>
      </div>
    </div>

    <!-- Content -->
    <div v-if="summaries.length > 0" class="space-y-4">
      <article
        v-for="summary in summaries"
        :key="summary.id"
        @click="handleSummaryClick(summary)"
        class="group bg-white dark:bg-gray-800 p-6 rounded-2xl border border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600 transition-all duration-200 cursor-pointer hover:shadow-lg"
      >
        <!-- Article Header -->
        <div class="flex items-start justify-between mb-4">
          <div class="min-w-0 flex-1">
            <h2 class="text-xl font-bold mb-2 text-gray-900 dark:text-white leading-tight group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
              {{ summary.title }}
            </h2>
            <div class="flex items-center text-sm text-gray-500 dark:text-gray-400 space-x-4">
              <span class="flex items-center space-x-1">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M4 11a9 9 0 0 1 9-9"/>
                  <path d="M4 4a16 16 0 0 1 16 16"/>
                  <circle cx="5" cy="19" r="1"/>
                </svg>
                <span>来源: <strong>{{ summary.originalUrl }}</strong></span>
              </span>
              <span class="flex items-center space-x-1">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="3"/>
                  <path d="M12 1v6m0 6v6"/>
                </svg>
                <span>{{ summary.publishedAt }}</span>
              </span>
            </div>
          </div>
          <button class="opacity-0 group-hover:opacity-100 text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 transition-all">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M7 17l10-10"/>
              <path d="M17 7H7v10"/>
            </svg>
          </button>
        </div>

        <!-- Article Content -->
        <p class="text-gray-600 dark:text-gray-300 leading-relaxed mb-4 line-clamp-3">
          {{ summary.content }}
        </p>

        <!-- Tags and Footer -->
        <div class="flex items-center justify-between">
          <!-- Tags preview -->
          <div class="flex flex-wrap gap-2" v-if="summary.tags && summary.tags.length > 0">
            <span
              v-for="tag in summary.tags.slice(0, 3)"
              :key="tag"
              class="inline-block px-3 py-1 text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200 rounded-full"
            >
              {{ tag }}
            </span>
            <span
              v-if="summary.tags.length > 3"
              class="inline-block px-3 py-1 text-xs font-medium bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400 rounded-full"
            >
              +{{ summary.tags.length - 3 }}
            </span>
          </div>

          <!-- Read More -->
          <div class="flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
            <span>阅读详情</span>
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="group-hover:translate-x-1 transition-transform">
              <line x1="5" y1="12" x2="19" y2="12"></line>
              <polyline points="12 5 19 12 12 19"></polyline>
            </svg>
          </div>
        </div>
      </article>
    </div>

    <!-- Empty State -->
    <div v-else class="flex flex-col items-center justify-center h-96 text-center">
      <div class="w-24 h-24 bg-gray-100 dark:bg-gray-800 rounded-2xl flex items-center justify-center mb-6">
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="text-gray-400 dark:text-gray-500">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
          <line x1="16" y1="13" x2="8" y2="13"/>
          <line x1="16" y1="17" x2="8" y2="17"/>
          <polyline points="10 9 9 9 8 9"/>
        </svg>
      </div>
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">暂无文章</h3>
      <p class="text-gray-500 dark:text-gray-400 max-w-md">
        {{ selectedFeed }} 订阅源还没有文章内容，请稍后再试或添加其他订阅源。
      </p>
    </div>
  </main>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { storeToRefs } from 'pinia'

const appStore = useAppStore()
const { summaries, selectedFeed } = storeToRefs(appStore)

function handleSummaryClick(summary) {
  console.log('点击了摘要:', summary.title) // 调试日志
  console.log('切换到详情视图:', summary) // 调试日志
  appStore.switchToDetailView(summary)
}
</script>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
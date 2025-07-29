<template>
  <main class="flex-1 p-6 lg:p-8 overflow-y-auto view-transition">
    <h1 class="text-2xl font-bold mb-6">{{ selectedFeed }} - 最新摘要</h1>
    <div class="space-y-6" v-if="summaries.length > 0">
      <article
        v-for="summary in summaries"
        :key="summary.id"
        @click="handleSummaryClick(summary)"
        class="summary-card bg-white dark:bg-gray-800 p-5 rounded-xl shadow-sm hover:shadow-md transition-shadow cursor-pointer"
      >
        <h2 class="text-lg font-bold mb-2 text-gray-900 dark:text-white">
          {{ summary.title }}
        </h2>
        <div class="flex items-center text-xs text-gray-500 dark:text-gray-400 mb-3 space-x-4">
          <span>来源: <strong>{{ summary.source }}</strong></span>
          <span>{{ summary.date }}</span>
        </div>
        <p class="text-gray-600 dark:text-gray-300 leading-relaxed">
          {{ summary.content }}
        </p>
        <!-- Tags preview -->
        <div class="flex flex-wrap gap-1 mt-3" v-if="summary.tags && summary.tags.length > 0">
          <span
            v-for="tag in summary.tags.slice(0, 3)"
            :key="tag"
            class="inline-block px-2 py-1 text-xs bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200 rounded-full"
          >
            {{ tag }}
          </span>
          <span
            v-if="summary.tags.length > 3"
            class="inline-block px-2 py-1 text-xs bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400 rounded-full"
          >
            +{{ summary.tags.length - 3 }}
          </span>
        </div>
      </article>
    </div>
    <div v-else class="flex flex-col items-center justify-center h-64 text-gray-500 dark:text-gray-400">
      <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" class="mb-4">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="16" y1="13" x2="8" y2="13"/>
        <line x1="16" y1="17" x2="8" y2="17"/>
        <polyline points="10 9 9 9 8 9"/>
      </svg>
      <p class="text-lg font-medium mb-2">暂无文章</p>
      <p class="text-sm">{{ selectedFeed }} 订阅源还没有文章内容</p>
    </div>
  </main>
</template>

<script setup>
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
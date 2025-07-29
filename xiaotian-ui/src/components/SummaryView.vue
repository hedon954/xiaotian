<template>
  <main class="flex-1 p-6 lg:p-8 overflow-y-auto view-transition">
    <h1 class="text-2xl font-bold mb-6">最新摘要</h1>
    <div class="space-y-6">
      <article
        v-for="summary in summaries"
        :key="summary.id"
        @click="handleSummaryClick(summary)"
        class="summary-card bg-white dark:bg-gray-800 p-5 rounded-xl shadow-sm hover:shadow-md transition-shadow"
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
      </article>
    </div>
  </main>
</template>

<script setup>
import { useAppStore } from '@/stores/app';

const appStore = useAppStore()
const { summaries } = appStore

function handleSummaryClick(summary) {
  appStore.switchToDetailView(summary)
}
</script>
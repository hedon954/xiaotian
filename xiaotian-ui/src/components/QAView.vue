<template>
  <div class="flex-1 flex-col p-6 lg:p-8 view-transition">
    <!-- Header -->
    <div class="flex-shrink-0 flex items-center mb-6">
      <button
        @click="appStore.switchToSummaryView()"
        class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 mr-4"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="19" y1="12" x2="5" y2="12"></line>
          <polyline points="12 19 5 12 12 5"></polyline>
        </svg>
      </button>
      <h1 class="text-2xl font-bold">沉浸式问答</h1>
    </div>

    <!-- Chat Content -->
    <div class="flex-grow overflow-y-auto space-y-6">
      <!-- User Question -->
      <div class="flex justify-end">
        <div class="bg-blue-600 text-white p-4 rounded-lg max-w-lg">
          <p>{{ qaQuestion || '最近 Rust 有哪些值得关注的性能更新?' }}</p>
        </div>
      </div>

      <!-- AI Answer -->
      <div class="flex justify-start">
        <div class="bg-white dark:bg-gray-800 p-4 rounded-lg max-w-lg shadow-sm">
          <p class="font-semibold text-gray-700 dark:text-gray-300 mb-2">AI 回答:</p>
          <div class="text-gray-600 dark:text-gray-300 leading-relaxed space-y-2">
            <p>{{ qaAnswer.content }}</p>
            <p class="font-semibold mt-4">参考来源:</p>
            <a
              v-for="source in qaAnswer.sources"
              :key="source"
              href="#"
              class="block text-sm text-blue-600 dark:text-blue-400 hover:underline"
            >
              {{ source }}
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useAppStore } from '@/stores/app';

const appStore = useAppStore()
const { qaQuestion, qaAnswer } = appStore
</script>
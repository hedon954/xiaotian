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
      <h1 class="text-2xl font-bold">{{ currentDetail?.title || '' }}</h1>
    </div>

    <!-- Content -->
    <div class="flex-grow overflow-y-auto" v-if="currentDetail">
      <!-- Meta Info -->
      <div class="text-sm text-gray-500 dark:text-gray-400 mb-4">
        <span>来源: <strong>{{ currentDetail.source }}</strong></span>
        <span class="ml-4">{{ currentDetail.date }}</span>
      </div>

      <!-- Summary Content -->
      <div class="prose prose-lg dark:prose-invert max-w-none text-gray-600 dark:text-gray-300 leading-relaxed">
        <p>{{ currentDetail.fullContent }}</p>
      </div>

      <!-- Original Link -->
      <div class="mt-6">
        <a
          :href="currentDetail.link"
          target="_blank"
          class="inline-flex items-center font-semibold text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300"
        >
          阅读原文
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-1">
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
            <polyline points="15 3 21 3 21 9"></polyline>
            <line x1="10" y1="14" x2="21" y2="3"></line>
          </svg>
        </a>
      </div>

      <!-- Notes Section -->
      <div class="mt-8 pt-6 border-t border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold mb-3">我的笔记</h3>
        <textarea
          v-model="notes"
          rows="5"
          placeholder="在这里添加你的想法和笔记... 这部分内容也会被加入知识库并与本文关联。"
          class="w-full bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
        ></textarea>
        <div class="flex justify-end mt-3">
          <button
            @click="handleSaveNotes"
            class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
          >
            保存笔记
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useAppStore } from '@/stores/app'
import { ref, watch } from 'vue'

const appStore = useAppStore()
const { currentDetail } = appStore

const notes = ref('')

// 当切换到新的详情时，加载对应的笔记
watch(currentDetail, (newDetail) => {
  if (newDetail) {
    notes.value = newDetail.notes || ''
  }
}, { immediate: true })

function handleSaveNotes() {
  if (currentDetail.value) {
    appStore.saveNotes(currentDetail.value.id, notes.value)
    alert('笔记已保存')
  }
}
</script>
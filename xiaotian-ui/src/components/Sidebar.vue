<template>
  <aside class="w-80 flex-shrink-0 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center space-x-3">
      <div class="p-2 bg-blue-600 rounded-lg">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
          <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"/>
          <path d="m9 12 2 2 4-4"/>
        </svg>
      </div>
      <h1 class="text-xl font-bold">AI Info Tracker</h1>
    </div>

    <!-- Feed List -->
    <div class="flex-grow p-4 overflow-y-auto">
      <h2 class="text-sm font-semibold text-gray-500 dark:text-gray-400 mb-3">订阅源</h2>
      <div class="space-y-2">
        <a
          v-for="feed in feeds"
          :key="feed.name"
          href="#"
          class="flex items-center justify-between p-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700"
        >
          <span class="font-semibold">{{ feed.name }}</span>
          <span class="text-xs font-bold text-gray-500 dark:text-gray-400">{{ feed.count }}</span>
        </a>
      </div>

      <!-- Add Feed Section -->
      <div class="mt-6">
        <label for="add-feed" class="text-sm font-semibold text-gray-500 dark:text-gray-400 mb-2 block">
          添加 RSS 源
        </label>
        <div class="flex space-x-2">
          <input
            v-model="newFeedUrl"
            type="url"
            id="add-feed"
            placeholder="https://example.com/feed.xml"
            class="flex-grow w-full bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
            @keyup.enter="handleAddFeed"
          >
          <button
            @click="handleAddFeed"
            class="bg-blue-600 hover:bg-blue-700 text-white p-2 rounded-lg flex-shrink-0"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"></line>
              <line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Q&A Section -->
    <div class="p-4 border-t border-gray-200 dark:border-gray-700">
      <h2 class="text-sm font-semibold text-gray-500 dark:text-gray-400 mb-2">知识库问答</h2>
      <textarea
        v-model="qaInput"
        rows="4"
        placeholder="向你的知识库提问..."
        class="w-full bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
      ></textarea>
      <button
        @click="handleQASubmit"
        class="w-full mt-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
      >
        开始问答
      </button>
    </div>
  </aside>
</template>

<script setup>
import { useAppStore } from '@/stores/app'
import { ref } from 'vue'

const appStore = useAppStore()
const { feeds } = appStore

const newFeedUrl = ref('')
const qaInput = ref('')

function handleAddFeed() {
  if (newFeedUrl.value.trim()) {
    appStore.addFeed(newFeedUrl.value)
    newFeedUrl.value = ''
  }
}

function handleQASubmit() {
  if (qaInput.value.trim()) {
    appStore.switchToQAView(qaInput.value)
    qaInput.value = ''
  } else {
    alert('请输入您的问题。')
  }
}
</script>
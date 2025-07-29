<script setup>
import DetailView from '@/components/DetailView.vue'
import QAView from '@/components/QAView.vue'
import Sidebar from '@/components/Sidebar.vue'
import SummaryView from '@/components/SummaryView.vue'
import { useAppStore } from '@/stores/app'
import { storeToRefs } from 'pinia'
import { watch } from 'vue'

const appStore = useAppStore()
const { currentView } = storeToRefs(appStore)

// 调试：监听视图变化
watch(currentView, (newView, oldView) => {
  console.log('App: 视图从', oldView, '切换到', newView)
}, { immediate: true })
</script>

<template>
  <div class="bg-gray-100 dark:bg-gray-900 text-gray-800 dark:text-gray-200 font-inter">
    <div class="flex h-screen overflow-hidden">
      <!-- Left Column: Navigation & Controls -->
      <Sidebar />

      <!-- Main Content Area -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Debug Info -->
        <div class="bg-yellow-100 p-2 text-xs text-yellow-800" v-if="true">
          当前视图: {{ currentView }}
        </div>

        <!-- View 1: Summaries List (Default) -->
        <SummaryView v-if="currentView === 'summary'" />

        <!-- View 2: Fullscreen Q&A -->
        <QAView v-else-if="currentView === 'qa'" />

        <!-- View 3: Summary Detail Page -->
        <DetailView v-else-if="currentView === 'detail'" />

        <!-- Fallback -->
        <div v-else class="flex-1 flex items-center justify-center">
          <p class="text-gray-500">未知视图: {{ currentView }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
html, body {
  height: 100%;
  margin: 0;
  padding: 0;
}

#app {
  height: 100vh;
}
</style>

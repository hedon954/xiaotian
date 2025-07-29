<script setup>
import DetailView from '@/components/DetailView.vue'
import QAView from '@/components/QAView.vue'
import Sidebar from '@/components/Sidebar.vue'
import SummaryView from '@/components/SummaryView.vue'
import { useAppStore } from '@/stores/app'
import { storeToRefs } from 'pinia'

const appStore = useAppStore()
const { currentView } = storeToRefs(appStore)
</script>

<template>
  <div class="bg-gray-100 dark:bg-gray-900 text-gray-800 dark:text-gray-200 font-inter">
    <div class="flex h-screen overflow-hidden">
      <!-- Left Column: Navigation & Controls -->
      <Sidebar />

      <!-- Main Content Area -->
      <div class="flex-1 flex flex-col overflow-hidden">
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

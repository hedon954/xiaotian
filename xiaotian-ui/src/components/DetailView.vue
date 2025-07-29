<template>
  <div class="flex-1 flex-col p-6 lg:p-8 view-transition">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6 flex-shrink-0">
      <div class="flex items-center space-x-3">
        <button
          @click="handleBackToSummary"
          class="p-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="19" y1="12" x2="5" y2="12"></line>
            <polyline points="12 19 5 12 12 5"></polyline>
          </svg>
        </button>
        <h1 class="text-2xl font-bold">详细内容</h1>
      </div>

      <!-- 返回聊天按钮 -->
      <button
        v-if="appStore.qaReturnContext"
        @click="returnToChat"
        class="flex items-center space-x-2 px-4 py-2 bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white font-medium rounded-xl transition-all duration-200 shadow-sm"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="7 10 12 15 17 10"/>
          <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
        <span>返回聊天</span>
      </button>
    </div>

    <!-- Content -->
    <div class="flex-grow overflow-y-auto" v-if="currentDetail">
      <!-- Meta Info -->
      <div class="text-sm text-gray-500 dark:text-gray-400 mb-4">
        <span>来源: <strong>{{ currentDetail.originalUrl }}</strong></span>
        <span class="ml-4">{{ currentDetail.publishedAt }}</span>
      </div>

      <!-- Tags Section -->
      <div class="mb-6">
        <h3 class="text-lg font-semibold mb-3">标签</h3>
        <div class="flex flex-wrap gap-2 mb-3">
          <span
            v-for="tag in currentDetail.tags"
            :key="tag"
            class="inline-flex items-center px-3 py-1 rounded-full text-sm bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200"
          >
            {{ tag }}
            <button
              @click="removeTag(tag)"
              class="ml-2 hover:bg-blue-200 dark:hover:bg-blue-800 rounded-full p-1"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>
          </span>
        </div>
        <div class="flex gap-2">
          <input
            v-model="newTag"
            @keyup.enter="addNewTag"
            placeholder="添加标签..."
            class="flex-grow bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
          >
          <button
            @click="addNewTag"
            class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors"
          >
            添加
          </button>
        </div>
      </div>

      <!-- Summary Content -->
      <div class="mb-6">
        <h3 class="text-lg font-semibold mb-3">摘要内容</h3>
        <div class="prose prose-lg dark:prose-invert max-w-none text-gray-600 dark:text-gray-300 leading-relaxed bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
          <p>{{ currentDetail.fullContent }}</p>
        </div>
      </div>

      <!-- Original Link -->
      <div class="mb-6">
        <h3 class="text-lg font-semibold mb-3">参考链接</h3>
        <a
                        :href="currentDetail.originalUrl"
          target="_blank"
          class="inline-flex items-center font-semibold text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 bg-blue-50 dark:bg-blue-900 px-4 py-2 rounded-lg transition-colors"
        >
          阅读原文
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-2">
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
            <polyline points="15 3 21 3 21 9"></polyline>
            <line x1="10" y1="14" x2="21" y2="3"></line>
          </svg>
        </a>
      </div>

      <!-- Notes Section -->
      <div class="pt-6 border-t border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold mb-3">我的笔记</h3>

        <!-- Existing Notes List -->
        <div v-if="notesList.length > 0" class="mb-6 space-y-3">
          <div
            v-for="(note, index) in notesList"
            :key="index"
            class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg border-l-4 border-blue-500"
          >
            <div class="flex justify-between items-start mb-2">
              <span class="text-xs text-gray-500 dark:text-gray-400">{{ note.createdAt }}</span>
              <button
                @click="deleteNote(index)"
                class="text-red-500 hover:text-red-700 text-xs"
              >
                删除
              </button>
            </div>
            <div
              class="prose dark:prose-invert max-w-none text-sm"
              v-html="appStore.renderMarkdown(note.content)"
            ></div>
          </div>
        </div>

        <!-- Add New Note -->
        <div class="space-y-3">
          <h4 class="font-medium text-gray-700 dark:text-gray-300">添加新笔记 (支持 Markdown)</h4>

          <!-- Markdown Editor and Preview Tabs -->
          <div class="mb-3">
            <div class="flex border-b border-gray-200 dark:border-gray-600">
              <button
                @click="activeTab = 'edit'"
                class="px-4 py-2 text-sm font-medium border-b-2 transition-colors"
                :class="activeTab === 'edit'
                  ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'border-transparent text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'"
              >
                编辑
              </button>
              <button
                @click="activeTab = 'preview'"
                class="px-4 py-2 text-sm font-medium border-b-2 transition-colors"
                :class="activeTab === 'preview'
                  ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'border-transparent text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'"
              >
                预览
              </button>
            </div>
          </div>

          <!-- Edit Tab -->
          <div v-if="activeTab === 'edit'">
            <textarea
              v-model="newNote"
              rows="6"
              placeholder="在这里添加你的想法和笔记... 支持 Markdown 语法：

**粗体文本**
*斜体文本*
`代码`
[链接](https://example.com)
- 列表项
- 另一个列表项"
              class="w-full bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-3 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none font-mono"
            ></textarea>
          </div>

          <!-- Preview Tab -->
          <div v-else-if="activeTab === 'preview'">
            <div
              class="w-full bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-3 min-h-[150px] prose dark:prose-invert max-w-none"
              v-html="newNotePreview"
            ></div>
          </div>

          <div class="flex justify-end">
            <button
              @click="handleAddNote"
              :disabled="!newNote.trim()"
              class="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-semibold py-2 px-4 rounded-lg transition-colors"
            >
              添加笔记
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { storeToRefs } from 'pinia'
import { computed, ref, watch } from 'vue'

const appStore = useAppStore()
const { currentDetail } = storeToRefs(appStore)

const newTag = ref('')
const activeTab = ref('edit')
const newNote = ref('')
const notesList = ref([])

// 计算新笔记的markdown预览
const newNotePreview = computed(() => {
  if (!newNote.value.trim()) {
    return '<p class="text-gray-500">暂无内容...</p>'
  }
  return appStore.renderMarkdown(newNote.value)
})

// 监听当前详情变化，加载对应的笔记列表
watch(currentDetail, (newDetail) => {
  if (newDetail) {
    // 使用笔记列表
    if (Array.isArray(newDetail.notesList)) {
      notesList.value = [...newDetail.notesList]
    } else {
      notesList.value = []
    }
  }
}, { immediate: true })

function handleBackToSummary() {
  appStore.switchToSummaryView()
}

// 返回到QA聊天界面
function returnToChat() {
  appStore.returnToQAChat()
}

function handleAddNote() {
  if (newNote.value.trim() && currentDetail.value) {
    const note = {
      content: newNote.value.trim(),
      createdAt: new Date().toLocaleString('zh-CN')
    }

    // 添加到本地列表
    notesList.value.push(note)

    // 保存到store
    appStore.addNoteToSummary(currentDetail.value.id, note)

    // 清空输入框
    newNote.value = ''

    // 切换到编辑标签页
    activeTab.value = 'edit'

    appStore.showFeedbackMessage('笔记已添加', 2000)
  }
}

function deleteNote(index) {
  if (currentDetail.value) {
    notesList.value.splice(index, 1)
    appStore.updateNotesForSummary(currentDetail.value.id, [...notesList.value])
    appStore.showFeedbackMessage('笔记已删除', 2000)
  }
}

function addNewTag() {
  if (newTag.value.trim() && currentDetail.value) {
    const success = appStore.addTag(currentDetail.value.id, newTag.value.trim())
    if (success) {
      newTag.value = ''
      appStore.showFeedbackMessage('标签已添加', 2000)
    } else {
      appStore.showFeedbackMessage('标签已存在', 2000)
    }
  }
}

function removeTag(tag) {
  if (currentDetail.value) {
    const success = appStore.removeTag(currentDetail.value.id, tag)
    if (success) {
      appStore.showFeedbackMessage('标签已移除', 2000)
    }
  }
}
</script>

<style scoped>
/* 为代码编辑器添加等宽字体 */
.font-mono {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

/* 确保预览区域的样式正确 */
:deep(.prose) {
  max-width: none;
}

:deep(.prose code) {
  background-color: rgba(0, 0, 0, 0.1);
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
}

:deep(.dark .prose code) {
  background-color: rgba(255, 255, 255, 0.1);
}
</style>
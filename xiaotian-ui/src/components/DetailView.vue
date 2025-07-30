<template>
  <div class="w-full bg-gradient-to-br from-gray-50 via-blue-50 to-indigo-50 dark:from-gray-900 dark:via-blue-900 dark:to-indigo-900 overflow-y-auto">
    <!-- Header -->
    <div class="sticky top-0 z-10 bg-white/80 dark:bg-gray-900/80 backdrop-blur-lg border-b border-gray-200/50 dark:border-gray-700/50">
      <div class="w-full px-6 py-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-4">
            <button
              @click="handleBackToSummary"
              class="group p-2 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 transform hover:scale-105"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-gray-600 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
                <line x1="19" y1="12" x2="5" y2="12"></line>
                <polyline points="12 19 5 12 12 5"></polyline>
              </svg>
            </button>
            <div>
              <h1 class="text-2xl font-bold bg-gradient-to-r from-gray-900 to-gray-600 dark:from-white dark:to-gray-300 bg-clip-text text-transparent">
                详细内容
              </h1>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">深度阅读与知识管理</p>
            </div>
          </div>

          <!-- 返回聊天按钮 -->
          <button
            v-if="appStore.qaReturnContext"
            @click="returnToChat"
            class="group flex items-center space-x-2 px-4 py-2 bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white font-medium rounded-xl transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="group-hover:animate-bounce">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="7 10 12 15 17 10"/>
              <line x1="12" y1="15" x2="12" y2="3"/>
            </svg>
            <span>返回聊天</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="w-full px-6 py-8" v-if="currentDetail">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 w-full min-h-screen">

        <!-- 左侧主内容区域 (2/3) -->
        <div class="lg:col-span-2 space-y-8">

          <!-- 文章标题和基本信息 -->
          <div class="bg-white/70 dark:bg-gray-800/70 backdrop-blur-sm rounded-2xl p-8 shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200/50 dark:border-gray-700/50">
            <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-6 leading-tight">
              {{ currentDetail.title }}
            </h2>

            <!-- 元数据信息 -->
            <div class="flex flex-wrap items-center gap-4 text-sm text-gray-600 dark:text-gray-400 mb-6">
              <div class="flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="10"/>
                  <polyline points="12 6 12 12 16 14"/>
                </svg>
                <span>{{ currentDetail.publishedAt }}</span>
              </div>
              <div class="flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M4 6l16 0"/>
                  <path d="M4 12l16 0"/>
                  <path d="M4 18l16 0"/>
                </svg>
                <span>{{ currentDetail.sourceMaterials?.length || 0 }} 个原始材料</span>
              </div>
              <div class="flex items-center space-x-2">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M9 11H5a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h4"/>
                  <path d="M15 11h4a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2h-4"/>
                  <path d="M12 11V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v4"/>
                </svg>
                <span>{{ computedRelatedSummaries.length }} 个相关摘要</span>
              </div>
            </div>

            <!-- 标签 -->
            <div class="flex flex-wrap gap-2 mb-6">
              <span
                v-for="tag in currentDetail.tags"
                :key="tag"
                class="group inline-flex items-center px-3 py-1 rounded-full text-sm bg-gradient-to-r from-blue-100 to-indigo-100 text-blue-800 dark:from-blue-900 dark:to-indigo-900 dark:text-blue-200 hover:from-blue-200 hover:to-indigo-200 dark:hover:from-blue-800 dark:hover:to-indigo-800 transition-all duration-200 cursor-pointer"
              >
                {{ tag }}
                <button
                  @click="removeTag(tag)"
                  class="ml-2 opacity-0 group-hover:opacity-100 hover:bg-blue-200 dark:hover:bg-blue-700 rounded-full p-0.5 transition-all duration-200"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                  </svg>
                </button>
              </span>

              <!-- 添加标签 -->
              <div class="flex items-center space-x-2">
                <input
                  v-model="newTag"
                  @keyup.enter="addNewTag"
                  placeholder="添加标签..."
                  class="px-3 py-1 bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-full text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all duration-200"
                >
                <button
                  @click="addNewTag"
                  class="p-1 bg-blue-600 hover:bg-blue-700 text-white rounded-full transition-colors duration-200"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="12" y1="5" x2="12" y2="19"></line>
                    <line x1="5" y1="12" x2="19" y2="12"></line>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- AI 总结内容 -->
          <div class="bg-white/70 dark:bg-gray-800/70 backdrop-blur-sm rounded-2xl p-8 shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200/50 dark:border-gray-700/50">
            <div class="flex items-center space-x-3 mb-6">
              <div class="w-10 h-10 bg-gradient-to-r from-purple-500 to-pink-500 rounded-xl flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
                  <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                  <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
                  <line x1="12" y1="22.08" x2="12" y2="12"/>
                </svg>
              </div>
              <h3 class="text-xl font-bold text-gray-900 dark:text-white">AI 智能摘要</h3>
            </div>
            <div class="prose prose-lg dark:prose-invert max-w-none">
              <div
                class="text-gray-700 dark:text-gray-300 leading-relaxed"
                v-html="appStore.renderMarkdown(currentDetail.fullContent)"
              ></div>
            </div>
          </div>
        </div>

        <!-- 右侧侧边栏 (1/3) -->
        <div class="w-full space-y-6">

          <!-- 我的笔记 -->
          <div class="w-full bg-white/70 dark:bg-gray-800/70 backdrop-blur-sm rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200/50 dark:border-gray-700/50">
            <div class="flex items-center space-x-3 mb-6">
              <div class="w-8 h-8 bg-gradient-to-r from-amber-500 to-orange-500 rounded-lg flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
                  <path d="M3 20.29V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v15.29"/>
                  <path d="M3 20.29L12 15l9 5.29"/>
                  <line x1="12" y1="3" x2="12" y2="15"/>
                </svg>
              </div>
              <h3 class="text-lg font-bold text-gray-900 dark:text-white">我的笔记</h3>
            </div>

            <!-- 现有笔记列表 -->
            <div v-if="notesList.length > 0" class="mb-6 space-y-3">
              <div
                v-for="(note, index) in notesList"
                :key="index"
                class="bg-gradient-to-r from-amber-50 to-yellow-50 dark:from-amber-900/20 dark:to-yellow-900/20 p-4 rounded-lg border-l-4 border-amber-500 hover:shadow-sm transition-all duration-200"
              >
                <div class="flex justify-between items-start mb-2">
                  <span class="text-xs text-amber-600 dark:text-amber-400 font-medium">{{ note.createdAt }}</span>
                  <button
                    @click="deleteNote(index)"
                    class="text-red-500 hover:text-red-700 hover:bg-red-100 dark:hover:bg-red-900/30 p-1 rounded transition-all duration-200"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <polyline points="3 6 5 6 21 6"/>
                      <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                    </svg>
                  </button>
                </div>
                <div
                  class="prose dark:prose-invert max-w-none text-xs"
                  v-html="appStore.renderMarkdown(note.content)"
                ></div>
              </div>
            </div>

            <!-- 添加新笔记 -->
            <div class="space-y-3">
              <h4 class="font-medium text-gray-700 dark:text-gray-300 text-sm">添加新笔记 (支持 Markdown)</h4>

              <!-- 标签页 -->
              <div class="border-b border-gray-200 dark:border-gray-600">
                <div class="flex space-x-6">
                  <button
                    @click="activeTab = 'edit'"
                    class="py-2 px-1 border-b-2 font-medium text-xs transition-colors duration-200"
                    :class="activeTab === 'edit'
                      ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                      : 'border-transparent text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'"
                  >
                    编辑
                  </button>
                  <button
                    @click="activeTab = 'preview'"
                    class="py-2 px-1 border-b-2 font-medium text-xs transition-colors duration-200"
                    :class="activeTab === 'preview'
                      ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                      : 'border-transparent text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'"
                  >
                    预览
                  </button>
                </div>
              </div>

              <!-- 编辑区域 -->
              <div v-if="activeTab === 'edit'">
                <textarea
                  v-model="newNote"
                  rows="4"
                  placeholder="添加笔记... 支持 Markdown"
                  class="w-full bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-3 text-xs focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none font-mono transition-all duration-200"
                ></textarea>
              </div>

              <!-- 预览区域 -->
              <div v-else-if="activeTab === 'preview'">
                <div
                  class="w-full bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-3 min-h-[100px] prose dark:prose-invert max-w-none text-xs"
                  v-html="newNotePreview"
                ></div>
              </div>

              <div class="flex justify-end">
                <button
                  @click="handleAddNote"
                  :disabled="!newNote.trim()"
                  class="bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 disabled:from-gray-400 disabled:to-gray-500 disabled:cursor-not-allowed text-white font-medium py-1.5 px-4 rounded-lg transition-all duration-200 text-sm"
                >
                  添加笔记
                </button>
              </div>
            </div>
          </div>

          <!-- 相关摘要 -->
          <div class="w-full bg-white/70 dark:bg-gray-800/70 backdrop-blur-sm rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200/50 dark:border-gray-700/50">
            <div class="flex items-center space-x-3 mb-6">
              <div class="w-8 h-8 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-lg flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
                  <path d="M9 11H5a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h4"/>
                  <path d="M15 11h4a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2h-4"/>
                  <path d="M12 11V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v4"/>
                </svg>
              </div>
              <h3 class="text-lg font-bold text-gray-900 dark:text-white">相关摘要</h3>
            </div>

            <div v-if="computedRelatedSummaries.length > 0" class="space-y-4">
              <div
                v-for="related in computedRelatedSummaries"
                :key="related.id"
                @click="jumpToRelatedSummary(related.id)"
                class="group cursor-pointer bg-gradient-to-r from-gray-50 to-indigo-50 dark:from-gray-700/50 dark:to-indigo-900/30 p-4 rounded-xl border border-gray-200/50 dark:border-gray-600/50 hover:shadow-md transition-all duration-200 hover:scale-[1.02]"
              >
                <!-- 相关度和类型 -->
                <div class="flex items-center justify-between mb-3">
                  <div class="flex items-center space-x-2">
                    <div class="w-2 h-2 rounded-full bg-gradient-to-r from-green-400 to-blue-500"></div>
                    <span class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide font-medium">
                      {{ getRelationTypeLabel(related.relationType) }}
                    </span>
                  </div>
                  <div class="flex items-center space-x-1">
                    <div class="w-16 bg-gray-200 dark:bg-gray-600 rounded-full h-1.5">
                      <div
                        class="bg-gradient-to-r from-green-400 to-blue-500 h-1.5 rounded-full transition-all duration-300"
                        :style="{ width: (related.relevanceScore * 100) + '%' }"
                      ></div>
                    </div>
                    <span class="text-xs text-gray-500 dark:text-gray-400 font-medium">
                      {{ Math.round(related.relevanceScore * 100) }}%
                    </span>
                  </div>
                </div>

                <!-- 标题 -->
                <h4 class="text-sm font-semibold text-gray-900 dark:text-white mb-2 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors line-clamp-2">
                  {{ related.title }}
                </h4>

                <!-- 摘要 -->
                <p class="text-xs text-gray-600 dark:text-gray-400 line-clamp-2 mb-3">
                  {{ related.excerpt }}
                </p>

                <!-- 共同标签 -->
                <div v-if="related.sharedTags && related.sharedTags.length > 0" class="flex flex-wrap gap-1 mb-2">
                  <span
                    v-for="tag in related.sharedTags.slice(0, 3)"
                    :key="tag"
                    class="inline-block px-2 py-0.5 bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 text-xs rounded-full"
                  >
                    {{ tag }}
                  </span>
                  <span v-if="related.sharedTags.length > 3" class="text-xs text-gray-500 dark:text-gray-400">
                    +{{ related.sharedTags.length - 3 }}
                  </span>
                </div>

                <!-- 发布时间 -->
                <div class="text-xs text-gray-500 dark:text-gray-400">
                  {{ related.publishedAt }}
                </div>
              </div>
            </div>

            <div v-else class="text-center text-gray-500 dark:text-gray-400 py-8">
              <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" class="mx-auto mb-4 opacity-50">
                <circle cx="11" cy="11" r="8"/>
                <path d="m21 21-4.35-4.35"/>
              </svg>
              <p class="text-sm">暂无相关摘要</p>
            </div>
          </div>

          <!-- 阅读统计 -->
          <div class="w-full bg-white/70 dark:bg-gray-800/70 backdrop-blur-sm rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200/50 dark:border-gray-700/50">
            <h3 class="text-lg font-bold text-gray-900 dark:text-white mb-4">阅读统计</h3>
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">总阅读时间</span>
                <span class="text-sm font-semibold text-gray-900 dark:text-white">
                  {{ totalReadingTime }} 分钟
                </span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">总字数</span>
                <span class="text-sm font-semibold text-gray-900 dark:text-white">
                  {{ totalWordCount.toLocaleString() }} 字
                </span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">笔记数量</span>
                <span class="text-sm font-semibold text-gray-900 dark:text-white">
                  {{ notesList.length }} 条
                </span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">标签数量</span>
                <span class="text-sm font-semibold text-gray-900 dark:text-white">
                  {{ currentDetail.tags.length }} 个
                </span>
              </div>
            </div>
          </div>

          <!-- 原始材料 -->
          <div class="w-full bg-white/70 dark:bg-gray-800/70 backdrop-blur-sm rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200/50 dark:border-gray-700/50">
            <div class="flex items-center space-x-3 mb-4">
              <div class="w-8 h-8 bg-gradient-to-r from-emerald-500 to-teal-500 rounded-lg flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-white">
                  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                  <polyline points="14 2 14 8 20 8"/>
                  <line x1="16" y1="13" x2="8" y2="13"/>
                  <line x1="16" y1="17" x2="8" y2="17"/>
                  <polyline points="10 9 9 9 8 9"/>
                </svg>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">原始材料</h3>
              <span class="text-xs text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded-full">
                {{ currentDetail.sourceMaterials?.length || 0 }} 篇
              </span>
            </div>

            <div class="space-y-3">
              <div
                v-for="(material, index) in currentDetail.sourceMaterials"
                :key="material.id"
                class="group bg-gradient-to-r from-gray-50 to-blue-50 dark:from-gray-700 dark:to-blue-900/30 rounded-lg p-4 border border-gray-200/50 dark:border-gray-600/50 hover:shadow-sm transition-all duration-200"
              >
                <!-- 材料头部 - 简化布局 -->
                <div class="flex items-center justify-between mb-2">
                  <div class="flex items-center space-x-2">
                    <div class="w-6 h-6 bg-gradient-to-r from-blue-500 to-indigo-500 rounded flex items-center justify-center text-white font-bold text-xs">
                      {{ index + 1 }}
                    </div>
                    <ContentTypeIcon :type="material.contentType" />
                    <span class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wide font-medium">
                      {{ getContentTypeLabel(material.contentType) }}
                    </span>
                  </div>
                  <a
                    :href="material.url"
                    target="_blank"
                    class="inline-flex items-center px-2 py-1 bg-blue-600 hover:bg-blue-700 text-white text-xs font-medium rounded transition-colors duration-200"
                  >
                    <span>原文</span>
                    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="ml-1">
                      <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                      <polyline points="15 3 21 3 21 9"></polyline>
                      <line x1="10" y1="14" x2="21" y2="3"></line>
                    </svg>
                  </a>
                </div>

                <!-- 材料标题 - 更小字体 -->
                <h4 class="text-sm font-medium text-gray-900 dark:text-white mb-2 hover:text-blue-600 dark:hover:text-blue-400 transition-colors cursor-pointer line-clamp-2">
                  {{ material.title }}
                </h4>

                <!-- 材料元信息 - 简化显示 -->
                <div class="flex flex-wrap gap-3 mb-2 text-xs text-gray-500 dark:text-gray-400">
                  <span v-if="material.author">{{ material.author }}</span>
                  <span>{{ material.source }}</span>
                  <span v-if="material.wordCount">{{ material.wordCount.toLocaleString() }} 字</span>
                  <span v-if="material.readingTime">{{ material.readingTime }} 分钟</span>
                </div>

                <!-- 材料摘要 - 限制行数 -->
                <p class="text-gray-600 dark:text-gray-400 text-xs leading-relaxed line-clamp-2" v-if="material.excerpt">
                  {{ material.excerpt }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import type { RelatedSummary } from '@/types'
import { storeToRefs } from 'pinia'
import { computed, ref, watch } from 'vue'

const appStore = useAppStore()
const { currentDetail } = storeToRefs(appStore)

const newTag = ref('')
const activeTab = ref('edit')
const newNote = ref('')
const notesList = ref([])

// 计算属性：动态计算关联摘要
const computedRelatedSummaries = computed<RelatedSummary[]>(() => {
  if (!currentDetail.value) return []

  // 优先使用预定义的关联摘要，如果没有则动态计算
  if (currentDetail.value.relatedSummaries && currentDetail.value.relatedSummaries.length > 0) {
    return currentDetail.value.relatedSummaries
  }

  return appStore.calculateRelatedSummaries(currentDetail.value.id)
})

// 计算属性：总阅读时间
const totalReadingTime = computed(() => {
  if (!currentDetail.value?.sourceMaterials) return 0
  return currentDetail.value.sourceMaterials.reduce((total, material) => {
    return total + (material.readingTime || 0)
  }, 0)
})

// 计算属性：总字数
const totalWordCount = computed(() => {
  if (!currentDetail.value?.sourceMaterials) return 0
  return currentDetail.value.sourceMaterials.reduce((total, material) => {
    return total + (material.wordCount || 0)
  }, 0)
})

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
    if (Array.isArray(newDetail.notesList)) {
      notesList.value = [...newDetail.notesList]
    } else {
      notesList.value = []
    }
  }
}, { immediate: true })

// 内容类型图标组件
const ContentTypeIcon = {
  props: ['type'],
  template: `
    <div class="w-5 h-5 flex items-center justify-center">
      <svg v-if="type === 'article'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-blue-600 dark:text-blue-400">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="16" y1="13" x2="8" y2="13"/>
        <line x1="16" y1="17" x2="8" y2="17"/>
        <polyline points="10 9 9 9 8 9"/>
      </svg>
      <svg v-else-if="type === 'video'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-red-600 dark:text-red-400">
        <polygon points="23 7 16 12 23 17 23 7"/>
        <rect x="1" y="5" width="15" height="14" rx="2" ry="2"/>
      </svg>
      <svg v-else-if="type === 'podcast'" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-green-600 dark:text-green-400">
        <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"/>
        <path d="M19 10v2a7 7 0 0 1-14 0v-2"/>
        <line x1="12" y1="19" x2="12" y2="23"/>
        <line x1="8" y1="23" x2="16" y2="23"/>
      </svg>
      <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-gray-600 dark:text-gray-400">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
      </svg>
    </div>
  `
}

// 获取内容类型标签
function getContentTypeLabel(type: string): string {
  const labels = {
    'article': '文章',
    'video': '视频',
    'podcast': '播客',
    'document': '文档'
  }
  return labels[type] || '文档'
}

// 获取关联类型标签
function getRelationTypeLabel(type: string): string {
  const labels = {
    'content': '内容相关',
    'tag': '标签匹配',
    'temporal': '时间相关',
    'source': '来源相关'
  }
  return labels[type] || '相关'
}

// 跳转到相关摘要
function jumpToRelatedSummary(summaryId: string) {
  // 遍历所有摘要找到目标摘要
  for (const feedId in appStore.feedSummaries) {
    const summary = appStore.feedSummaries[feedId].find(s => s.id === summaryId)
    if (summary) {
      appStore.switchToDetailView(summary)
      break
    }
  }
}

function handleBackToSummary() {
  appStore.switchToSummaryView()
}

function returnToChat() {
  appStore.returnToQAChat()
}

function handleAddNote() {
  if (newNote.value.trim() && currentDetail.value) {
    const note = {
      content: newNote.value.trim(),
      createdAt: new Date().toLocaleString('zh-CN')
    }

    notesList.value.push(note)
    appStore.addNoteToSummary(currentDetail.value.id, note)

    newNote.value = ''
    activeTab.value = 'edit'

    appStore.showFeedbackMessage('笔记已添加')
  }
}

function deleteNote(index) {
  if (currentDetail.value) {
    notesList.value.splice(index, 1)
    appStore.updateNotesForSummary(currentDetail.value.id, [...notesList.value])
    appStore.showFeedbackMessage('笔记已删除')
  }
}

function addNewTag() {
  if (newTag.value.trim() && currentDetail.value) {
    const success = appStore.addTag(currentDetail.value.id, newTag.value.trim())
    if (success) {
      newTag.value = ''
      appStore.showFeedbackMessage('标签已添加')
    } else {
      appStore.showFeedbackMessage('标签已存在')
    }
  }
}

function removeTag(tag) {
  if (currentDetail.value) {
    const success = appStore.removeTag(currentDetail.value.id, tag)
    if (success) {
      appStore.showFeedbackMessage('标签已移除')
    }
  }
}
</script>

<style scoped>
/* 行数限制样式 */
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.3);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.5);
}

/* 深色模式滚动条 */
.dark ::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
}

.dark ::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.3);
}

.dark ::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.5);
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

/* 动画类 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.view-transition {
  animation: fadeInUp 0.5s ease-out;
}
</style>
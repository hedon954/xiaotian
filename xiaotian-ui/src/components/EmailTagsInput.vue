<template>
  <div class="email-tags-input">
    <div class="flex flex-wrap gap-2 p-3 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg min-h-[3rem] focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500 transition-colors">
      <!-- 邮件地址标签 -->
      <div
        v-for="(email, index) in emails"
        :key="index"
        class="flex items-center space-x-1 bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200 px-2 py-1 rounded-md text-sm border border-blue-200 dark:border-blue-700"
      >
        <span>{{ email }}</span>
        <button
          @click="removeEmail(index)"
          class="ml-1 text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-200 hover:bg-blue-200 dark:hover:bg-blue-800 rounded-full w-4 h-4 flex items-center justify-center text-xs"
          type="button"
        >
          ×
        </button>
      </div>

      <!-- 输入框 -->
      <input
        v-model="inputValue"
        @keydown="handleKeydown"
        @blur="handleBlur"
        :placeholder="emails.length === 0 ? placeholder : ''"
        class="flex-1 min-w-[200px] bg-transparent border-none outline-none text-sm text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400"
        type="email"
      >
    </div>

    <!-- 错误信息 -->
    <div v-if="errorMessage" class="mt-1 text-sm text-red-600 dark:text-red-400">
      {{ errorMessage }}
    </div>

    <!-- 提示信息 -->
    <div v-if="!errorMessage && emails.length === 0" class="mt-1 text-xs text-gray-500 dark:text-gray-400">
      输入邮件地址后按回车键添加
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

// Props
interface Props {
  modelValue: string[]
  placeholder?: string
  maxEmails?: number
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '输入邮件地址...',
  maxEmails: 10
})

// Emits
const emit = defineEmits<{
  'update:modelValue': [emails: string[]]
}>()

// State
const emails = ref<string[]>([...props.modelValue])
const inputValue = ref<string>('')
const errorMessage = ref<string>('')

// Watch for external changes
watch(() => props.modelValue, (newValue) => {
  emails.value = [...newValue]
}, { deep: true })

// Watch for internal changes and emit
watch(emails, (newEmails) => {
  emit('update:modelValue', [...newEmails])
}, { deep: true })

// 邮件地址验证
function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

// 添加邮件地址
function addEmail(email: string) {
  const trimmedEmail = email.trim().toLowerCase()

  if (!trimmedEmail) {
    return
  }

  if (!isValidEmail(trimmedEmail)) {
    errorMessage.value = '请输入有效的邮件地址'
    return
  }

  if (emails.value.includes(trimmedEmail)) {
    errorMessage.value = '该邮件地址已存在'
    return
  }

  if (emails.value.length >= props.maxEmails) {
    errorMessage.value = `最多只能添加 ${props.maxEmails} 个邮件地址`
    return
  }

  emails.value.push(trimmedEmail)
  inputValue.value = ''
  errorMessage.value = ''
}

// 移除邮件地址
function removeEmail(index: number) {
  emails.value.splice(index, 1)
  errorMessage.value = ''
}

// 处理键盘事件
function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter') {
    event.preventDefault()
    addEmail(inputValue.value)
  } else if (event.key === 'Backspace' && !inputValue.value && emails.value.length > 0) {
    // 如果输入框为空且按下退格键，删除最后一个邮件地址
    emails.value.pop()
  }
}

// 处理失焦事件
function handleBlur() {
  if (inputValue.value.trim()) {
    addEmail(inputValue.value)
  }
}

// 清除错误信息
watch(inputValue, () => {
  if (errorMessage.value) {
    errorMessage.value = ''
  }
})
</script>

<style scoped>
.email-tags-input input::placeholder {
  color: rgb(107, 114, 128);
}

.dark .email-tags-input input::placeholder {
  color: rgb(156, 163, 175);
}
</style>
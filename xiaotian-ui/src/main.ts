import './assets/main.css'

import { debugEnvVars } from '@/utils/env'
import { createPinia } from 'pinia'
import { createApp } from 'vue'

import App from './App.vue'

// 输出调试信息
console.log('🚀 启动应用...')
debugEnvVars()

const app = createApp(App)

app.use(createPinia())

app.mount('#app')

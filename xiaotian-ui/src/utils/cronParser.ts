import type { CronParseResult } from '@/types'

// 简单的 cron 表达式解析器
export function parseCronExpression(cronExpression: string): CronParseResult {
  if (!cronExpression || typeof cronExpression !== 'string') {
    return {
      isValid: false,
      description: '',
      error: 'Cron 表达式不能为空'
    }
  }

  const expression = cronExpression.trim()
  const parts = expression.split(/\s+/)

  // 基本格式验证 (分 时 日 月 周)
  if (parts.length !== 5) {
    return {
      isValid: false,
      description: '',
      error: 'Cron 表达式格式错误，应为 5 个字段：分 时 日 月 周'
    }
  }

  const [minute, hour, day, month, weekday] = parts

  try {
    const description = generateCronDescription(minute, hour, day, month, weekday)
    const nextRun = calculateNextRun(minute, hour, day, month, weekday)

    return {
      isValid: true,
      description,
      nextRun
    }
  } catch (error) {
    return {
      isValid: false,
      description: '',
      error: error instanceof Error ? error.message : '解析错误'
    }
  }
}

function generateCronDescription(minute: string, hour: string, day: string, month: string, weekday: string): string {
  const descriptions: string[] = []

  // 处理分钟
  if (minute === '*') {
    descriptions.push('每分钟')
  } else if (minute.includes('/')) {
    const interval = minute.split('/')[1]
    descriptions.push(`每 ${interval} 分钟`)
  } else if (minute.includes(',')) {
    descriptions.push(`在第 ${minute.replace(',', '、')} 分钟`)
  } else if (minute.includes('-')) {
    descriptions.push(`在第 ${minute} 分钟`)
  } else {
    descriptions.push(`在第 ${minute} 分钟`)
  }

  // 处理小时
  if (hour === '*') {
    if (minute === '*') {
      descriptions[0] = '每分钟'
    }
  } else if (hour.includes('/')) {
    const interval = hour.split('/')[1]
    descriptions.push(`每 ${interval} 小时`)
  } else if (hour.includes(',')) {
    descriptions.push(`在 ${hour.replace(',', '、')} 点`)
  } else if (hour.includes('-')) {
    descriptions.push(`在 ${hour} 点`)
  } else {
    descriptions.push(`在 ${hour} 点`)
  }

  // 处理日期
  if (day === '*') {
    // 每天
  } else if (day.includes('/')) {
    const interval = day.split('/')[1]
    descriptions.push(`每 ${interval} 天`)
  } else if (day.includes(',')) {
    descriptions.push(`在每月第 ${day.replace(',', '、')} 日`)
  } else if (day.includes('-')) {
    descriptions.push(`在每月第 ${day} 日`)
  } else {
    descriptions.push(`在每月第 ${day} 日`)
  }

  // 处理月份
  if (month !== '*') {
    const monthNames = ['一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '十一', '十二']
    if (month.includes(',')) {
      const months = month.split(',').map(m => monthNames[parseInt(m) - 1]).join('、')
      descriptions.push(`在 ${months} 月`)
    } else if (month.includes('-')) {
      const [start, end] = month.split('-')
      descriptions.push(`在 ${monthNames[parseInt(start) - 1]} 到 ${monthNames[parseInt(end) - 1]} 月`)
    } else {
      descriptions.push(`在 ${monthNames[parseInt(month) - 1]} 月`)
    }
  }

  // 处理星期
  if (weekday !== '*') {
    const weekNames = ['日', '一', '二', '三', '四', '五', '六']
    if (weekday.includes(',')) {
      const weeks = weekday.split(',').map(w => `周${weekNames[parseInt(w)]}`).join('、')
      descriptions.push(`在 ${weeks}`)
    } else if (weekday.includes('-')) {
      const [start, end] = weekday.split('-')
      descriptions.push(`在周${weekNames[parseInt(start)]}到周${weekNames[parseInt(end)]}`)
    } else {
      descriptions.push(`在周${weekNames[parseInt(weekday)]}`)
    }
  }

  // 生成最终描述
  return descriptions.join('，')
}

function calculateNextRun(minute: string, hour: string, day: string, month: string, weekday: string): Date {
  const now = new Date()
  const nextRun = new Date(now)

  // 简化的下次运行时间计算
  // 这里只处理一些常见的情况
  if (minute !== '*' && hour !== '*' && day === '*' && month === '*' && weekday === '*') {
    // 每天的特定时间
    nextRun.setHours(parseInt(hour))
    nextRun.setMinutes(parseInt(minute))
    nextRun.setSeconds(0)
    nextRun.setMilliseconds(0)

    if (nextRun <= now) {
      nextRun.setDate(nextRun.getDate() + 1)
    }
  } else {
    // 对于复杂情况，简单地加1小时
    nextRun.setTime(now.getTime() + 60 * 60 * 1000)
  }

  return nextRun
}

// 预设的常用 cron 表达式
export const PRESET_CRON_EXPRESSIONS = [
  { expression: '0 9 * * *', description: '每天上午9点' },
  { expression: '0 18 * * *', description: '每天下午6点' },
  { expression: '0 9 * * 1', description: '每周一上午9点' },
  { expression: '0 9 1 * *', description: '每月1号上午9点' },
  { expression: '*/30 * * * *', description: '每30分钟' },
  { expression: '0 */6 * * *', description: '每6小时' },
  { expression: '0 9 * * 1-5', description: '工作日上午9点' },
  { expression: '0 20 * * 0', description: '每周日晚上8点' }
]
<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue'
import type { QuotaInfo } from '../types/usage'

const props = defineProps<{
  quota: QuotaInfo
  vendorColor: 'zhipu' | 'kimi'
}>()

const fillClass = computed(() => props.vendorColor)

const percentageDisplay = computed(() => {
  const pct = props.quota.percentage
  return pct >= 100 ? '100%' : `${pct.toFixed(1)}%`
})

const detailDisplay = computed(() => {
  if (props.quota.quota_type === 'mcp_monthly') {
    const used = props.quota.used >= 1000 ? props.quota.used.toLocaleString() : String(Math.round(props.quota.used))
    const total = props.quota.total >= 1000 ? props.quota.total.toLocaleString() : String(Math.round(props.quota.total))
    return `${used} / ${total} 次`
  }
  return null
})

const resetCountdown = computed(() => {
  if (!props.quota.reset_time || props.quota.quota_type === 'mcp_monthly') return null
  const resetDate = new Date(props.quota.reset_time)
  const diffMs = resetDate.getTime() - now.value
  if (diffMs <= 0) return '即将刷新'

  const pad = (n: number) => String(n).padStart(2, '0')
  const dateStr = `${resetDate.getFullYear()}-${pad(resetDate.getMonth() + 1)}-${pad(resetDate.getDate())} ${pad(resetDate.getHours())}:${pad(resetDate.getMinutes())}`

  const hours = Math.floor(diffMs / 3600000)
  const minutes = Math.floor((diffMs % 3600000) / 60000)
  let duration: string
  if (hours > 0) {
    duration = `${hours}小时${minutes}分钟`
  } else {
    duration = `${minutes}分钟`
  }

  return `${dateStr}（${duration}后刷新）`
})

const now = ref(Date.now())
let timer: ReturnType<typeof setInterval> | null = null

onMounted(() => {
  timer = setInterval(() => { now.value = Date.now() }, 60000)
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})
</script>

<template>
  <div class="quota-row">
    <div class="quota-label-row">
      <span class="quota-label">{{ quota.label }}</span>
      <span class="quota-value tabular-nums">
        <strong>{{ percentageDisplay }}</strong>
        <span v-if="detailDisplay" class="quota-detail">{{ detailDisplay }}</span>
      </span>
    </div>
    <div class="progress-track">
      <div
        class="progress-fill"
        :class="fillClass"
        :style="{ width: Math.min(quota.percentage, 100) + '%' }"
      />
    </div>
    <div v-if="resetCountdown" class="reset-hint" :class="vendorColor">
      {{ resetCountdown }}
    </div>
  </div>
</template>

<style scoped>
.quota-row {
  margin-bottom: 14px;
}

.quota-row:last-child {
  margin-bottom: 0;
}

.quota-label-row {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 6px;
}

.quota-label {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.quota-value {
  font-size: 12px;
  color: var(--color-text-tertiary);
}

.quota-value strong {
  color: var(--color-text-primary);
  font-weight: 600;
}

.quota-detail {
  margin-left: 6px;
  font-weight: 400;
}

.progress-track {
  width: 100%;
  height: 6px;
  background: var(--color-border);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.6s ease;
}

.progress-fill.zhipu {
  background: linear-gradient(90deg, #4f6ef7, #7b93f9);
}

.progress-fill.kimi {
  background: linear-gradient(90deg, #6c5ce7, #a855f7);
}

.reset-hint {
  font-size: 11px;
  font-weight: 500;
  margin-top: 4px;
}

.reset-hint.zhipu {
  color: var(--color-zhipu);
}

.reset-hint.kimi {
  color: var(--color-kimi);
}
</style>

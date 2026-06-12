<script setup lang="ts">
import { computed } from 'vue'
import type { UsageData } from '../types/usage'
import QuotaBar from './QuotaBar.vue'

const props = defineProps<{
  data: UsageData
}>()

const vendorClass = computed(() => props.data.vendor_id === 'zhipu' ? 'zhipu' : 'kimi')

const vendorInitial = computed(() => {
  if (props.data.vendor_id === 'zhipu') return '智'
  return 'K'
})

const planBadgeClass = computed(() => {
  const level = props.data.plan_level.toLowerCase()
  if (level === 'pro' || level === 'lite' || level === 'max') return 'zhipu-badge'
  return 'kimi-badge'
})

const planLabel = computed(() => {
  const level = props.data.plan_level
  return level.charAt(0).toUpperCase() + level.slice(1)
})
</script>

<template>
  <div class="vendor-card" :class="vendorClass">
    <div v-if="data.is_error" class="error-banner">
      {{ data.error_message || '数据获取失败' }}
    </div>

    <div class="vendor-header">
      <div class="vendor-info">
        <div class="vendor-icon" :class="vendorClass">{{ vendorInitial }}</div>
        <div>
          <div class="vendor-name">{{ data.vendor_name }}</div>
          <div class="vendor-plan-sub">GLM-5.1 / GLM-4.7 / GLM-4.5-Air</div>
        </div>
      </div>
      <span class="plan-badge" :class="planBadgeClass">{{ planLabel }}</span>
    </div>

    <QuotaBar
      v-for="quota in data.quotas"
      :key="quota.quota_type"
      :quota="quota"
      :vendor-color="vendorClass"
    />
  </div>
</template>

<style scoped>
.vendor-card {
  margin: 16px;
  padding: 18px;
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
  transition: box-shadow 0.2s;
}

.vendor-card:hover {
  box-shadow: var(--shadow-md);
}

.vendor-card.zhipu {
  border-left: 3px solid var(--color-zhipu);
}

.vendor-card.kimi {
  border-left: 3px solid var(--color-kimi);
}

.error-banner {
  background: rgba(255, 59, 48, 0.1);
  color: var(--color-danger);
  font-size: 12px;
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  margin-bottom: 12px;
}

.vendor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 14px;
}

.vendor-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.vendor-icon {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: 700;
  color: white;
}

.vendor-icon.zhipu {
  background: linear-gradient(135deg, #4f6ef7, #3b5de7);
}

.vendor-icon.kimi {
  background: linear-gradient(135deg, #6c5ce7, #a855f7);
}

.vendor-name {
  font-size: 15px;
  font-weight: 600;
}

.vendor-plan-sub {
  font-size: 11px;
  color: var(--color-text-tertiary);
  margin-top: 1px;
}

.plan-badge {
  padding: 3px 10px;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 600;
}

.plan-badge.zhipu-badge {
  background: var(--color-zhipu-light);
  color: var(--color-zhipu);
}

.plan-badge.kimi-badge {
  background: var(--color-kimi-light);
  color: var(--color-kimi);
}
</style>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useUsageStore } from '../stores/usage'
import VendorCard from '../components/VendorCard.vue'

const store = useUsageStore()

onMounted(() => {
  store.init()
})

const statusDotClass = computed(() => {
  if (store.isRefreshing) return 'refreshing'
  if (store.usageData.some(d => d.is_error)) return 'offline'
  return 'online'
})

const timeAgo = computed(() => {
  if (!store.lastUpdateTime) return ''
  const diff = Date.now() - new Date(store.lastUpdateTime).getTime()
  const minutes = Math.floor(diff / 60000)
  if (minutes < 1) return '刚刚更新'
  if (minutes < 60) return `${minutes} min ago`
  const hours = Math.floor(minutes / 60)
  return `${hours}h ${minutes % 60}m ago`
})

const refreshIntervalLabel = computed(() => {
  if (!store.config) return '5'
  const mins = store.config.refresh_interval_secs / 60
  return String(mins)
})

async function handleRefresh() {
  await store.refreshUsage()
}
</script>

<template>
  <div class="dashboard">
    <header class="app-header">
      <div class="header-left">
        <div class="app-title">
          <span class="status-dot" :class="statusDotClass" />
          Model Usage
        </div>
        <div class="app-subtitle">{{ timeAgo || 'Loading...' }}</div>
      </div>
      <div class="header-actions">
        <button class="settings-btn" title="Settings" @click="store.showSettings = true">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
        </button>
        <button
          class="refresh-btn"
          :class="{ spinning: store.isRefreshing }"
          title="Refresh"
          @click="handleRefresh"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#636366" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="23 4 23 10 17 10" />
            <polyline points="1 20 1 14 7 14" />
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
          </svg>
        </button>
      </div>
    </header>

    <div v-if="!store.hasAnyConfig && !store.isLoading" class="setup-prompt">
      <div class="setup-icon">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3" />
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
        </svg>
      </div>
      <h3 class="setup-title">Get Started</h3>
      <p class="setup-desc">Configure your API keys to start monitoring model usage.</p>
      <button class="setup-btn" @click="store.showSettings = true">Open Settings</button>
    </div>

    <div v-else class="cards-list">
      <VendorCard
        v-for="card in store.vendorCards"
        :key="card.vendor_id"
        :data="card"
      />
    </div>

    <footer class="app-footer">
      <span class="footer-text">Auto refresh every {{ refreshIntervalLabel }} min</span>
    </footer>
  </div>
</template>

<style scoped>
.dashboard {
  max-width: 420px;
  margin: 0 auto;
  background: var(--color-surface);
  min-height: 100vh;
}

.app-header {
  padding: 20px 20px 16px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  flex: 1;
}

.app-title {
  font-size: 18px;
  font-weight: 700;
  letter-spacing: -0.3px;
  display: flex;
  align-items: center;
  gap: 4px;
}

.app-subtitle {
  font-size: 12px;
  color: var(--color-text-tertiary);
  margin-top: 2px;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  display: inline-block;
  margin-right: 4px;
  flex-shrink: 0;
}

.status-dot.online {
  background: var(--color-success);
}

.status-dot.offline {
  background: var(--color-danger);
}

.status-dot.refreshing {
  background: var(--color-warning);
  animation: pulse 1s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

.header-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.settings-btn,
.refresh-btn {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: var(--color-bg);
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s;
  color: var(--color-text-secondary);
}

.settings-btn:hover,
.refresh-btn:hover {
  background: var(--color-border-strong);
}

.refresh-btn.spinning svg {
  animation: spin 0.6s linear;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.setup-prompt {
  padding: 48px 32px;
  text-align: center;
}

.setup-icon {
  color: var(--color-text-tertiary);
  margin-bottom: 16px;
}

.setup-title {
  font-size: 18px;
  font-weight: 700;
  margin-bottom: 8px;
}

.setup-desc {
  font-size: 13px;
  color: var(--color-text-tertiary);
  margin-bottom: 20px;
  line-height: 1.5;
}

.setup-btn {
  padding: 10px 24px;
  border-radius: var(--radius-sm);
  border: none;
  background: var(--color-zhipu);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}

.setup-btn:hover {
  opacity: 0.9;
}

.cards-list {
  padding-bottom: 0;
}

.app-footer {
  padding: 12px 20px;
  text-align: center;
  border-top: 1px solid var(--color-border);
}

.footer-text {
  font-size: 11px;
  color: var(--color-text-tertiary);
}
</style>

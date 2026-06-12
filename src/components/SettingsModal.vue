<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUsageStore } from '../stores/usage'
import type { AppConfig } from '../types/usage'

const emit = defineEmits<{
  close: []
}>()

const store = useUsageStore()

const zhipuKey = ref('')
const kimiKey = ref('')
const refreshInterval = ref(300)
const notificationThreshold = ref(20)
const showZhipuKey = ref(false)
const showKimiKey = ref(false)
const isSaving = ref(false)

onMounted(() => {
  if (store.config) {
    zhipuKey.value = store.config.zhipu_api_key
    kimiKey.value = store.config.kimi_api_key
    refreshInterval.value = store.config.refresh_interval_secs
    notificationThreshold.value = store.config.notification_threshold
  }
})

async function handleSave() {
  isSaving.value = true
  const newConfig: AppConfig = {
    zhipu_api_key: zhipuKey.value,
    kimi_api_key: kimiKey.value,
    refresh_interval_secs: refreshInterval.value,
    notification_threshold: notificationThreshold.value
  }
  await store.updateConfig(newConfig)
  isSaving.value = false
  emit('close')
}

function onOverlayClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains('modal-overlay')) {
    emit('close')
  }
}
</script>

<template>
  <div class="modal-overlay" @click="onOverlayClick">
    <div class="modal-card">
      <div class="modal-header">
        <h2 class="modal-title">Settings</h2>
        <button class="close-btn" @click="emit('close')">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <div class="section-label">API Keys</div>

        <div class="field-group">
          <label class="field-label">智谱 API Key</label>
          <div class="input-with-toggle">
            <input
              v-model="zhipuKey"
              :type="showZhipuKey ? 'text' : 'password'"
              class="field-input"
              placeholder="输入智谱 API Key..."
            />
            <button class="toggle-visibility" @click="showZhipuKey = !showZhipuKey">
              <svg v-if="!showZhipuKey" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
                <circle cx="12" cy="12" r="3" />
              </svg>
              <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" />
                <line x1="1" y1="1" x2="23" y2="23" />
              </svg>
            </button>
          </div>
        </div>

        <div class="field-group">
          <label class="field-label">Kimi API Key</label>
          <div class="input-with-toggle">
            <input
              v-model="kimiKey"
              :type="showKimiKey ? 'text' : 'password'"
              class="field-input"
              placeholder="输入 Kimi API Key..."
            />
            <button class="toggle-visibility" @click="showKimiKey = !showKimiKey">
              <svg v-if="!showKimiKey" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
                <circle cx="12" cy="12" r="3" />
              </svg>
              <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" />
                <line x1="1" y1="1" x2="23" y2="23" />
              </svg>
            </button>
          </div>
        </div>

        <div class="section-label" style="margin-top: 20px;">Preferences</div>

        <div class="field-group">
          <label class="field-label">Refresh Interval</label>
          <select v-model.number="refreshInterval" class="field-select">
            <option :value="60">1 minute</option>
            <option :value="180">3 minutes</option>
            <option :value="300">5 minutes</option>
            <option :value="600">10 minutes</option>
          </select>
        </div>

        <div class="field-group">
          <label class="field-label">Notification Threshold: {{ notificationThreshold }}%</label>
          <input
            v-model.number="notificationThreshold"
            type="range"
            min="10"
            max="50"
            step="5"
            class="field-slider"
          />
          <div class="slider-labels">
            <span>10%</span>
            <span>50%</span>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-save" :disabled="isSaving" @click="handleSave">
          {{ isSaving ? 'Saving...' : 'Save' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal-card {
  background: var(--color-surface);
  border-radius: var(--radius-xl);
  width: 90%;
  max-width: 380px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 18px 20px 14px;
  border-bottom: 1px solid var(--color-border);
}

.modal-title {
  font-size: 17px;
  font-weight: 700;
}

.close-btn {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: var(--color-bg);
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
  transition: background 0.2s;
}

.close-btn:hover {
  background: var(--color-border-strong);
}

.modal-body {
  padding: 16px 20px;
}

.section-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 10px;
}

.field-group {
  margin-bottom: 14px;
}

.field-label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 6px;
}

.input-with-toggle {
  position: relative;
  display: flex;
  align-items: center;
}

.field-input {
  width: 100%;
  padding: 9px 12px;
  padding-right: 40px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border-strong);
  background: var(--color-bg);
  color: var(--color-text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.2s;
}

.field-input:focus {
  border-color: var(--color-zhipu);
}

.field-input::placeholder {
  color: var(--color-text-tertiary);
}

.toggle-visibility {
  position: absolute;
  right: 8px;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-tertiary);
  padding: 4px;
  display: flex;
  align-items: center;
}

.toggle-visibility:hover {
  color: var(--color-text-secondary);
}

.field-select {
  width: 100%;
  padding: 9px 12px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border-strong);
  background: var(--color-bg);
  color: var(--color-text-primary);
  font-size: 13px;
  outline: none;
  cursor: pointer;
  transition: border-color 0.2s;
  -webkit-appearance: none;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2386868b' stroke-width='2.5' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 32px;
}

.field-select:focus {
  border-color: var(--color-zhipu);
}

.field-slider {
  width: 100%;
  -webkit-appearance: none;
  appearance: none;
  height: 4px;
  background: var(--color-border-strong);
  border-radius: 2px;
  outline: none;
  margin: 8px 0 4px;
}

.field-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--color-zhipu);
  cursor: pointer;
  border: 3px solid var(--color-surface);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.15);
}

.slider-labels {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: var(--color-text-tertiary);
}

.modal-footer {
  padding: 12px 20px 18px;
}

.btn-save {
  width: 100%;
  padding: 10px;
  border-radius: var(--radius-sm);
  border: none;
  background: var(--color-zhipu);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn-save:hover {
  opacity: 0.9;
}

.btn-save:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>

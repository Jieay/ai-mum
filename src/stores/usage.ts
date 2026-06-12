import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { UsageData, AppConfig } from '../types/usage'
import * as tauri from '../services/tauri'

export const useUsageStore = defineStore('usage', () => {
  const usageData = ref<UsageData[]>([])
  const config = ref<AppConfig | null>(null)
  const lastUpdateTime = ref<string | null>(null)
  const isLoading = ref(false)
  const isRefreshing = ref(false)
  const showSettings = ref(false)

  const hasAnyConfig = computed(() => {
    if (!config.value) return false
    return config.value.zhipu_api_key.length > 0 || config.value.kimi_api_key.length > 0
  })

  const vendorCards = computed(() => usageData.value)

  async function fetchUsage() {
    isLoading.value = true
    try {
      usageData.value = await tauri.getUsage()
      lastUpdateTime.value = new Date().toISOString()
    } catch (e) {
      console.error('Failed to fetch usage:', e)
    } finally {
      isLoading.value = false
    }
  }

  async function refreshUsage() {
    isRefreshing.value = true
    try {
      usageData.value = await tauri.refreshUsage()
      lastUpdateTime.value = new Date().toISOString()
    } catch (e) {
      console.error('Failed to refresh usage:', e)
    } finally {
      isRefreshing.value = false
    }
  }

  async function fetchConfig() {
    try {
      config.value = await tauri.getConfig()
    } catch (e) {
      console.error('Failed to fetch config:', e)
    }
  }

  async function updateConfig(newConfig: AppConfig) {
    try {
      await tauri.saveConfig(newConfig)
      config.value = { ...newConfig }
    } catch (e) {
      console.error('Failed to save config:', e)
    }
  }

  async function init() {
    await Promise.all([fetchUsage(), fetchConfig()])

    tauri.onUsageUpdated((data: UsageData[]) => {
      usageData.value = data
      lastUpdateTime.value = new Date().toISOString()
    })
  }

  return {
    usageData,
    config,
    lastUpdateTime,
    isLoading,
    isRefreshing,
    showSettings,
    hasAnyConfig,
    vendorCards,
    fetchUsage,
    refreshUsage,
    fetchConfig,
    updateConfig,
    init
  }
})

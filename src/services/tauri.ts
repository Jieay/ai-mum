import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { UsageData, AppConfig } from '../types/usage'

const isTauri = '__TAURI_INTERNALS__' in window

const mockUsageData: UsageData[] = [
  {
    vendor_id: 'zhipu',
    vendor_name: '智谱 GLM Coding Plan',
    plan_level: 'pro',
    quotas: [
      { quota_type: 'five_hour', label: '5 小时额度', used: 265, total: 400, remaining: 135, percentage: 33.75, reset_time: new Date(Date.now() + 2 * 3600000 + 15 * 60000).toISOString() },
      { quota_type: 'weekly', label: '每周额度', used: 1120, total: 2000, remaining: 880, percentage: 56, reset_time: null },
      { quota_type: 'mcp_monthly', label: 'MCP 工具调用（月）', used: 328, total: 1000, remaining: 672, percentage: 32.8, reset_time: null }
    ],
    last_updated: new Date().toISOString(),
    is_error: false,
    error_message: null
  },
  {
    vendor_id: 'kimi',
    vendor_name: 'Kimi Code',
    plan_level: 'andante',
    quotas: [
      { quota_type: 'five_hour', label: '5 小时额度', used: 486, total: 800, remaining: 314, percentage: 60.75, reset_time: new Date(Date.now() + 1 * 3600000 + 42 * 60000).toISOString() },
      { quota_type: 'weekly', label: '每周额度', used: 2150, total: 5600, remaining: 3450, percentage: 38.4, reset_time: null }
    ],
    last_updated: new Date().toISOString(),
    is_error: false,
    error_message: null
  }
]

const mockConfig: AppConfig = {
  zhipu_api_key: '',
  kimi_api_key: '',
  refresh_interval_secs: 300,
  notification_threshold: 20
}

export async function getUsage(): Promise<UsageData[]> {
  if (!isTauri) return mockUsageData
  return invoke<UsageData[]>('get_usage')
}

export async function refreshUsage(): Promise<UsageData[]> {
  if (!isTauri) return mockUsageData
  return invoke<UsageData[]>('refresh_usage')
}

export async function getConfig(): Promise<AppConfig> {
  if (!isTauri) return { ...mockConfig }
  return invoke<AppConfig>('get_config')
}

export async function saveConfig(config: AppConfig): Promise<void> {
  if (!isTauri) return
  return invoke<void>('save_config', { config })
}

export async function getLastUpdateTime(): Promise<string | null> {
  if (!isTauri) return new Date().toISOString()
  return invoke<string | null>('get_last_update_time')
}

export async function onUsageUpdated(callback: (data: UsageData[]) => void): Promise<() => void> {
  if (!isTauri) return () => {}
  const unlisten = await listen<UsageData[]>('usage-updated', (event) => {
    callback(event.payload)
  })
  return unlisten
}

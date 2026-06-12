export interface UsageData {
  vendor_id: string
  vendor_name: string
  plan_level: string
  quotas: QuotaInfo[]
  last_updated: string
  is_error: boolean
  error_message: string | null
}

export interface QuotaInfo {
  quota_type: string
  label: string
  used: number
  total: number
  remaining: number
  percentage: number
  reset_time: string | null
}

export interface AppConfig {
  zhipu_api_key: string
  kimi_api_key: string
  refresh_interval_secs: number
  notification_threshold: number
}

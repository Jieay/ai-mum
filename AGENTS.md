# AGENTS.md

AI Agent 开发指南。阅读本文档即可理解项目全貌，无需逐文件探索。

## 项目简介

macOS 桌面工具，用卡片式仪表盘展示大模型编程套餐（智谱 GLM Coding Plan、Kimi Code）的实时用量额度。

技术栈：**Tauri v2 + Vue 3 + Rust + TypeScript**

## 架构

```
┌─────────────────────────────────────┐
│           macOS System              │
│  System Tray ◄── Left Click: Toggle│
│  Notification ◄── Low Quota Alert  │
└──────────┬──────────────────────────┘
           │
┌──────────▼──────────────────────────┐
│           Tauri Runtime             │
│  ┌────────────────────────────────┐ │
│  │  Rust Backend                  │ │
│  │  vendors/zhipu.rs ──HTTP──► 智谱API│
│  │  vendors/kimi.rs ──HTTP──► Kimi API│
│  │  scheduler.rs ──5min poll─────► │ │
│  │  cache.rs ◄── ~/.ai-usage-monitor/│
│  │  commands/* ◄── Tauri Commands──│ │
│  └──────────────┬─────────────────┘ │
│                 │ Tauri Events       │
│  ┌──────────────▼─────────────────┐ │
│  │  Vue 3 Frontend (WebView)      │ │
│  │  Dashboard → VendorCard → QuotaBar│
│  │  Pinia Store ← tauri.ts        │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

**数据流**：Rust 定时拉取 API → 缓存到内存+文件 → emit Tauri Event → Vue Pinia store 更新 → UI 响应。

## 目录结构

```
├── src/                          # Vue 3 前端（TypeScript）
│   ├── App.vue                   # 根组件，挂载 Dashboard + SettingsModal
│   ├── main.ts                   # 入口，注册 Pinia，导入样式
│   ├── views/
│   │   └── Dashboard.vue         # 主面板：标题栏 + 厂商卡片列表 + 底栏
│   ├── components/
│   │   ├── VendorCard.vue        # 单个厂商卡片（图标/名称/计划标签/额度条）
│   │   ├── QuotaBar.vue          # 进度条组件（百分比 + 进度条 + 重置倒计时）
│   │   └── SettingsModal.vue     # 设置弹窗（API Key / 刷新频率 / 通知阈值）
│   ├── stores/
│   │   └── usage.ts              # Pinia store（usageData / config / actions）
│   ├── services/
│   │   └── tauri.ts              # Tauri invoke 封装 + 浏览器 mock 数据
│   ├── types/
│   │   └── usage.ts              # TypeScript 接口定义
│   └── styles/
│       ├── variables.css         # CSS 变量（含深色模式）
│       └── base.css              # 全局样式重置
│
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml                # Rust 依赖
│   ├── tauri.conf.json           # Tauri 配置（窗口/托盘/打包）
│   ├── capabilities/default.json # Tauri 权限
│   └── src/
│       ├── main.rs               # 程序入口
│       ├── lib.rs                # 核心注册（托盘菜单、命令注册、调度器启动）
│       ├── models.rs             # 数据结构（UsageData / QuotaInfo / AppConfig / AppState）
│       ├── cache.rs              # 文件缓存（~/.ai-usage-monitor/cache.json）
│       ├── scheduler.rs          # 后台定时刷新 + do_refresh 逻辑
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── usage.rs          # get_usage / refresh_usage / get_last_update_time
│       │   └── config.rs         # get_config / save_config + 文件 I/O
│       └── vendors/
│           ├── mod.rs            # Vendor trait 定义
│           ├── kimi.rs           # Kimi Code API 对接
│           └── zhipu.rs          # 智谱 API 对接（动态 JSON 解析）
│
├── Makefile                      # 快捷命令
├── PLAN.md                       # 项目规划文档
├── docs/ui-preview.html          # UI 设计预览
└── README.md                     # 用户文档
```

## 核心数据模型

前后端共享的数据结构，修改时需同步两端：

### Rust（`src-tauri/src/models.rs`）

```rust
struct UsageData {
    vendor_id: String,        // "zhipu" | "kimi"
    vendor_name: String,
    plan_level: String,       // "pro" | "lite" | "max" | "andante"
    quotas: Vec<QuotaInfo>,
    last_updated: String,     // ISO 8601
    is_error: bool,
    error_message: Option<String>,
}

struct QuotaInfo {
    quota_type: String,       // "5hour" | "weekly" | "mcp_monthly"
    label: String,
    used: f64,
    total: f64,
    remaining: f64,
    percentage: f64,          // 0.0 - 100.0
    reset_time: Option<String>, // ISO 8601，MCP 无此字段
}

struct AppConfig {
    zhipu_api_key: String,
    kimi_api_key: String,
    refresh_interval_secs: u64,     // 默认 300
    notification_threshold: f64,     // 默认 20.0
}
```

### TypeScript（`src/types/usage.ts`）

字段名和类型与 Rust 侧一一对应，Rust 用 snake_case，前端自动接收 camelCase（Tauri 默认转换）。

## Tauri 命令接口

前端通过 `invoke('command_name', { args })` 调用 Rust：

| 命令 | 参数 | 返回值 | 说明 |
|---|---|---|---|
| `get_usage` | 无 | `Vec<UsageData>` | 返回缓存的用量数据 |
| `refresh_usage` | 无 | `Vec<UsageData>` | 强制从 API 拉取并返回 |
| `get_config` | 无 | `AppConfig` | 获取用户配置 |
| `save_config` | `config: AppConfig` | `()` | 保存配置到文件 |
| `get_last_update_time` | 无 | `Option<String>` | 上次成功刷新时间 |

后台事件：
- `usage-updated` — 调度器每次刷新后 emit，payload 为 `Vec<UsageData>`

## 前端调用约定

```typescript
// 所有调用封装在 src/services/tauri.ts
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// 浏览器开发模式（非 Tauri 环境）自动使用 mock 数据
const isTauri = '__TAURI_INTERNALS__' in window
```

## 智谱 API

```
GET https://open.bigmodel.cn/api/monitor/usage/quota/limit
Authorization: {api_key}
```

响应使用 `serde_json::Value` 动态解析，字段通过 `.get("fieldName")` 提取，避免类型不匹配导致解析失败。

关键映射：
- `TOKENS_LIMIT`（按 `nextResetTime` 排序）→ 第一个 = 5 小时额度，第二个 = 每周额度
- `TIME_LIMIT` → MCP 月额度（`currentValue` = 已用，`usage` = 总量，`remaining` = 剩余）
- `nextResetTime` 为毫秒时间戳（i64），转为 ISO 8601 字符串传给前端

## Kimi Code API

```
GET https://api.kimi.com/coding/v1/usages
Authorization: Bearer {api_key}
```

关键映射：
- `usage` → 每周额度（`limit` / `used` / `remaining` / `resetTime`）
- `limits` 中 `window.duration = 300` 且 `window.timeUnit = TIME_UNIT_MINUTE` 的项 → 5 小时额度
- `user.membership.level` → 套餐等级，`LEVEL_INTERMEDIATE` 映射为 `allegretto`

## 用户数据存储

```
~/.ai-usage-monitor/
├── config.json    # AppConfig（含 API Key）
└── cache.json     # Vec<UsageData> 上次拉取的缓存
```

## 编码规范

- **不加注释** — 代码应自解释
- **不加 doc comments** — 通过命名传达意图
- **不用 `unwrap()`** — 用 `map_err` 或 `?` 处理错误
- **不用 `any`** — TypeScript 必须有明确类型
- **不用 `@ts-ignore`** — 修复类型错误而非压制
- **不用 UI 框架** — 纯 CSS（CSS Variables），不用 Tailwind/Element Plus
- **不打印 API Key** — 安全红线
- **Rust 数据目录** — 使用 `std::env::var("HOME")` 拼接 `~/.ai-usage-monitor/`

## 常用开发命令

```bash
make install-deps    # 安装依赖
make dev             # 开发模式（热重载）
make check           # 编译检查（Rust + TypeScript）
make build           # 正式构建
make dist            # 构建并显示产物
make clean           # 清理
```

## 新增厂商步骤

1. **Rust 端**：
   - `src-tauri/src/vendors/` 下新建 `kimi.rs`
   - 实现 `Vendor` trait（`id` / `name` / `fetch_usage`）
   - 在 `vendors/mod.rs` 添加 `pub mod kimi`
   - 在 `scheduler.rs` 的 `do_refresh` 中添加该厂商的 fetch 逻辑

2. **前端**：
   - `src/services/tauri.ts` 添加 mock 数据（用于浏览器开发）
   - `src/components/VendorCard.vue` 已支持任意 `vendor_id`，无需修改
   - 如需新品牌色，在 `src/styles/variables.css` 和组件中添加对应 CSS 变量

## 已知限制

- **iOS**：需要 Apple Developer 账号才能部署到真机
- **DMG 打包**：需额外安装 `brew install create-dmg`
- **无签名**：用户首次打开需右键→「打开」绕过 Gatekeeper

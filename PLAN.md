# AI Model Usage Monitor - 项目规划文档

## 1. 项目概述

### 1.1 项目名称

AI Model Usage Monitor（暂定名：ModelLens）

### 1.2 项目目标

开发一款轻量级桌面工具，实时展示主流大模型厂商编程套餐的用量额度，帮助开发者直观掌握各厂商套餐消耗情况，避免额度耗尽导致工作中断。

### 1.3 目标用户

- 使用智谱 GLM Coding Plan 的开发者
- 使用 Kimi Code 套餐的开发者
- 同时订阅多个厂商套餐、需要集中监控用量的用户

### 1.4 核心约束

| 约束 | 要求 |
|---|---|
| 资源消耗 | 内存占用 < 50MB，CPU 空闲时 < 1% |
| 平台优先级 | Mac 优先 → 后续 Android → iOS（需开发者账号） |
| 安装包体积 | < 10MB |
| 无需开发者账号 | Mac 端通过 .dmg 直接分发 |

---

## 2. 技术方案

### 2.1 技术栈

| 层级 | 技术选型 | 版本 |
|---|---|---|
| 桌面框架 | Tauri v2 | 2.x |
| 后端语言 | Rust | 1.80+ |
| 前端框架 | Vue 3 | 3.5+ |
| 构建工具 | Vite | 6.x |
| 状态管理 | Pinia | 2.x |
| 样式方案 | 原生 CSS（CSS Variables） | - |
| 类型系统 | TypeScript | 5.x |

### 2.2 架构设计

```
┌─────────────────────────────────────────┐
│              macOS System               │
│  ┌───────────┐  ┌───────────────────┐   │
│  │ System    │  │  Notification     │   │
│  │ Tray Icon │  │  Center           │   │
│  └─────┬─────┘  └───────▲───────────┘   │
│        │                │               │
│  ┌─────▼─────────────────────────────┐  │
│  │         Tauri Runtime             │  │
│  │  ┌─────────────────────────────┐  │  │
│  │  │     Rust Backend            │  │  │
│  │  │  ┌───────┐  ┌───────────┐  │  │  │
│  │  │  │HTTP   │  │Scheduler  │  │  │  │
│  │  │  │Client │  │(5min poll)│  │  │  │
│  │  │  └───┬───┘  └─────┬─────┘  │  │  │
│  │  │      │            │        │  │  │
│  │  │  ┌───▼────────────▼─────┐  │  │  │
│  │  │  │    Data Cache        │  │  │  │
│  │  │  │  (in-memory + file)  │  │  │  │
│  │  │  └──────────┬───────────┘  │  │  │
│  │  └─────────────┼──────────────┘  │  │
│  │                │ Tauri Commands   │  │
│  │  ┌─────────────▼──────────────┐  │  │
│  │  │     Vue 3 Frontend         │  │  │
│  │  │  ┌───────┐  ┌───────────┐  │  │  │
│  │  │  │Cards  │  │  Pinia    │  │  │  │
│  │  │  │Layout │  │  Store    │  │  │  │
│  │  │  └───────┘  └───────────┘  │  │  │
│  │  └────────────────────────────┘  │  │
│  └──────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

### 2.3 项目目录结构

```
oh-my-aimodelusage/
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Tauri 配置
│   ├── capabilities/
│   │   └── default.json          # 权限配置
│   ├── icons/                    # 应用图标
│   └── src/
│       ├── main.rs               # 入口（系统托盘、窗口管理）
│       ├── lib.rs                # Tauri 命令注册
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── usage.rs          # 用量查询命令
│       │   └── config.rs         # 配置管理命令
│       ├── vendors/
│       │   ├── mod.rs
│       │   ├── zhipu.rs          # 智谱 API 对接
│       │   └── kimi.rs           # Kimi API 对接
│       ├── scheduler.rs          # 定时刷新调度
│       ├── cache.rs              # 数据缓存
│       └── config.rs             # 用户配置（API Key 存储）
│
├── src/                          # Vue 3 前端
│   ├── App.vue
│   ├── main.ts
│   ├── views/
│   │   └── Dashboard.vue         # 主面板
│   ├── components/
│   │   ├── VendorCard.vue        # 厂商卡片
│   │   ├── QuotaBar.vue          # 额度进度条
│   │   └── SettingsModal.vue     # 设置弹窗
│   ├── stores/
│   │   └── usage.ts              # Pinia 用量状态
│   ├── services/
│   │   └── tauri.ts              # Tauri 命令调用封装
│   ├── types/
│   │   └── usage.ts              # 类型定义
│   └── styles/
│       ├── variables.css          # CSS 变量（主题）
│       └── base.css               # 基础样式
│
├── docs/
│   └── ui-preview.html           # UI 预览
├── package.json
├── vite.config.ts
├── tsconfig.json
└── PLAN.md                       # 本文档
```

---

## 3. 数据源调研

### 3.1 智谱 GLM Coding Plan

#### API 信息（已验证）

| 项目 | 详情 |
|---|---|
| 国内版端点 | `GET https://open.bigmodel.cn/api/monitor/usage/quota/limit` |
| 国际版端点 | `GET https://api.z.ai/api/monitor/usage/quota/limit` |
| 认证方式 | `Authorization: <API_TOKEN>`（即 API Key） |
| 响应格式 | JSON |

#### 响应数据结构

```json
{
  "code": 200,
  "msg": "操作成功",
  "data": {
    "limits": [
      {
        "type": "TIME_LIMIT",
        "percentage": 7,
        "usage": 1000,
        "currentValue": 72,
        "remaining": 928
      },
      {
        "type": "TOKENS_LIMIT",
        "percentage": 44
      },
      {
        "type": "TOKENS_LIMIT",
        "percentage": 53
      }
    ],
    "level": "pro"
  },
  "success": true
}
```

#### 数据字段解析

| 字段 | 含义 |
|---|---|
| `limits` 中 `TOKENS_LIMIT` | 按 `nextResetTime` 排序，第一个为 **5 小时额度**，第二个为 **每周额度** |
| `limits` 中 `TIME_LIMIT` | MCP 工具每月调用额度（usage=总额, currentValue=已用, remaining=剩余） |
| `percentage` | 已使用百分比（0-100） |
| `level` | 套餐等级：`lite` / `pro` / `max` |

#### 套餐额度参考

| 套餐 | 每 5 小时 | 每周 |
|---|---|---|
| Lite（¥20/月） | ~80 prompts | ~400 prompts |
| Pro（¥100/月） | ~400 prompts | ~2,000 prompts |
| Max | ~1,600 prompts | ~8,000 prompts |

#### 数据获取方案

**直接调用 API** — 使用用户配置的 API Key 直接请求，无需登录态/Cookie。

---

### 3.2 Kimi Code

#### 官方信息

| 项目 | 详情 |
|---|---|
| 套餐页面 | https://www.kimi.com/code/docs/ |
| API Base URL | `https://api.kimi.com/coding/v1`（OpenAI 兼容） |
| 模型 ID | `kimi-for-coding` |
| 认证方式 | API Key 或 OAuth |
| 控制台 | Kimi Code 控制台可查看剩余额度与频限状态 |

#### 套餐额度参考

| 套餐 | 每 5 小时 | 每周 | 并发 |
|---|---|---|---|
| Andante（¥49/月） | ~300-1,200 请求 | ~5,600 请求 | 30 |
| Moderato（¥99/月） | 4x Andante | 4x Andante | 更高 |
| Allegretto（¥199/月） | 更高 | 更高 | 更高 |

#### 数据获取方案

**⚠️ Kimi 暂无公开的用量查询 API。** 可行方案按优先级：

1. **抓包分析控制台接口**（推荐）— 访问 Kimi Code 控制台时抓取 XHR 请求，找到用量查询的实际 API 端点，然后通过 API Key 认证直接调用
2. **Cookie 登录态抓取** — 通过用户登录后的 Cookie 调用控制台内部接口
3. **等待官方 API** — 已有 GitHub Issue 请求（MoonshotAI/Kimi-K2.5#16），持续关注

> **实现建议**：先完成智谱的数据对接（有明确 API），Kimi 部分预留接口，待抓包确认后补充。

---

## 4. 功能设计

### 4.1 核心功能

| 功能 | 优先级 | 说明 |
|---|---|---|
| 用量卡片展示 | P0 | 按厂商分卡片，显示 5 小时额度、每周额度、使用百分比 |
| 系统托盘常驻 | P0 | 最小化到托盘，点击展开面板 |
| 自动刷新 | P0 | 每 5 分钟后台自动拉取数据 |
| API Key 配置 | P0 | 设置页管理各厂商的 API Key |
| 刷新倒计时 | P1 | 显示距离下次额度重置的时间 |
| 用量通知 | P1 | 额度低于 20% 时推送系统通知 |
| MCP 额度展示 | P2 | 智谱套餐的 MCP 工具调用次数（如适用） |
| 深色模式 | P2 | 跟随系统主题 |

### 4.2 交互设计

#### 主窗口

- 默认宽度 400px，高度自适应
- 顶部：应用标题 + 刷新按钮 + 上次更新时间
- 中部：厂商卡片列表（每个厂商一张卡片）
- 底部：自动刷新状态

#### 系统托盘

- 常驻图标：绿色圆点（正常）/ 红色圆点（额度紧张）
- 左键点击：展开/收起主窗口
- 右键菜单：刷新数据 / 打开设置 / 退出

#### 设置弹窗

- 各厂商 API Key 输入（密码遮罩）
- 刷新频率配置（默认 5 分钟）
- 通知阈值配置（默认 20%）
- 关于信息

### 4.3 UI 设计预览

打开 `docs/ui-preview.html` 在浏览器中查看卡片式布局预览。

设计要点：
- 每个厂商一张卡片，左侧彩色边框区分
- 进度条使用各厂商品牌色
- 额度数值使用等宽数字（tabular-nums）保证对齐
- 底部显示重置倒计时

---

## 5. 开发计划

### Phase 1：MVP（预计 2-3 天）

| 步骤 | 内容 | 依赖 |
|---|---|---|
| 1.1 | 项目初始化：Tauri v2 + Vue 3 + TypeScript + Pinia | - |
| 1.2 | 系统托盘配置：图标、右键菜单、窗口显示/隐藏 | 1.1 |
| 1.3 | 智谱数据对接：Rust HTTP 请求 + 缓存 | 1.1 |
| 1.4 | 前端卡片组件：VendorCard + QuotaBar | 1.1 |
| 1.5 | 设置页：API Key 存储与加载 | 1.1 |
| 1.6 | 定时刷新：Rust 后台调度器 | 1.3 |
| 1.7 | 集成测试：完整数据流 | 1.3, 1.4, 1.5, 1.6 |

> 1.1 完成后，1.2 ~ 1.5 可并行开发。

### Phase 2：体验优化（预计 1-2 天）

| 步骤 | 内容 |
|---|---|
| 2.1 | Kimi 数据对接（待确认 API） |
| 2.2 | 额度不足通知 |
| 2.3 | 深色模式 |
| 2.4 | 打包与分发（.dmg） |

### Phase 3：移动端（远期）

| 步骤 | 内容 |
|---|---|
| 3.1 | Tauri v2 Android 适配 |
| 3.2 | 移动端 UI 响应式适配 |
| 3.3 | iOS 适配（需评估是否需要开发者账号） |

---

## 6. 关键技术决策

### 6.1 为什么数据请求放在 Rust 层

- Rust 的 HTTP 客户端（reqwest）性能远优于 WebView 内的 fetch
- 不经过 WebView 线程，避免阻塞 UI 渲染
- 密钥存储更安全（不暴露到前端 JS 环境）
- 后台调度不依赖 WebView 生命周期

### 6.2 为什么选择 CSS Variables 而非 UI 框架

- 项目 UI 极简（2-3 个组件），引入 UI 框架（Element Plus / Naive UI）会增加 200KB+ 体积
- CSS Variables 天然支持主题切换（深色模式）
- 减少依赖，降低维护成本

### 6.3 缓存策略

```
启动时:
  → 读取本地缓存文件（上次数据）
  → 立即展示缓存数据
  → 后台发起 API 请求
  → 请求完成后更新 UI 并写入缓存

定时刷新（每 5 分钟）:
  → 后台发起 API 请求
  → 请求完成后通过 Tauri Event 更新前端

异常处理:
  → 请求失败时保持缓存数据
  → 显示"上次更新于 X 分钟前"提示
```

---

## 7. 安全考虑

| 项目 | 方案 |
|---|---|
| API Key 存储 | 使用系统 Keychain（macOS Keychain Services）通过 Tauri 插件存储 |
| 网络请求 | 强制 HTTPS |
| 前端暴露 | API Key 不传入 WebView，仅在 Rust 层使用 |
| 缓存文件 | 仅存储用量数据，不存储任何密钥信息 |

---

## 8. 风险与应对

| 风险 | 影响 | 应对措施 |
|---|---|---|
| Kimi 无公开用量 API | 无法获取 Kimi 用量数据 | 优先通过抓包确认内部接口；预留接口抽象层，后续补充 |
| Tauri v2 移动端成熟度 | 移动端适配可能遇到问题 | 先完成 Mac 版验证可行性；移动端作为 Phase 3 远期目标 |
| API 变更导致数据获取失败 | 工具不可用 | Rust 层做好错误处理和降级；缓存上次数据保证展示 |
| 智谱 API 限流 | 请求被拒绝 | 5 分钟刷新间隔足够宽松；失败后指数退避重试 |

---

## 9. 参考资源

- [Tauri v2 官方文档](https://v2.tauri.app/)
- [智谱 GLM Coding Plan 文档](https://docs.bigmodel.cn/cn/coding-plan/overview)
- [智谱 GLM Coding Plan FAQ](https://docs.bigmodel.cn/cn/coding-plan/faq)
- [Kimi Code 文档](https://www.kimi.com/code/docs/)
- [Kimi API 开放平台](https://platform.moonshot.cn/)
- [cc-switch 智谱用量查询示例](https://github.com/farion1231/cc-switch/issues/1588)
- [Kimi K2.5 用量查询 API Feature Request](https://github.com/MoonshotAI/Kimi-K2.5/issues/16)

# 贡献指南

感谢你对 AI Model Usage Monitor 的兴趣！

## 开发环境

- macOS 12+
- Rust 1.80+
- Node.js 18+
- Xcode Command Line Tools

## 快速开始

```bash
make install-deps
make dev
```

## 提交前检查

```bash
make check
cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings
```

## 提交规范

提交信息使用以下类型前缀：

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `refactor`: 重构
- `style`: 代码格式
- `chore`: 构建/工具链
- `test`: 测试

## 安全提醒

- 不要提交 API Key、密码或个人 config/cache 文件
- `~/.ai-usage-monitor/` 下的文件不会被 Git 追踪

## 报告问题

请使用 GitHub Issues，并尽量提供复现步骤和环境信息。

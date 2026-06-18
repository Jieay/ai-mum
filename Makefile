.PHONY: help dev build clean check lint dist open install-deps

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  dev          开发模式（热重载）"
	@echo "  build        正式构建"
	@echo "  check        编译检查（Rust + TypeScript）"
	@echo "  lint         代码检查"
	@echo "  dist         构建并显示产物"
	@echo "  open         构建并打开应用"
	@echo "  clean        清理构建产物"
	@echo "  install-deps 安装依赖"

dev:
	npm run tauri dev

build:
	npm run tauri build

check:
	cd src-tauri && cargo check
	npx vue-tsc --noEmit

clean:
	rm -rf dist
	rm -rf src-tauri/target
	rm -rf node_modules

dist: build
	@echo ""
	@echo "Build output:"
	@ls -lh src-tauri/target/release/ai-mum 2>/dev/null || true
	@ls -lh src-tauri/target/release/bundle/macos/*.app 2>/dev/null || true
	@ls -lh src-tauri/target/release/bundle/dmg/*.dmg 2>/dev/null || true

open: build
	open "src-tauri/target/release/bundle/macos/AI MUM.app"

install-deps:
	npm install
	cd src-tauri && cargo fetch

.PHONY: dev build clean check lint dist open

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

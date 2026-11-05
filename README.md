# Space 语音输入法

基于 Typeless 架构，在 ASR 转写和光标注入之间插入「知识库匹配阀门」的桌面语音输入工具。

**一键说话 → 自动转写 → 匹配知识库 → 按键选择 → 光标注入**

## 技术栈

- **Tauri 2** — 跨平台桌面框架（Rust + Web UI）
- **cpal** — 音频录制
- **reqwest** — ASR API 调用
- **CGEvent (macOS) / SendInput (Windows)** — 光标注入

## 快速开始

```bash
# 1. 安装依赖
npm install

# 2. 开发运行
npm run dev

# 3. 构建发布
npm run build
# 产物: src-tauri/target/release/bundle/
```

## CI/CD

本项目使用 GitHub Actions 自动构建 macOS 和 Windows 安装包。

每次推送至 `main` 或 `beta` 分支时触发构建，产物上传至 Actions Artifact。
发布 tag `v*` 时自动创建 Release 并上传安装包。

## 项目结构

```
space-voice-input/
├── .github/workflows/build.yml  # CI/CD 流水线
├── src/                          # 前端 Web UI
│   └── index.html
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs               # 入口
│       ├── lib.rs                # Tauri 应用设置
│       ├── kb.rs                 # 知识库匹配引擎
│       ├── asr.rs                # ASR API 客户端
│       ├── audio.rs              # 音频录制
│       ├── inject.rs             # 光标注入
│       └── commands.rs           # 前端命令
└── package.json
```
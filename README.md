# deskQ 🐾

> An AI desktop companion built with Rust + Tauri 2.

deskQ 是一个开源的 AI 桌面宠物。它以一只圆滚滚的小生物形态住在你的桌面上，
可以拖拽移动、用对话气泡聊天，并根据对话内容切换情绪动画（待机/开心/困倦/思考）。

## ✨ 功能

- 🐱 **桌面角色**：纯 SVG 绘制 + CSS 多状态动画（idle 呼吸 / happy 弹跳 / sleep 摇晃 / think 倾斜）
- 🖱️ **窗口交互**：透明无边框置顶窗口，鼠标按下拖拽移动，右键唤出菜单
- 💬 **AI 对话**：点击宠物打开对话气泡，内置本地规则回复引擎（可插拔 provider，预留真实 LLM 接入点）
- 🪶 **轻量**：Tauri 2 构建，macOS 包体仅约 8MB（对比 Electron 动辄 100MB+）

## 🎮 使用

| 操作 | 效果 |
|------|------|
| 按住宠物拖拽 | 移动窗口位置 |
| 单击宠物 | 打开/关闭对话气泡 |
| 右键点击 | 菜单：切换情绪 / 退出 |
| 对话框输入 + 回车 | 与桌宠对话（试试说"你好"、"你是谁"、"我累了"）|

## 🛠️ 技术栈

| 层 | 选择 | 说明 |
|----|------|------|
| 桌面壳 / 后端 | Rust + Tauri 2 | 轻量、跨平台、隐私优先 |
| 前端 | Vue 3 + Vite + TypeScript | 单文件组件，状态驱动动画 |
| AI | 可插拔 provider | 当前本地规则回复，可扩展 Ollama / OpenAI 兼容 API |

> 选型依据：调研了 GitHub 桌宠高星项目——BongoCat (21k⭐, Vue3+Tauri) 与 AIRI (41k⭐, Vue3) 均采用 Vue 3，本仓库沿用同栈以获得最成熟的参照实现。详见 `NOTES.md`。

## 🚀 开发

### 环境要求

- Rust (stable) + Cargo
- Node.js ≥ 20 + pnpm
- macOS: Xcode Command Line Tools

### 安装与运行

```bash
pnpm install          # 安装前端依赖
pnpm tauri dev        # 开发模式（热重载）
pnpm tauri build      # 构建 release 产物（.app / .dmg）
```

### 测试

```bash
pnpm test             # 前端组件测试 (vitest)
cd src-tauri && cargo test   # Rust 单元测试
```

## 📁 项目结构

```
deskQ/
├── src/                      # 前端 (Vue 3)
│   ├── components/
│   │   ├── PetCharacter.vue  # 桌宠角色 SVG + 动画
│   │   └── ChatBubble.vue    # 对话气泡 + 输入
│   ├── App.vue               # 主界面（窗口/拖拽/菜单）
│   ├── main.ts
│   └── types.ts              # 共享类型 (Mood / ChatReply)
└── src-tauri/                # 后端 (Rust)
    └── src/
        ├── lib.rs            # Tauri 入口
        ├── ai.rs             # AI 对话模块 + provider 抽象
        └── commands.rs       # Tauri 命令 (quit / chat)
```

## 🔮 路线图

- [ ] 接入 Live2D（`pixi-live2d-display`）实现更丰富的角色动画
- [ ] 真实 LLM provider（Ollama 本地 / OpenAI 兼容）
- [ ] 语音输入/输出
- [ ] 鼠标点击穿透优化
- [ ] Windows / Linux 平台测试

## 📄 License

[MIT](./LICENSE) © deskQ contributors

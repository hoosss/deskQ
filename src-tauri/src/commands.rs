//! Tauri 命令定义。
//!
//! 单独成模块，避免 `#[tauri::command]` 宏在 lib 顶层
//! 与 `generate_handler!` 产生宏名冲突。

use crate::ai::{self, Mood};
use serde::Serialize;

/// 对话返回结构。
#[derive(Debug, Clone, Serialize)]
pub struct ChatReply {
    /// 回复文本
    pub text: String,
    /// 当前情绪状态（前端据此切换动画）
    pub mood: Mood,
}

/// 退出应用。
#[tauri::command]
pub fn quit(app: tauri::AppHandle) {
    app.exit(0);
}

/// AI 对话入口。
///
/// 当前使用 [`ai::local_reply`]（本地规则回复），后续可在此切换为
/// 真实 LLM provider（Ollama / OpenAI 兼容 API 等）。
#[tauri::command]
pub fn chat(message: String) -> ChatReply {
    let (text, mood) = ai::local_reply(&message);
    ChatReply { text, mood }
}

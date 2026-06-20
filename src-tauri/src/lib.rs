// deskQ - AI desktop companion
// Tauri 后端入口
//! deskQ 后端核心。
//!
//! - [`commands`]：Tauri 命令（quit / chat）
//! - [`ai`]：AI 对话模块（本地规则回复 + provider 抽象）

pub mod ai;
pub mod commands;

pub use commands::{chat, quit};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![quit, chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

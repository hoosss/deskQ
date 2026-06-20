//! AI 对话模块。
//!
//! 设计目标：provider 可插拔。当前实现 [`local_reply`]（本地规则回复），
//! 后续可在 [`reply`] 中按配置切换为真实 LLM（Ollama / OpenAI 兼容等）。
//!
//! 所有回复逻辑均为纯函数，便于单元测试。

use serde::{Deserialize, Serialize};

/// 情绪状态，前端据此切换角色动画。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    /// 平静待机
    Idle,
    /// 开心
    Happy,
    /// 困倦/睡觉
    Sleep,
    /// 困惑/思考
    Think,
}

/// 本地规则回复（provider = local）。
///
/// 基于关键词匹配返回预设回复，无需网络与外部依赖。
/// 保证非空返回，避免前端收到空消息。
pub fn local_reply(message: &str) -> (String, Mood) {
    let msg = message.trim();
    if msg.is_empty() {
        return ("你好像没说话呀～要不要跟我聊聊天？".to_string(), Mood::Think);
    }

    let lower = msg.to_lowercase();

    // 关键词规则（顺序即优先级）
    if matches_any(&lower, &["你好", "hi", "hello", "嗨", "在吗"]) {
        return ("你好呀！我是 deskQ，你的桌面小伙伴 🐾".to_string(), Mood::Happy);
    }
    if matches_any(&lower, &["再见", "拜拜", "bye", "晚安"]) {
        return ("下次见！我会在这里等你～".to_string(), Mood::Sleep);
    }
    if matches_any(&lower, &["谢谢", "thanks", "感谢"]) {
        return ("不客气哒！能帮到你我很开心～".to_string(), Mood::Happy);
    }
    if matches_any(&lower, &["困", "累", "睡", "tired", "sleep"]) {
        return ("累了吗？那我陪你休息一会儿……".to_string(), Mood::Sleep);
    }
    if matches_any(&lower, &["你是谁", "名字", "who", "what are you"]) {
        return ("我是 deskQ，一个住在桌面上的人工智能小宠物！".to_string(), Mood::Happy);
    }
    if msg.contains('?') || msg.contains('？') {
        return ("嗯……让我想想……（这个问题我先记下啦）".to_string(), Mood::Think);
    }

    // 兜底：回声式，确认收到
    let echo = format!("你说的是「{}」吗？我听着呢～", truncate(msg, 30));
    (echo, Mood::Idle)
}

/// 统一对话入口。
///
/// 目前直接转发到 [`local_reply`]；预留 provider 切换点。
pub fn reply(message: &str) -> (String, Mood) {
    local_reply(message)
}

// ---- 内部工具 ----

/// 任一关键词命中即返回 true（大小写不敏感由调用方保证 lower）。
fn matches_any(text: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|k| text.contains(k))
}

/// 截断到 max 个字符，超出加省略号（按 char 计数，兼容中文）。
fn truncate(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        s.to_string()
    } else {
        let head: String = chars.iter().take(max).collect();
        format!("{}…", head)
    }
}

// ==================== 单元测试 ====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_message_does_not_panic() {
        let (text, mood) = local_reply("");
        assert!(!text.is_empty(), "空输入也要返回提示");
        assert_eq!(mood, Mood::Think);
    }

    #[test]
    fn whitespace_only_treated_as_empty() {
        let (text, _) = local_reply("   \n\t ");
        assert!(!text.is_empty());
    }

    #[test]
    fn greeting_returns_happy() {
        for input in &["你好", "Hello", "hi", "嗨", "在吗?"] {
            let (text, mood) = local_reply(input);
            assert_eq!(mood, Mood::Happy, "问候「{}」应为 Happy", input);
            assert!(text.contains("deskQ"), "问候回复应提到名字");
        }
    }

    #[test]
    fn farewell_returns_sleep() {
        for input in &["再见", "拜拜", "bye", "晚安"] {
            let (_, mood) = local_reply(input);
            assert_eq!(mood, Mood::Sleep, "道别「{}」应为 Sleep", input);
        }
    }

    #[test]
    fn thanks_returns_happy() {
        let (_, mood) = local_reply("谢谢你！");
        assert_eq!(mood, Mood::Happy);
    }

    #[test]
    fn tired_returns_sleep() {
        let (_, mood) = local_reply("我好累啊");
        assert_eq!(mood, Mood::Sleep);
    }

    #[test]
    fn identity_question_returns_happy() {
        let (text, mood) = local_reply("你是谁？");
        assert_eq!(mood, Mood::Happy);
        assert!(text.contains("宠物") || text.contains("deskQ"));
    }

    #[test]
    fn general_question_returns_think() {
        let (_, mood) = local_reply("今天天气怎么样？");
        assert_eq!(mood, Mood::Think);
    }

    #[test]
    fn fallback_echoes_input_as_idle() {
        let (text, mood) = local_reply("随便说点什么吧");
        assert_eq!(mood, Mood::Idle);
        assert!(text.contains("随便说点什么"), "兜底应回声原话");
    }

    #[test]
    fn case_insensitive_english() {
        let (_, mood) = local_reply("HELLO THERE");
        assert_eq!(mood, Mood::Happy);
        let (_, mood2) = local_reply("BYE");
        assert_eq!(mood2, Mood::Sleep);
    }

    #[test]
    fn truncate_handles_chinese() {
        assert_eq!(truncate("一二三", 5), "一二三");
        assert_eq!(truncate("一二三四五六", 3), "一二三…");
        assert_eq!(truncate("ab", 5), "ab");
    }

    #[test]
    fn truncate_long_message_in_echo() {
        let long = "a".repeat(100);
        let (text, _) = local_reply(&long);
        // 回声应被截断，长度远小于原文
        assert!(text.len() < long.len() + 20);
    }

    #[test]
    fn reply_delegates_to_local() {
        let via_reply = reply("你好");
        let via_local = local_reply("你好");
        assert_eq!(via_reply.0, via_local.0);
        assert_eq!(via_reply.1, via_local.1);
    }

    #[test]
    fn mood_serializes_lowercase() {
        let json = serde_json::to_string(&Mood::Happy).unwrap();
        assert_eq!(json, "\"happy\"");
        let idle = serde_json::to_string(&Mood::Idle).unwrap();
        assert_eq!(idle, "\"idle\"");
    }

    #[test]
    fn mood_roundtrip_deserialize() {
        let m: Mood = serde_json::from_str("\"think\"").unwrap();
        assert_eq!(m, Mood::Think);
    }
}

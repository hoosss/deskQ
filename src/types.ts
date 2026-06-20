/**
 * 前端共享类型。
 * Mood 与 Rust 端 `ai::Mood` 的 serde 小写形式保持一致。
 */
export type Mood = "idle" | "happy" | "sleep" | "think";

/** 后端 chat 命令返回结构。 */
export interface ChatReply {
  text: string;
  mood: Mood;
}

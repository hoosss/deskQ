<script setup lang="ts">
/**
 * 对话气泡 + 输入框。
 * 点击宠物打开，回车发送，显示 AI 回复。
 * 发送期间禁用输入，避免重复请求。
 */
import { ref, nextTick, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ChatReply, Mood } from "../types";

const props = defineProps<{ open: boolean }>();
const emit = defineEmits<{
  (e: "close"): void;
  (e: "mood-change", mood: Mood): void;
}>();

const input = ref("");
const reply = ref("");
const loading = ref(false);
const inputEl = ref<HTMLInputElement | null>(null);

// 打开时自动聚焦输入框
watch(
  () => props.open,
  async (v) => {
    if (v) {
      await nextTick();
      inputEl.value?.focus();
    } else {
      input.value = "";
      reply.value = "";
    }
  }
);

async function send() {
  const msg = input.value.trim();
  if (!msg || loading.value) return;
  loading.value = true;
  try {
    const res = await invoke<ChatReply>("chat", { message: msg });
    reply.value = res.text;
    emit("mood-change", res.mood);
    input.value = "";
  } catch (err) {
    reply.value = "（连接出了点小问题，待会儿再试～）";
    console.error(err);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div v-if="open" class="bubble">
    <div class="bubble__body">
      <p v-if="reply" class="bubble__reply">{{ reply }}</p>
      <p v-else class="bubble__placeholder">跟我说话吧～</p>
      <button class="bubble__close" @click="emit('close')" aria-label="关闭">×</button>
    </div>
    <form class="bubble__input" @submit.prevent="send">
      <input
        ref="inputEl"
        v-model="input"
        placeholder="说点什么..."
        :disabled="loading"
      />
      <button type="submit" :disabled="loading || !input.trim()">
        {{ loading ? "..." : "→" }}
      </button>
    </form>
  </div>
</template>

<style scoped>
.bubble {
  position: absolute;
  bottom: 72%;
  left: 50%;
  transform: translateX(-50%);
  width: 220px;
  background: #ffffff;
  border: 2px solid #ffbf47;
  border-radius: 16px;
  padding: 10px;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.18);
  font-size: 13px;
  /* 气泡小尖角 */
}
.bubble::after {
  content: "";
  position: absolute;
  bottom: -10px;
  left: 50%;
  transform: translateX(-50%);
  border-width: 10px 8px 0 8px;
  border-style: solid;
  border-color: #ffbf47 transparent transparent transparent;
}

.bubble__body {
  position: relative;
  min-height: 32px;
  padding-right: 16px;
}
.bubble__reply {
  margin: 0;
  color: #4a3520;
  line-height: 1.4;
  word-break: break-word;
}
.bubble__placeholder {
  margin: 0;
  color: #b0a080;
}
.bubble__close {
  position: absolute;
  top: -6px;
  right: -6px;
  width: 18px;
  height: 18px;
  border: none;
  background: #ff9eb5;
  color: #fff;
  border-radius: 50%;
  font-size: 12px;
  line-height: 1;
  cursor: pointer;
  padding: 0;
}

.bubble__input {
  display: flex;
  gap: 4px;
  margin-top: 8px;
}
.bubble__input input {
  flex: 1;
  border: 1px solid #e0d090;
  border-radius: 8px;
  padding: 4px 8px;
  font-size: 12px;
  outline: none;
}
.bubble__input input:focus {
  border-color: #ffbf47;
}
.bubble__input button {
  border: none;
  background: #ffbf47;
  color: #fff;
  border-radius: 8px;
  padding: 4px 10px;
  font-size: 12px;
  cursor: pointer;
}
.bubble__input button:disabled {
  background: #ddd;
  cursor: not-allowed;
}
</style>

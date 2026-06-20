<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import PetCharacter from "./components/PetCharacter.vue";
import ChatBubble from "./components/ChatBubble.vue";
import type { Mood } from "./types";

/**
 * 桌宠主界面。
 * - 透明窗口内显示角色
 * - 鼠标按下拖拽移动窗口
 * - 左键单击：开关对话气泡
 * - 右键：菜单（退出 / 各状态切换）
 */

const mood = ref<Mood>("idle");
const chatOpen = ref(false);
const menuOpen = ref(false);

const win = getCurrentWindow();

// ---- 拖拽窗口 ----
// 在角色上按下鼠标即开始拖拽（Tauri 原生窗口移动）
async function startDrag() {
  try {
    await win.startDragging();
  } catch (e) {
    // 非 Tauri 环境（纯浏览器调试）忽略
    console.warn("startDragging failed:", e);
  }
}

// ---- 点击开关对话 ----
function onPetClick() {
  chatOpen.value = !chatOpen.value;
  menuOpen.value = false;
}

// ---- 右键菜单 ----
function onContextMenu(e: MouseEvent) {
  e.preventDefault();
  menuOpen.value = true;
}

// ---- 菜单动作 ----
async function quit() {
  try {
    await invoke("quit");
  } catch {
    window.close();
  }
}
function setMood(m: Mood) {
  mood.value = m;
  menuOpen.value = false;
}

// ---- 点击空白处关闭菜单 ----
function onWindowClick() {
  if (menuOpen.value) menuOpen.value = false;
}

onMounted(() => {
  window.addEventListener("click", onWindowClick);
});
onUnmounted(() => {
  window.removeEventListener("click", onWindowClick);
});
</script>

<template>
  <div
    class="app"
    @contextmenu="onContextMenu"
  >
    <!-- 对话气泡 -->
    <ChatBubble
      :open="chatOpen"
      @close="chatOpen = false"
      @mood-change="(m) => (mood = m)"
    />

    <!-- 桌宠角色 -->
    <div
      class="app__pet-wrap"
      @mousedown.left="startDrag"
      @click="onPetClick"
    >
      <PetCharacter :mood="mood" />
    </div>

    <!-- 右键菜单 -->
    <div v-if="menuOpen" class="menu" @click.stop>
      <button class="menu__item" @click="setMood('idle')">🙂 待机</button>
      <button class="menu__item" @click="setMood('happy')">😄 开心</button>
      <button class="menu__item" @click="setMood('sleep')">😴 睡觉</button>
      <button class="menu__item" @click="setMood('think')">🤔 思考</button>
      <hr class="menu__sep" />
      <button class="menu__item menu__item--danger" @click="quit">退出 deskQ</button>
    </div>
  </div>
</template>

<style>
/* 全局：透明窗口背景 */
html,
body,
#app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: transparent;
  font-family: -apple-system, "PingFang SC", "Microsoft YaHei", sans-serif;
}
</style>

<style scoped>
.app {
  position: relative;
  width: 100vw;
  height: 100vh;
  display: flex;
  align-items: flex-end;
  justify-content: center;
}

.app__pet-wrap {
  width: 80%;
  height: 80%;
  display: flex;
  align-items: flex-end;
  justify-content: center;
  /* 允许点击穿透到角色以外的区域，但角色本身可交互 */
}

.menu {
  position: absolute;
  top: 30%;
  left: 50%;
  transform: translateX(-50%);
  background: #fff;
  border: 1px solid #e5d8a0;
  border-radius: 10px;
  box-shadow: 0 6px 18px rgba(0, 0, 0, 0.15);
  padding: 4px;
  z-index: 10;
  display: flex;
  flex-direction: column;
  min-width: 120px;
}
.menu__item {
  border: none;
  background: transparent;
  text-align: left;
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: #4a3520;
}
.menu__item:hover {
  background: #fff6dd;
}
.menu__item--danger {
  color: #d04040;
}
.menu__sep {
  border: none;
  border-top: 1px solid #f0e8c8;
  margin: 3px 0;
}
</style>

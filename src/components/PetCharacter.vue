<script setup lang="ts">
import { computed } from "vue";
import type { Mood } from "../types";

/**
 * 桌宠角色（SVG + CSS 动画）。
 * 用纯 SVG 绘制一只圆滚滚的小生物，根据 mood 切换动画：
 * - idle：轻微呼吸起伏
 * - happy：弹跳 + 眼睛弯成笑眼
 * - sleep：缓慢左右摇晃 + 闭眼
 * - think：轻微倾斜 + 疑惑眼
 */
const props = defineProps<{ mood: Mood }>();

// 根据 mood 切换眼睛形态
const eyes = computed(() => {
  switch (props.mood) {
    case "happy":
      return "happy";
    case "sleep":
      return "closed";
    case "think":
      return "think";
    default:
      return "normal";
  }
});
</script>

<template>
  <div class="pet" :class="`pet--${mood}`" aria-label="deskQ 桌面宠物">
    <svg viewBox="0 0 200 200" class="pet__svg" xmlns="http://www.w3.org/2000/svg">
      <!-- 阴影 -->
      <ellipse class="pet__shadow" cx="100" cy="182" rx="42" ry="6" />

      <!-- 身体 -->
      <g class="pet__body">
        <!-- 耳朵 -->
        <path class="pet__ear" d="M58 52 Q50 20 72 32 Q70 46 78 58 Z" />
        <path class="pet__ear" d="M142 52 Q150 20 128 32 Q130 46 122 58 Z" />

        <!-- 主体圆 -->
        <circle class="pet__belly" cx="100" cy="108" r="58" />

        <!-- 脸蛋红晕 -->
        <circle class="pet__cheek" cx="68" cy="120" r="9" />
        <circle class="pet__cheek" cx="132" cy="120" r="9" />

        <!-- 眼睛 -->
        <g v-if="eyes === 'normal'">
          <circle class="pet__eye" cx="80" cy="98" r="7" />
          <circle class="pet__eye" cx="120" cy="98" r="7" />
          <circle class="pet__eye-shine" cx="82" cy="95" r="2.5" />
          <circle class="pet__eye-shine" cx="122" cy="95" r="2.5" />
        </g>
        <g v-else-if="eyes === 'happy'">
          <!-- 笑眼 ^_^ -->
          <path class="pet__eye-line" d="M72 100 Q80 88 88 100" />
          <path class="pet__eye-line" d="M112 100 Q120 88 128 100" />
        </g>
        <g v-else-if="eyes === 'closed'">
          <!-- 闭眼 - - -->
          <path class="pet__eye-line" d="M72 98 L88 98" />
          <path class="pet__eye-line" d="M112 98 L128 98" />
        </g>
        <g v-else>
          <!-- 思考眼 > < -->
          <path class="pet__eye-line" d="M74 94 L86 98 L74 102" />
          <path class="pet__eye-line" d="M126 94 L114 98 L126 102" />
        </g>

        <!-- 嘴巴 -->
        <path
          class="pet__mouth"
          :class="`pet__mouth--${mood}`"
          d="M88 122 Q100 134 112 122"
        />
      </g>
    </svg>
  </div>
</template>

<style scoped>
.pet {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  user-select: none;
  -webkit-user-select: none;
}

.pet__svg {
  width: 75%;
  height: 75%;
  cursor: grab;
}

.pet__svg:active {
  cursor: grabbing;
}

/* 颜色 */
.pet__shadow {
  fill: rgba(0, 0, 0, 0.15);
}
.pet__body {
  fill: #ffd66b;
}
.pet__ear {
  fill: #ffbf47;
}
.pet__belly {
  fill: #ffe8a3;
  stroke: #ffbf47;
  stroke-width: 2;
}
.pet__cheek {
  fill: #ff9eb5;
  opacity: 0.7;
}
.pet__eye {
  fill: #4a3520;
}
.pet__eye-shine {
  fill: #ffffff;
}
.pet__eye-line {
  fill: none;
  stroke: #4a3520;
  stroke-width: 3;
  stroke-linecap: round;
}
.pet__mouth {
  fill: none;
  stroke: #4a3520;
  stroke-width: 2.5;
  stroke-linecap: round;
  transform-origin: 100px 122px;
}
.pet__mouth--sleep {
  /* 睡觉时小嘴 */
  d: path("M95 122 Q100 126 105 122");
}
.pet__mouth--think {
  d: path("M92 124 Q100 118 110 124");
}

/* ---- 状态动画 ---- */

/* idle：呼吸 */
.pet--idle .pet__body {
  animation: breathe 3.2s ease-in-out infinite;
  transform-origin: 100px 150px;
}
@keyframes breathe {
  0%, 100% { transform: scaleY(1) scaleX(1); }
  50% { transform: scaleY(1.04) scaleX(0.98); }
}

/* happy：弹跳 */
.pet--happy .pet__body {
  animation: bounce 0.6s ease-in-out infinite;
  transform-origin: 100px 160px;
}
@keyframes bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-12px); }
}

/* sleep：摇晃 + 整体下沉 */
.pet--sleep .pet__body {
  animation: sway 4s ease-in-out infinite;
  transform-origin: 100px 170px;
}
@keyframes sway {
  0%, 100% { transform: rotate(-3deg); }
  50% { transform: rotate(3deg); }
}

/* think：轻微倾斜 */
.pet--think .pet__body {
  animation: tilt 1.8s ease-in-out infinite;
  transform-origin: 100px 170px;
}
@keyframes tilt {
  0%, 100% { transform: rotate(-2deg); }
  50% { transform: rotate(2deg); }
}
</style>

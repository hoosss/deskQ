/**
 * PetCharacter 组件单元测试。
 * 验证：不同 mood 渲染出对应的动画 class 与眼睛形态。
 */
import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import PetCharacter from "./PetCharacter.vue";
import type { Mood } from "../types";

function mountPet(mood: Mood) {
  return mount(PetCharacter, { props: { mood } });
}

describe("PetCharacter", () => {
  it("idle 状态渲染呼吸动画 class", () => {
    const wrapper = mountPet("idle");
    const pet = wrapper.find(".pet");
    expect(pet.classes()).toContain("pet--idle");
  });

  it("happy 状态渲染弹跳动画 class", () => {
    const wrapper = mountPet("happy");
    expect(wrapper.find(".pet").classes()).toContain("pet--happy");
  });

  it("sleep 状态渲染摇晃动画 class", () => {
    const wrapper = mountPet("sleep");
    expect(wrapper.find(".pet").classes()).toContain("pet--sleep");
  });

  it("think 状态渲染倾斜动画 class", () => {
    const wrapper = mountPet("think");
    expect(wrapper.find(".pet").classes()).toContain("pet--think");
  });

  it("idle 时眼睛为 normal（圆形）", () => {
    const wrapper = mountPet("idle");
    expect(wrapper.findAll(".pet__eye").length).toBe(2);
    expect(wrapper.findAll(".pet__eye-shine").length).toBe(2);
  });

  it("happy 时眼睛为笑眼（曲线）", () => {
    const wrapper = mountPet("happy");
    expect(wrapper.find(".pet__eye-line").exists()).toBe(true);
    // 笑眼有两条曲线
    expect(wrapper.findAll(".pet__eye-line").length).toBeGreaterThanOrEqual(2);
  });

  it("sleep 时眼睛为闭合线", () => {
    const wrapper = mountPet("sleep");
    const lines = wrapper.findAll(".pet__eye-line");
    expect(lines.length).toBeGreaterThanOrEqual(2);
  });

  it("think 时眼睛为箭头形", () => {
    const wrapper = mountPet("think");
    expect(wrapper.find(".pet__eye-line").exists()).toBe(true);
  });

  it("切换 mood 后 class 随之更新", async () => {
    const wrapper = mountPet("idle");
    expect(wrapper.find(".pet").classes()).toContain("pet--idle");
    await wrapper.setProps({ mood: "happy" });
    expect(wrapper.find(".pet").classes()).toContain("pet--happy");
    expect(wrapper.find(".pet").classes()).not.toContain("pet--idle");
  });

  it("始终渲染身体与嘴巴", () => {
    for (const m of ["idle", "happy", "sleep", "think"] as Mood[]) {
      const wrapper = mountPet(m);
      expect(wrapper.find(".pet__body").exists()).toBe(true);
      expect(wrapper.find(".pet__mouth").exists()).toBe(true);
    }
  });
});

import { createApp } from "vue";
import App from "./App.vue";
import "./styles/main.css";

// 实时响应 macOS 系统深色/浅色模式切换
const setupSystemTheme = () => {
  if (typeof window === "undefined" || !window.matchMedia) return;

  const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

  const applyTheme = (isDark: boolean) => {
    if (isDark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };

  // 1. 初始化当前系统的深色/浅色状态
  applyTheme(darkModeMediaQuery.matches);

  // 2. 监听系统深浅色切换事件 (Mac 设置中切换深浅色时实时触发)
  try {
    darkModeMediaQuery.addEventListener("change", (e) => {
      applyTheme(e.matches);
    });
  } catch {
    // 兼容旧版 Safari / WebKit 的 addListener
    darkModeMediaQuery.addListener((e) => {
      applyTheme(e.matches);
    });
  }
};

setupSystemTheme();

createApp(App).mount("#app");

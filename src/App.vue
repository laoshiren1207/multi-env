<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEnvManager } from "./composables/useEnvManager";
import type { Profile, RuntimeInfo } from "./types";
import { useI18n } from "./i18n";
import StatusNotification from "./components/StatusNotification.vue";
import ProfileForm from "./components/ProfileForm.vue";

const { locale, setLocale, toggleLocale, t } = useI18n();
const appWindow = getCurrentWindow();

// 支持平滑拖拽整个窗口
const startDrag = async (e: MouseEvent) => {
  if (e.buttons !== 1) return;
  // 如果点到了按钮、输入框、可交互元素则不触发拖拽
  const target = e.target as HTMLElement | null;
  if (target?.closest("button, input, select, textarea, a")) return;
  try {
    await appWindow.startDragging();
  } catch (err) {
    console.error("Window startDragging failed:", err);
  }
};

const {
  runtimes,
  profiles,
  nodeMirrors,
  settingMirror,
  loading,
  switching,
  installing,
  uninstalling,
  installingManager,
  fetchRuntimes,
  fetchNodeMirrors,
  setNpmRegistry,
  setNvmMirror,
  setBothNodeMirrors,
  switchVersion,
  installVersion,
  uninstallVersion,
  installManager,
  fetchProfiles,
  saveProfile,
  deleteProfile,
  applyProfile,
} = useEnvManager();

// 自定义镜像输入
const customNpmRegistry = ref("");
const customNvmMirror = ref("");

// 快捷镜像切换方法
const handleSetAllChinaMirrors = async () => {
  const res = await setBothNodeMirrors(
    "https://registry.npmmirror.com/",
    "https://npmmirror.com/mirrors/node/"
  );
  if (res.success) {
    showNotification(t("allChinaMirrorsSuccessNotice"), true);
  } else {
    showNotification(res.message, false);
  }
};

const handleRestoreOfficialMirrors = async () => {
  const res = await setBothNodeMirrors(
    "https://registry.npmjs.org/",
    "https://nodejs.org/dist"
  );
  if (res.success) {
    showNotification(t("allOfficialMirrorsSuccessNotice"), true);
  } else {
    showNotification(res.message, false);
  }
};

const handleSetNpmRegistry = async (url: string) => {
  const target = url.trim();
  if (!target) return;
  const res = await setNpmRegistry(target);
  if (res.success) {
    showNotification(t("npmRegistrySuccessNotice", { url: target }), true);
    customNpmRegistry.value = "";
  } else {
    showNotification(res.message, false);
  }
};

const handleSetNvmMirror = async (url: string) => {
  const target = url.trim();
  if (!target) return;
  const res = await setNvmMirror(target);
  if (res.success) {
    showNotification(t("nvmMirrorSuccessNotice", { url: target }), true);
    customNvmMirror.value = "";
  } else {
    showNotification(res.message, false);
  }
};

// 输入的新版本号
const customInstallVersion = ref<string>("");

// 获取管理器的官方一键安装命令
const getManagerInstallCmd = (manager: string) => {
  if (manager === "sdkman") return 'curl -s "https://get.sdkman.io" | bash';
  if (manager === "nvm") return 'curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash';
  if (manager === "uv") return 'curl -LsSf https://astral.sh/uv/install.sh | sh';
  return "";
};

// 一键安装环境管理器处理函数
const handleInstallManager = async (manager: string) => {
  showNotification(t("installingManager", { manager }), true);
  const result = await installManager(manager);
  if (result.success) {
    showNotification(t("installManagerSuccessNotice", { manager }), true);
  } else {
    showNotification(t("installManagerFailedNotice", { manager, msg: result.message }), false);
  }
};

// 滚动状态监测：未滚动或切换面板时滚动条保持隐形，滚动时显现并在停止后淡出
const isScrolling = ref(false);
let scrollTimeout: any = null;

const handleScroll = () => {
  isScrolling.value = true;
  if (scrollTimeout) clearTimeout(scrollTimeout);
  scrollTimeout = setTimeout(() => {
    isScrolling.value = false;
  }, 800);
};

// 快速推荐安装的版本
const quickInstallVersions = computed(() => {
  if (selectedNav.value === "node") {
    return ["22", "20", "18", "16"];
  }
  if (selectedNav.value === "java") {
    return ["21.0.2-tem", "17.0.10-tem", "11.0.22-tem", "8.0.402-tem"];
  }
  if (selectedNav.value === "python") {
    return ["3.12", "3.11", "3.10", "3.9"];
  }
  return [];
});

const handleInstall = async (runtimeName: string, verToInstall?: string) => {
  const ver = verToInstall || customInstallVersion.value.trim();
  if (!ver) return;

  showNotification(t("installingNotice", { runtime: runtimeName, version: ver }), true);
  const result = await installVersion(runtimeName, ver);
  if (result.success) {
    showNotification(t("installSuccessNotice", { version: ver }), true);
    customInstallVersion.value = "";
  } else {
    showNotification(t("installFailedNotice", { msg: result.message }), false);
  }
};

// 卸载二次确认弹窗状态
const confirmDialog = ref<{
  show: boolean;
  runtimeName: string;
  version: string;
  displayName: string;
}>({
  show: false,
  runtimeName: "",
  version: "",
  displayName: "",
});

const openUninstallConfirm = (runtimeName: string, ver: string, displayName?: string) => {
  confirmDialog.value = {
    show: true,
    runtimeName,
    version: ver,
    displayName: displayName || ver,
  };
};

const cancelUninstallConfirm = () => {
  confirmDialog.value.show = false;
};

const executeUninstall = async () => {
  const { runtimeName, version, displayName } = confirmDialog.value;
  confirmDialog.value.show = false;

  showNotification(t("uninstallingNotice", { name: displayName }), true);
  const result = await uninstallVersion(runtimeName, version);
  if (result.success) {
    showNotification(t("uninstallSuccessNotice", { name: displayName }), true);
  } else {
    showNotification(t("uninstallFailedNotice", { msg: result.message }), false);
  }
};

// 当前选中的侧边栏项: 'java' | 'node' | 'python' | 'profiles' | 'about'
const selectedNav = ref<string>("java");
const searchQuery = ref<string>("");
const searchInputRef = ref<HTMLInputElement | null>(null);

// 模态框/编辑表单状态
const showProfileModal = ref(false);
const editingProfile = ref<Profile | null>(null);

// Toast 通知状态
const toast = ref<{
  show: boolean;
  success: boolean;
  message: string;
}>({
  show: false,
  success: true,
  message: "",
});

const showNotification = (message: string, success: boolean = true) => {
  toast.value = {
    show: true,
    success,
    message,
  };
};

const closeNotification = () => {
  toast.value.show = false;
};

// 快捷键支持: ⌘F / Ctrl+F 聚焦搜索框
const handleKeyDown = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && (e.key === "f" || e.key === "F")) {
    e.preventDefault();
    searchInputRef.value?.focus();
    searchInputRef.value?.select();
  }
  if (e.key === "Escape") {
    if (searchQuery.value) {
      searchQuery.value = "";
    } else {
      searchInputRef.value?.blur();
    }
  }
};

onMounted(async () => {
  window.addEventListener("keydown", handleKeyDown);
  await Promise.all([fetchRuntimes(), fetchProfiles(), fetchNodeMirrors()]);

  // 默认选中第一个可用的运行时，或者 java
  if (runtimes.value.length > 0) {
    selectedNav.value = runtimes.value[0].name;
  }
});

watch(selectedNav, (newVal) => {
  if (newVal === "node") {
    fetchNodeMirrors();
  }
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
});

// 侧边栏菜单项配置
interface NavItem {
  id: string;
  name: string;
  category: "envs" | "tools";
  iconName: string;
  bgColor: string;
  badge?: string;
  keywords: string[];
}

const navItems = computed<NavItem[]>(() => {
  const items: NavItem[] = [];

  for (const rt of runtimes.value) {
    let bgColor = "bg-blue-500";
    if (rt.name === "node") bgColor = "bg-emerald-500";
    if (rt.name === "python") bgColor = "bg-amber-500";

    const verKeywords = rt.versions.map((v) => `${v.version} ${v.display_name}`).join(" ");
    const name = rt.name === "java" ? t("javaEnv") : rt.name === "node" ? t("nodeEnv") : t("pythonEnv");

    items.push({
      id: rt.name,
      name,
      category: "envs",
      iconName: rt.icon,
      bgColor,
      badge: rt.current_version ? rt.current_version.split("-")[0] : undefined,
      keywords: [rt.name, rt.display_name, rt.manager, verKeywords, name],
    });
  }

  // 环境预设
  items.push({
    id: "profiles",
    name: t("profiles"),
    category: "tools",
    iconName: "📋",
    bgColor: "bg-indigo-500",
    badge: profiles.value.length > 0 ? `${profiles.value.length}` : undefined,
    keywords: ["profile", "预设", "环境组", "方案", ...profiles.value.map((p) => p.name)],
  });

  // 关于与设置
  items.push({
    id: "about",
    name: t("about"),
    category: "tools",
    iconName: "⚙️",
    bgColor: "bg-neutral-500",
    keywords: ["about", "关于", "设置", "sdkman", "nvm", "uv", "路径", "language", "语言"],
  });

  return items;
});

// 过滤后的侧边栏列表 (基于搜索词)
const filteredNavItems = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return navItems.value;

  return navItems.value.filter((item) => {
    return (
      item.name.toLowerCase().includes(q) ||
      item.keywords.some((k) => k.toLowerCase().includes(q))
    );
  });
});

// 当前选中的标题
const selectedNavTitle = computed(() => {
  const item = navItems.value.find((i) => i.id === selectedNav.value);
  return item ? item.name : "";
});

// 当前正在查看的 Runtime
const currentRuntime = computed<RuntimeInfo | undefined>(() => {
  return runtimes.value.find((r) => r.name === selectedNav.value);
});

// 刷新处理
const handleRefresh = async () => {
  await Promise.all([fetchRuntimes(), fetchProfiles(), fetchNodeMirrors()]);
  showNotification(t("refreshedNotice"), true);
};

// 切换版本
const handleSwitch = async (runtimeName: string, version: string) => {
  const result = await switchVersion(runtimeName, version);
  if (result.success) {
    showNotification(t("switchSuccessNotice", { version }), true);
  } else {
    showNotification(t("switchFailedNotice", { version }), false);
  }
};

// 当前正在应用的 Profile ID (用于 loading 动画)
const applyingProfileId = ref<string | null>(null);

// 应用 Profile
const handleApplyProfile = async (id: string) => {
  if (applyingProfileId.value) return; // 防止重复点击
  applyingProfileId.value = id;

  const profile = profiles.value.find((p) => p.id === id);
  const name = profile ? profile.name : "Profile";
  try {
    const ok = await applyProfile(id);
    if (ok) {
      showNotification(t("profileAppliedNotice", { name }), true);
    } else {
      showNotification(t("profileApplyFailedNotice", { name }), false);
    }
  } catch (e) {
    showNotification(t("profileApplyException", { msg: String(e) }), false);
  } finally {
    applyingProfileId.value = null;
  }
};

// 选中的 Profile ID (用于列表高亮与底部减号删除)
const selectedProfileId = ref<string | null>(null);

// 判断某个预设是否已是当前激活状态
const isProfileActive = (p: Profile) => {
  const javaCurrent = runtimes.value.find((r) => r.name === "java")?.current_version;
  const nodeCurrent = runtimes.value.find((r) => r.name === "node")?.current_version;
  const pyCurrent = runtimes.value.find((r) => r.name === "python")?.current_version;

  const javaMatch = !p.java_version || javaCurrent === p.java_version;
  const nodeMatch = !p.node_version || nodeCurrent === p.node_version;
  const pyMatch = !p.python_version || (pyCurrent && (pyCurrent.startsWith(p.python_version) || p.python_version.startsWith(pyCurrent)));

  return Boolean(javaMatch && nodeMatch && pyMatch);
};

// 删除当前选中的 Profile
const handleDeleteSelectedProfile = async () => {
  if (!selectedProfileId.value) return;
  const p = profiles.value.find((i) => i.id === selectedProfileId.value);
  if (!p) return;
  await handleDeleteProfile(p.id);
  selectedProfileId.value = null;
};

// 打开新建 Profile 弹窗
const openNewProfileModal = () => {
  editingProfile.value = {
    id: "",
    name: "",
    java_version: null,
    node_version: null,
    python_version: null,
  };
  showProfileModal.value = true;
};

// 打开编辑 Profile 弹窗
const openEditProfileModal = (p: Profile) => {
  editingProfile.value = { ...p };
  showProfileModal.value = true;
};

// 保存 Profile
const handleSaveProfile = async (profile: Profile) => {
  showProfileModal.value = false;
  const ok = await saveProfile(profile);
  if (ok) {
    showNotification(t("profileSavedNotice", { name: profile.name }), true);
  } else {
    showNotification(t("profileSaveFailedNotice"), false);
  }
};

// 删除 Profile
const handleDeleteProfile = async (id: string) => {
  const ok = await deleteProfile(id);
  if (ok) {
    showNotification(t("profileDeletedNotice"), true);
  } else {
    showNotification(t("profileDeleteFailedNotice"), false);
  }
};
</script>

<template>
  <div class="flex h-screen w-screen overflow-hidden select-none bg-neutral-100 dark:bg-[#1e1e1e] text-[#1d1d1f] dark:text-[#f5f5f7] font-sans antialiased text-[13px]">
    
    <!-- ============================================== -->
    <!-- 左侧边栏 (Sidebar: 紧凑型 macOS System Settings) -->
    <!-- ============================================== -->
    <aside class="w-[190px] flex-shrink-0 flex flex-col border-r border-[#d1d1d6] dark:border-[#38383a] bg-[#f2f2f7]/80 dark:bg-[#28282a]/90 backdrop-blur-xl">
      
      <!-- 顶部留白区域（为 macOS 交通灯窗口控制按钮预留可拖拽区域） -->
      <div
        data-tauri-drag-region
        @mousedown="startDrag"
        class="h-10 flex items-center px-3 pt-0.5 flex-shrink-0 cursor-default"
      >
        <!-- 交通灯占位空间 -->
        <div class="w-14 pointer-events-none"></div>
      </div>

      <!-- 搜索栏 (支持 ⌘F 聚焦) -->
      <div class="px-2.5 pb-2 flex-shrink-0">
        <div class="relative flex items-center">
          <svg class="absolute left-2 w-3 h-3 text-[#8e8e93]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
          </svg>
          <input
            ref="searchInputRef"
            v-model="searchQuery"
            type="text"
            :placeholder="t('searchPlaceholder')"
            class="w-full pl-6 pr-6 py-0.5 bg-[#e3e3e8] dark:bg-[#3a3a3c] text-[11px] text-[#1d1d1f] dark:text-[#f5f5f7] placeholder-[#8e8e93] rounded-[5px] border border-transparent focus:border-[#007aff]/50 focus:bg-white dark:focus:bg-[#1c1c1e] focus:outline-none transition-all shadow-inner"
          />
          <button
            v-if="searchQuery"
            type="button"
            @click="searchQuery = ''"
            class="absolute right-1.5 text-[#8e8e93] hover:text-[#1d1d1f] dark:hover:text-[#f5f5f7]"
          >
            <svg class="w-3 h-3" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
      </div>

      <!-- 导航列表项 (紧凑分组展示) -->
      <nav class="flex-1 overflow-y-auto px-1.5 py-0.5 space-y-3">
        
        <!-- 环境组 -->
        <div>
          <div class="px-2 pb-0.5 text-[10px] font-medium text-[#8e8e93] uppercase tracking-wider">
            {{ t("categoryRuntimes") }}
          </div>
          <div class="space-y-0.5">
            <button
              v-for="item in filteredNavItems.filter(i => i.category === 'envs')"
              :key="item.id"
              type="button"
              @click="selectedNav = item.id"
              class="w-full flex items-center justify-between px-2 py-1 rounded-[5px] text-left transition-all group"
              :class="selectedNav === item.id 
                ? 'bg-[#007aff] text-white font-medium shadow-xs' 
                : 'text-[#1d1d1f] dark:text-[#e5e5ea] hover:bg-black/[0.04] dark:hover:bg-white/[0.05]'"
            >
              <div class="flex items-center gap-2 min-w-0">
                <div 
                  class="w-[18px] h-[18px] rounded-[4px] flex items-center justify-center text-[10px] shadow-xs flex-shrink-0"
                  :class="[item.bgColor, selectedNav === item.id ? 'ring-1 ring-white/30' : '']"
                >
                  <span>{{ item.iconName }}</span>
                </div>
                <span class="text-[12px] truncate">{{ item.name }}</span>
              </div>
              <span
                v-if="item.badge"
                class="text-[9px] px-1.5 py-0.2 rounded-full font-mono font-medium truncate max-w-[65px]"
                :class="selectedNav === item.id 
                  ? 'bg-white/20 text-white' 
                  : 'bg-black/5 dark:bg-white/10 text-[#8e8e93] dark:text-[#a1a1a6]'"
              >
                {{ item.badge }}
              </span>
            </button>
          </div>
        </div>

        <!-- 预设与工具组 -->
        <div>
          <div class="px-2 pb-0.5 text-[10px] font-medium text-[#8e8e93] uppercase tracking-wider">
            {{ t("categoryTools") }}
          </div>
          <div class="space-y-0.5">
            <button
              v-for="item in filteredNavItems.filter(i => i.category === 'tools')"
              :key="item.id"
              type="button"
              @click="selectedNav = item.id"
              class="w-full flex items-center justify-between px-2 py-1 rounded-[5px] text-left transition-all"
              :class="selectedNav === item.id 
                ? 'bg-[#007aff] text-white font-medium shadow-xs' 
                : 'text-[#1d1d1f] dark:text-[#e5e5ea] hover:bg-black/[0.04] dark:hover:bg-white/[0.05]'"
            >
              <div class="flex items-center gap-2 min-w-0">
                <div 
                  class="w-[18px] h-[18px] rounded-[4px] flex items-center justify-center text-[10px] shadow-xs flex-shrink-0"
                  :class="[item.bgColor, selectedNav === item.id ? 'ring-1 ring-white/30' : '']"
                >
                  <span>{{ item.iconName }}</span>
                </div>
                <span class="text-[12px] truncate">{{ item.name }}</span>
              </div>
              <span
                v-if="item.badge"
                class="text-[9px] px-1.5 py-0.2 rounded-full font-medium"
                :class="selectedNav === item.id 
                  ? 'bg-white/20 text-white' 
                  : 'bg-black/5 dark:bg-white/10 text-[#8e8e93]'"
              >
                {{ item.badge }}
              </span>
            </button>
          </div>
        </div>

      </nav>

      <!-- 底部刷新状态 (紧凑) -->
      <div class="p-2 border-t border-[#d1d1d6]/60 dark:border-[#38383a]/80 flex items-center justify-between text-[10px] text-[#8e8e93]">
        <span class="flex items-center gap-1.5">
          <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
          {{ t("systemReady") }}
        </span>
        <button
          type="button"
          @click="handleRefresh"
          class="p-1 rounded hover:bg-black/5 dark:hover:bg-white/5 text-[#8e8e93] hover:text-[#1d1d1f] dark:hover:text-white transition-colors flex items-center gap-1"
          :title="t('refreshAll')"
        >
          <svg class="w-3.5 h-3.5" :class="loading ? 'animate-spin' : ''" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
          </svg>
        </button>
      </div>

    </aside>

    <!-- ============================================== -->
    <!-- 右侧详情面板 (Detail Content View) -->
    <!-- ============================================== -->
    <main class="flex-1 flex flex-col h-full bg-[#fbfbfd] dark:bg-[#1e1e1e] overflow-hidden">
      
      <!-- 顶部操作栏与可拖拽区域 -->
      <div
        data-tauri-drag-region
        @mousedown="startDrag"
        class="h-10 px-4 flex items-center justify-between border-b border-[#e5e5ea] dark:border-[#2c2c2e] flex-shrink-0 bg-transparent cursor-default"
      >
        <div class="flex items-center gap-1.5">
          <!-- 返回/前进占位图标 (macOS 标准) -->
          <button type="button" class="w-6 h-6 flex items-center justify-center rounded text-[#8e8e93] hover:bg-black/5 dark:hover:bg-white/5 disabled:opacity-30" disabled>
            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
              <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <button type="button" class="w-6 h-6 flex items-center justify-center rounded text-[#8e8e93] hover:bg-black/5 dark:hover:bg-white/5 disabled:opacity-30" disabled>
            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
            </svg>
          </button>
          <span class="text-[13px] font-bold text-[#1d1d1f] dark:text-[#f5f5f7] ml-2">
            {{ selectedNavTitle }}
          </span>
        </div>

        <div class="flex items-center gap-1.5">
          <!-- 语言切换按钮 (中 / EN) -->
          <button
            type="button"
            @click="toggleLocale"
            class="px-2 py-0.5 text-[11px] font-medium rounded-[5px] bg-[#e3e3e8] dark:bg-[#2c2c2e] hover:bg-[#d8d8dd] dark:hover:bg-[#3a3a3c] text-[#1d1d1f] dark:text-[#f5f5f7] transition-colors shadow-xs flex items-center gap-1 cursor-pointer"
            title="Switch Language / 切换语言"
          >
            <span class="text-[10px]">🌐</span>
            <span>{{ locale === 'zh' ? 'EN' : '中' }}</span>
          </button>

          <!-- 刷新按钮 -->
          <button
            type="button"
            @click="handleRefresh"
            :disabled="loading"
            class="px-2 py-0.5 text-[11px] font-medium rounded-[5px] bg-[#e3e3e8] dark:bg-[#2c2c2e] hover:bg-[#d8d8dd] dark:hover:bg-[#3a3a3c] transition-colors flex items-center gap-1.5 shadow-xs cursor-pointer"
          >
            <svg class="w-3 h-3" :class="loading ? 'animate-spin' : ''" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
            </svg>
            <span>{{ t("refresh") }}</span>
          </button>
        </div>
      </div>

      <!-- 内容滚动区域 (仅在滚动中显示滚动条，停止后自动淡出) -->
      <div
        @scroll="handleScroll"
        class="flex-1 overflow-y-auto px-5 py-3.5 space-y-3.5"
        :class="{ 'is-scrolling': isScrolling }"
      >

        <!-- ────────────────────────────────────────── -->
        <!-- 页面 1: 运行时详情 (Java / Node.js / Python) -->
        <!-- ────────────────────────────────────────── -->
        <div v-if="currentRuntime" class="space-y-3.5 max-w-2xl mx-auto">
          
          <!-- 紧凑型顶部运行环境概览条 -->
          <div class="flex items-center gap-3 px-1 py-0.5">
            <div 
              class="w-10 h-10 rounded-xl flex items-center justify-center text-xl shadow-xs flex-shrink-0"
              :class="currentRuntime.name === 'java' ? 'bg-blue-500 text-white' : currentRuntime.name === 'node' ? 'bg-emerald-500 text-white' : 'bg-amber-500 text-white'"
            >
              <span>{{ currentRuntime.icon }}</span>
            </div>
            <div class="min-w-0">
              <h2 class="text-[14px] font-bold tracking-tight text-[#1d1d1f] dark:text-[#f5f5f7] leading-tight">
                {{ currentRuntime.display_name }}
              </h2>
              <p class="text-[11px] text-[#8e8e93] mt-0.5 leading-snug">
                {{ t("runtimeDesc", { manager: currentRuntime.manager }) }}
              </p>
            </div>
          </div>

          <!-- 管理器未安装时：展示一键安装卡片 -->
          <div
            v-if="!currentRuntime.is_available"
            class="bg-white dark:bg-[#252528] rounded-lg border border-[#e5e5ea] dark:border-[#38383a] shadow-xs p-5 text-center space-y-3.5"
          >
            <div
              class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-xs mx-auto"
              :class="currentRuntime.name === 'java' ? 'bg-blue-500/10 text-blue-500' : currentRuntime.name === 'node' ? 'bg-emerald-500/10 text-emerald-500' : 'bg-amber-500/10 text-amber-500'"
            >
              <span>{{ currentRuntime.icon }}</span>
            </div>

            <div>
              <h3 class="text-[14px] font-bold text-[#1d1d1f] dark:text-[#f5f5f7]">
                {{ t("managerNotInstalledTitle", { manager: currentRuntime.manager }) }}
              </h3>
              <p class="text-[11px] text-[#8e8e93] mt-1 max-w-md mx-auto leading-relaxed">
                {{ t("managerNotInstalledDesc", { manager: currentRuntime.manager, runtime: currentRuntime.display_name }) }}
              </p>
            </div>

            <!-- 一键安装按钮 -->
            <div class="pt-1">
              <button
                type="button"
                :disabled="Boolean(installingManager[currentRuntime.manager])"
                @click="handleInstallManager(currentRuntime.manager)"
                class="px-4 py-2 bg-[#007aff] hover:bg-blue-600 text-white rounded-lg text-[12px] font-medium shadow-xs transition-all active:scale-95 disabled:opacity-50 flex items-center gap-2 mx-auto cursor-pointer"
              >
                <svg
                  v-if="installingManager[currentRuntime.manager]"
                  class="w-3.5 h-3.5 animate-spin"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z"></path>
                </svg>
                <svg
                  v-else
                  class="w-3.5 h-3.5"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
                </svg>
                <span>
                  {{ installingManager[currentRuntime.manager] ? t("installingManager", { manager: currentRuntime.manager }) : t("oneClickInstallManager", { manager: currentRuntime.manager }) }}
                </span>
              </button>
            </div>

            <!-- 官方安装命令提示 -->
            <div class="pt-1.5 border-t border-[#f2f2f7] dark:border-[#2c2c2e] text-[10px] text-[#8e8e93]">
              <span>{{ t("officialInstallCmdTip") }}: </span>
              <code class="px-1.5 py-0.5 bg-black/5 dark:bg-white/10 rounded font-mono text-[10px] select-all">
                {{ getManagerInstallCmd(currentRuntime.manager) }}
              </code>
            </div>
          </div>

          <!-- 管理器已安装时：正常展示所有卡片 -->
          <template v-else>
          <!-- macOS 风格的分组卡片 1: 当前版本状态 -->
          <div class="bg-white dark:bg-[#252528] rounded-lg border border-[#e5e5ea] dark:border-[#38383a] shadow-xs overflow-hidden">
            <div class="px-3.5 py-2 flex items-center justify-between border-b border-[#f2f2f7] dark:border-[#2c2c2e]">
              <div class="font-medium text-[12px]">{{ t("currentVersionTitle") }}</div>
              <div class="flex items-center gap-2">
                <span 
                  v-if="currentRuntime.current_version"
                  class="px-2 py-0.5 rounded-full text-[11px] font-mono font-medium bg-blue-50 dark:bg-blue-950/50 text-[#007aff] dark:text-blue-400 border border-blue-200 dark:border-blue-900"
                >
                  {{ currentRuntime.current_version }}
                </span>
                <span v-else class="text-[11px] text-[#8e8e93] italic">
                  {{ t("notConfigured") }}
                </span>
              </div>
            </div>

            <div class="px-3.5 py-1.5 bg-neutral-50/50 dark:bg-white/[0.02] text-[11px] text-[#8e8e93] flex items-center justify-between">
              <span>{{ t("managerLabel") }}</span>
              <span class="font-mono text-[#1d1d1f] dark:text-[#f5f5f7]">{{ currentRuntime.manager }}</span>
            </div>
          </div>

          <!-- Node.js 专属: 镜像源配置卡片 (npm registry & nvm mirror) -->
          <div
            v-if="currentRuntime.name === 'node'"
            class="bg-white dark:bg-[#252528] rounded-lg border border-[#e5e5ea] dark:border-[#38383a] shadow-xs overflow-hidden"
          >
            <!-- 标题栏 + 一键切换快捷按钮 -->
            <div class="px-3.5 py-2.5 border-b border-[#f2f2f7] dark:border-[#2c2c2e] flex items-center justify-between flex-wrap gap-2">
              <div class="flex items-center gap-1.5">
                <span class="text-sm">⚡</span>
                <span class="font-medium text-[12px] text-[#1d1d1f] dark:text-[#f5f5f7]">
                  {{ t("nodeMirrorsCardTitle") }}
                </span>
              </div>
              
              <div class="flex items-center gap-1.5">
                <!-- 一键国内镜像 (淘宝源) -->
                <button
                  type="button"
                  :disabled="settingMirror"
                  @click="handleSetAllChinaMirrors"
                  class="px-2.5 py-1 bg-emerald-600 hover:bg-emerald-700 text-white rounded-[5px] text-[10px] font-medium shadow-xs transition-all active:scale-95 disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                >
                  <span v-if="settingMirror" class="animate-spin">⏳</span>
                  <span v-else>🚀</span>
                  <span>{{ t("quickSetChinaMirrors") }}</span>
                </button>

                <!-- 一键恢复官方源 -->
                <button
                  type="button"
                  :disabled="settingMirror"
                  @click="handleRestoreOfficialMirrors"
                  class="px-2 py-1 bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/20 text-[#1d1d1f] dark:text-[#f5f5f7] rounded-[5px] text-[10px] font-medium transition-all active:scale-95 disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                >
                  <span>{{ t("quickRestoreOfficial") }}</span>
                </button>
              </div>
            </div>

            <div class="p-3.5 space-y-3 divide-y divide-[#f2f2f7] dark:divide-[#2c2c2e]">
              <!-- 1. npm Registry -->
              <div class="space-y-1.5">
                <div class="flex items-center justify-between text-[11px]">
                  <span class="font-medium text-[#1d1d1f] dark:text-[#f5f5f7] flex items-center gap-1">
                    <span>📦</span>
                    <span>{{ t("npmRegistryLabel") }}</span>
                  </span>
                  <span class="text-[10px] font-mono text-emerald-600 dark:text-emerald-400 bg-emerald-50 dark:bg-emerald-950/40 px-2 py-0.5 rounded border border-emerald-200 dark:border-emerald-800 truncate max-w-[240px]">
                    {{ nodeMirrors.npm_registry || 'https://registry.npmjs.org/' }}
                  </span>
                </div>

                <div class="flex items-center gap-1.5 flex-wrap">
                  <button
                    type="button"
                    :disabled="settingMirror"
                    @click="handleSetNpmRegistry('https://registry.npmmirror.com/')"
                    class="px-2 py-0.5 text-[10px] rounded-[5px] border border-[#d1d1d6] dark:border-[#38383a] hover:bg-black/5 dark:hover:bg-white/5 transition-colors disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                    :class="nodeMirrors.npm_registry?.includes('npmmirror') ? 'bg-emerald-50 dark:bg-emerald-950/30 text-emerald-600 dark:text-emerald-400 border-emerald-300 dark:border-emerald-700 font-semibold' : ''"
                  >
                    <span>淘宝/npmmirror</span>
                  </button>
                  <button
                    type="button"
                    :disabled="settingMirror"
                    @click="handleSetNpmRegistry('https://mirrors.cloud.tencent.com/npm/')"
                    class="px-2 py-0.5 text-[10px] rounded-[5px] border border-[#d1d1d6] dark:border-[#38383a] hover:bg-black/5 dark:hover:bg-white/5 transition-colors disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                    :class="nodeMirrors.npm_registry?.includes('tencent') ? 'bg-blue-50 dark:bg-blue-950/30 text-blue-600 dark:text-blue-400 border-blue-300 dark:border-blue-700 font-semibold' : ''"
                  >
                    <span>腾讯云</span>
                  </button>
                  <button
                    type="button"
                    :disabled="settingMirror"
                    @click="handleSetNpmRegistry('https://registry.npmjs.org/')"
                    class="px-2 py-0.5 text-[10px] rounded-[5px] border border-[#d1d1d6] dark:border-[#38383a] hover:bg-black/5 dark:hover:bg-white/5 transition-colors disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                    :class="nodeMirrors.npm_registry?.includes('npmjs.org') ? 'bg-neutral-100 dark:bg-neutral-800 text-[#1d1d1f] dark:text-[#f5f5f7] border-neutral-400 dark:border-neutral-600 font-semibold' : ''"
                  >
                    <span>官方源 (npmjs)</span>
                  </button>
                </div>

                <!-- 自定义 npm 输入 -->
                <div class="flex items-center gap-1.5 pt-0.5">
                  <input
                    v-model="customNpmRegistry"
                    type="text"
                    :placeholder="t('customPlaceholder')"
                    class="flex-1 px-2 py-0.5 bg-[#f2f2f7] dark:bg-[#1c1c1e] text-[10px] text-[#1d1d1f] dark:text-[#f5f5f7] placeholder-[#8e8e93] rounded-[5px] border border-transparent focus:border-emerald-500/50 focus:bg-white dark:focus:bg-[#2c2c2e] focus:outline-none transition-all font-mono h-6"
                    @keydown.enter="handleSetNpmRegistry(customNpmRegistry)"
                  />
                  <button
                    type="button"
                    :disabled="!customNpmRegistry.trim() || settingMirror"
                    @click="handleSetNpmRegistry(customNpmRegistry)"
                    class="px-2 py-0.5 bg-[#007aff] text-white rounded-[5px] text-[10px] font-medium hover:bg-blue-600 disabled:opacity-40 transition-all shadow-xs h-6 cursor-pointer"
                  >
                    {{ t("applyBtn") }}
                  </button>
                </div>
              </div>

              <!-- 2. nvm Node 镜像 -->
              <div class="space-y-1.5 pt-3">
                <div class="flex items-center justify-between text-[11px]">
                  <span class="font-medium text-[#1d1d1f] dark:text-[#f5f5f7] flex items-center gap-1">
                    <span>⬇️</span>
                    <span>{{ t("nvmMirrorLabel") }}</span>
                  </span>
                  <span class="text-[10px] font-mono text-emerald-600 dark:text-emerald-400 bg-emerald-50 dark:bg-emerald-950/40 px-2 py-0.5 rounded border border-emerald-200 dark:border-emerald-800 truncate max-w-[240px]">
                    {{ nodeMirrors.nvm_node_mirror || 'https://nodejs.org/dist' }}
                  </span>
                </div>

                <div class="flex items-center gap-1.5 flex-wrap">
                  <button
                    type="button"
                    :disabled="settingMirror"
                    @click="handleSetNvmMirror('https://npmmirror.com/mirrors/node/')"
                    class="px-2 py-0.5 text-[10px] rounded-[5px] border border-[#d1d1d6] dark:border-[#38383a] hover:bg-black/5 dark:hover:bg-white/5 transition-colors disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                    :class="nodeMirrors.nvm_node_mirror?.includes('npmmirror') ? 'bg-emerald-50 dark:bg-emerald-950/30 text-emerald-600 dark:text-emerald-400 border-emerald-300 dark:border-emerald-700 font-semibold' : ''"
                  >
                    <span>淘宝/npmmirror</span>
                  </button>
                  <button
                    type="button"
                    :disabled="settingMirror"
                    @click="handleSetNvmMirror('https://mirrors.cloud.tencent.com/nodejs-release/')"
                    class="px-2 py-0.5 text-[10px] rounded-[5px] border border-[#d1d1d6] dark:border-[#38383a] hover:bg-black/5 dark:hover:bg-white/5 transition-colors disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                    :class="nodeMirrors.nvm_node_mirror?.includes('tencent') ? 'bg-blue-50 dark:bg-blue-950/30 text-blue-600 dark:text-blue-400 border-blue-300 dark:border-blue-700 font-semibold' : ''"
                  >
                    <span>腾讯云</span>
                  </button>
                  <button
                    type="button"
                    :disabled="settingMirror"
                    @click="handleSetNvmMirror('https://nodejs.org/dist')"
                    class="px-2 py-0.5 text-[10px] rounded-[5px] border border-[#d1d1d6] dark:border-[#38383a] hover:bg-black/5 dark:hover:bg-white/5 transition-colors disabled:opacity-50 flex items-center gap-1 cursor-pointer"
                    :class="nodeMirrors.nvm_node_mirror?.includes('nodejs.org') ? 'bg-neutral-100 dark:bg-neutral-800 text-[#1d1d1f] dark:text-[#f5f5f7] border-neutral-400 dark:border-neutral-600 font-semibold' : ''"
                  >
                    <span>官方源 (nodejs.org)</span>
                  </button>
                </div>

                <!-- 自定义 nvm 输入 -->
                <div class="flex items-center gap-1.5 pt-0.5">
                  <input
                    v-model="customNvmMirror"
                    type="text"
                    :placeholder="t('customPlaceholder')"
                    class="flex-1 px-2 py-0.5 bg-[#f2f2f7] dark:bg-[#1c1c1e] text-[10px] text-[#1d1d1f] dark:text-[#f5f5f7] placeholder-[#8e8e93] rounded-[5px] border border-transparent focus:border-emerald-500/50 focus:bg-white dark:focus:bg-[#2c2c2e] focus:outline-none transition-all font-mono h-6"
                    @keydown.enter="handleSetNvmMirror(customNvmMirror)"
                  />
                  <button
                    type="button"
                    :disabled="!customNvmMirror.trim() || settingMirror"
                    @click="handleSetNvmMirror(customNvmMirror)"
                    class="px-2 py-0.5 bg-[#007aff] text-white rounded-[5px] text-[10px] font-medium hover:bg-blue-600 disabled:opacity-40 transition-all shadow-xs h-6 cursor-pointer"
                  >
                    {{ t("applyBtn") }}
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- macOS 风格的分组卡片 2: 已安装版本列表与切换 -->
          <div>
            <div class="px-1 pb-1 text-[10px] font-semibold text-[#8e8e93] uppercase tracking-wider">
              {{ t("installedVersionsTitle", { count: currentRuntime.versions.length }) }}
            </div>

            <div class="bg-white dark:bg-[#252528] rounded-lg border border-[#e5e5ea] dark:border-[#38383a] shadow-xs divide-y divide-[#f2f2f7] dark:divide-[#2c2c2e] overflow-hidden">
              
              <div v-if="currentRuntime.versions.length === 0" class="p-4 text-center text-xs text-[#8e8e93]">
                {{ t("noVersionsInstalled", { runtime: currentRuntime.name }) }}
              </div>

              <!-- 单行版本项 (紧凑) -->
              <div
                v-for="ver in currentRuntime.versions"
                :key="ver.version"
                class="px-3.5 py-2 flex items-center justify-between hover:bg-black/[0.015] dark:hover:bg-white/[0.02] transition-colors"
              >
                <!-- 左侧版本标题与标签 -->
                <div class="flex items-center gap-2.5 min-w-0">
                  <!-- 状态小圆点 -->
                  <div
                    class="w-2 h-2 rounded-full flex-shrink-0"
                    :class="ver.is_current ? 'bg-[#007aff] ring-2 ring-blue-500/20' : 'bg-[#c7c7cc] dark:bg-[#48484a]'"
                  ></div>

                  <div class="min-w-0">
                    <div class="font-medium text-[12px] text-[#1d1d1f] dark:text-[#f5f5f7] flex items-center gap-2 leading-tight">
                      <span>{{ ver.display_name }}</span>
                      <span
                        v-if="ver.is_current"
                        class="text-[9px] px-1.5 py-0.2 rounded font-medium bg-emerald-100 dark:bg-emerald-950/60 text-emerald-700 dark:text-emerald-400"
                      >
                        {{ t("activeBadge") }}
                      </span>
                    </div>
                    <div class="text-[10px] text-[#8e8e93] font-mono leading-tight mt-0.5 truncate">
                      {{ ver.version }}
                    </div>
                  </div>
                </div>

                <!-- 右侧操作按钮 -->
                <div class="flex items-center gap-1.5">
                  <template v-if="!ver.is_current">
                    <button
                      type="button"
                      :disabled="Boolean(switching[currentRuntime.name]) || Boolean(uninstalling[`${currentRuntime.name}:${ver.version}`])"
                      @click="handleSwitch(currentRuntime.name, ver.version)"
                      class="px-2.5 py-0.5 text-[11px] font-medium rounded-[5px] bg-[#007aff] text-white hover:bg-blue-600 disabled:opacity-50 transition-all shadow-xs active:scale-95 flex items-center gap-1 cursor-pointer"
                    >
                      <svg
                        v-if="switching[currentRuntime.name]"
                        class="w-3 h-3 animate-spin"
                        fill="none"
                        viewBox="0 0 24 24"
                      >
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z"></path>
                      </svg>
                      <span>{{ t("switchBtn") }}</span>
                    </button>

                    <!-- 卸载按钮 -->
                    <button
                      type="button"
                      :disabled="Boolean(switching[currentRuntime.name]) || Boolean(uninstalling[`${currentRuntime.name}:${ver.version}`])"
                      @click="openUninstallConfirm(currentRuntime.name, ver.version, ver.display_name)"
                      class="p-1 text-neutral-400 hover:text-rose-600 dark:hover:text-rose-400 hover:bg-rose-50 dark:hover:bg-rose-950/40 rounded-[5px] transition-colors disabled:opacity-30 cursor-pointer"
                      :title="t('uninstallTitle', { name: ver.display_name })"
                    >
                      <svg
                        v-if="uninstalling[`${currentRuntime.name}:${ver.version}`]"
                        class="w-3.5 h-3.5 animate-spin text-rose-500"
                        fill="none"
                        viewBox="0 0 24 24"
                      >
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z"></path>
                      </svg>
                      <svg
                        v-else
                        class="w-3.5 h-3.5"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="2"
                      >
                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                      </svg>
                    </button>
                  </template>

                  <div v-else class="text-blue-500 dark:text-blue-400 flex items-center gap-1 text-[11px] font-medium pr-1">
                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                    </svg>
                    <span>{{ t("inUse") }}</span>
                  </div>
                </div>

              </div>

            </div>
          </div>

          <!-- macOS 风格的分组卡片 3: 安装新版本 -->
          <div>
            <div class="px-1 pb-1 text-[10px] font-semibold text-[#8e8e93] uppercase tracking-wider">
              {{ t("installCardTitle") }}
            </div>

            <div class="bg-white dark:bg-[#252528] rounded-lg border border-[#e5e5ea] dark:border-[#38383a] shadow-xs p-3 space-y-2.5">
              
              <!-- 快捷推荐版本 -->
              <div v-if="quickInstallVersions.length > 0" class="space-y-1">
                <div class="text-[10px] text-[#8e8e93] font-medium">{{ t("recommendedVersions") }}</div>
                <div class="flex items-center gap-1.5 flex-wrap">
                  <button
                    v-for="qVer in quickInstallVersions"
                    :key="qVer"
                    type="button"
                    :disabled="Boolean(installing[currentRuntime.name])"
                    @click="handleInstall(currentRuntime.name, qVer)"
                    class="px-2 py-0.5 text-[10px] font-mono font-medium rounded-[5px] border border-[#d1d1d6] dark:border-[#38383a] hover:bg-black/5 dark:hover:bg-white/5 disabled:opacity-50 transition-colors flex items-center gap-1 cursor-pointer"
                  >
                    <span>+</span>
                    <span>{{ currentRuntime.name === 'node' ? 'Node ' + qVer : currentRuntime.name === 'python' ? 'Python ' + qVer : qVer }}</span>
                  </button>
                </div>
              </div>

              <!-- 自定义版本输入与安装 -->
              <div class="pt-2 border-t border-[#f2f2f7] dark:border-[#2c2c2e]">
                <div class="text-[10px] text-[#8e8e93] font-medium mb-1">{{ t("enterVersionTitle") }}</div>
                <div class="flex items-center gap-2">
                  <input
                    v-model="customInstallVersion"
                    type="text"
                    :placeholder="currentRuntime.name === 'node' ? t('placeholderNode') : currentRuntime.name === 'python' ? t('placeholderPython') : t('placeholderJava')"
                    class="flex-1 px-2.5 py-1 bg-[#f2f2f7] dark:bg-[#1c1c1e] text-[11px] text-[#1d1d1f] dark:text-[#f5f5f7] placeholder-[#8e8e93] rounded-[6px] border border-transparent focus:border-[#007aff]/50 focus:bg-white dark:focus:bg-[#2c2c2e] focus:outline-none transition-all font-mono h-7"
                    @keydown.enter="handleInstall(currentRuntime.name)"
                  />
                  <button
                    type="button"
                    :disabled="!customInstallVersion.trim() || Boolean(installing[currentRuntime.name])"
                    @click="handleInstall(currentRuntime.name)"
                    class="px-3 py-1 bg-[#007aff] text-white rounded-[6px] text-[11px] font-medium hover:bg-blue-600 disabled:opacity-40 transition-all shadow-xs flex items-center gap-1.5 flex-shrink-0 h-7 cursor-pointer"
                  >
                    <svg
                      v-if="installing[currentRuntime.name]"
                      class="w-3 h-3 animate-spin"
                      fill="none"
                      viewBox="0 0 24 24"
                    >
                      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z"></path>
                    </svg>
                    <span>{{ installing[currentRuntime.name] ? t("installing") : t("startInstall") }}</span>
                  </button>
                </div>
                <div class="text-[10px] text-[#8e8e93] mt-1">
                  {{ t("installTip", { manager: currentRuntime.manager }) }}
                </div>
              </div>

            </div>
          </div>
          </template>

        </div>

        <!-- ────────────────────────────────────────── -->
        <!-- 页面 2: 环境预设 (Profiles: 像素级还原 macOS 原生分组表格与 Switch 开关) -->
        <!-- ────────────────────────────────────────── -->
        <div v-else-if="selectedNav === 'profiles'" class="space-y-3 max-w-2xl mx-auto">
          
          <div class="space-y-0.5">
            <h2 class="text-[14px] font-bold tracking-tight text-[#1d1d1f] dark:text-[#f5f5f7]">
              {{ t("profilesTitle") }}
            </h2>
            <p class="text-[11px] text-[#8e8e93]">
              {{ t("profilesSubtitle") }}
            </p>
          </div>

          <!-- 仿图 2 的 macOS 表格容器 -->
          <div class="rounded-lg border border-[#d1d1d6] dark:border-[#38383a] bg-white dark:bg-[#252528] shadow-xs overflow-hidden text-[12px]">
            
            <!-- 仿图 2 顶部说明灰字 -->
            <div class="px-3.5 py-2 bg-[#f5f5f7]/60 dark:bg-white/[0.02] border-b border-[#e5e5ea] dark:border-[#2c2c2e] text-[11px] text-[#636366] dark:text-[#8e8e93]">
              {{ t("profilesTip") }}
            </div>

            <!-- 空状态 -->
            <div v-if="profiles.length === 0" class="py-8 text-center text-xs text-[#8e8e93]">
              {{ t("noProfiles") }}
            </div>

            <!-- 列表行 (仿图 2 Alfred / node 行) -->
            <div v-else class="divide-y divide-[#f2f2f7] dark:divide-[#2c2c2e]">
              <div
                v-for="p in profiles"
                :key="p.id"
                @click="selectedProfileId = p.id"
                class="px-3.5 py-2 flex items-center justify-between transition-colors cursor-default select-none"
                :class="[
                  selectedProfileId === p.id 
                    ? 'bg-[#007aff]/10 dark:bg-[#007aff]/20' 
                    : 'hover:bg-black/[0.02] dark:hover:bg-white/[0.02]'
                ]"
              >
                <!-- 左侧图标与文本 -->
                <div class="flex items-center gap-2.5 min-w-0">
                  <!-- 应用风格圆角小图标 -->
                  <div class="w-7 h-7 rounded-[6px] bg-gradient-to-tr from-indigo-500 to-purple-600 flex items-center justify-center text-white text-[13px] shadow-xs flex-shrink-0">
                    <span>📋</span>
                  </div>

                  <div class="min-w-0">
                    <div class="font-medium text-[12px] text-[#1d1d1f] dark:text-[#f5f5f7] flex items-center gap-1.5 leading-tight">
                      <span>{{ p.name }}</span>
                      <button
                        type="button"
                        @click.stop="openEditProfileModal(p)"
                        class="text-[11px] text-[#8e8e93] hover:text-[#007aff] transition-colors p-0.5 cursor-pointer"
                        :title="t('editProfile')"
                      >
                        <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125" />
                        </svg>
                      </button>
                    </div>

                    <div class="text-[10px] text-[#8e8e93] mt-0.5 flex items-center gap-2 truncate font-mono leading-tight">
                      <span v-if="p.java_version">☕ {{ p.java_version }}</span>
                      <span v-if="p.node_version">⬢ {{ p.node_version }}</span>
                      <span v-if="p.python_version">🐍 {{ p.python_version }}</span>
                    </div>
                  </div>
                </div>

                <!-- 右侧: 仿图 2 的 macOS 原生 Switch 开关 -->
                <div class="flex items-center gap-2">
                  <div
                    v-if="applyingProfileId === p.id"
                    class="w-3.5 h-3.5 animate-spin text-[#007aff] mr-1"
                  >
                    <svg fill="none" viewBox="0 0 24 24">
                      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z"></path>
                    </svg>
                  </div>

                  <!-- 现代 macOS 原生 Toggle Switch 开关 -->
                  <button
                    type="button"
                    role="switch"
                    :aria-checked="isProfileActive(p)"
                    :disabled="Boolean(applyingProfileId)"
                    @click.stop="handleApplyProfile(p.id)"
                    class="relative inline-flex h-[20px] w-[34px] flex-shrink-0 cursor-pointer rounded-full p-[2px] transition-colors duration-250 ease-in-out focus:outline-none disabled:opacity-50"
                    :class="isProfileActive(p) ? 'bg-[#34c759]' : 'bg-[#e5e5ea] dark:bg-[#39393d]'"
                    :title="isProfileActive(p) ? t('profileActive') : t('profileClickToActive')"
                  >
                    <span
                      class="pointer-events-none inline-block h-[16px] w-[16px] transform rounded-full bg-white shadow-[0_2px_4px_rgba(0,0,0,0.18),0_1px_1px_rgba(0,0,0,0.12)] transition duration-200 ease-in-out"
                      :class="isProfileActive(p) ? 'translate-x-[14px]' : 'translate-x-0'"
                    />
                  </button>
                </div>

              </div>
            </div>

            <!-- 仿图 2 底部工具栏条: + 和 — 按钮 -->
            <div class="h-7 bg-[#f5f5f7] dark:bg-[#1f1f21] border-t border-[#e5e5ea] dark:border-[#38383a] flex items-center px-1">
              <!-- + 按钮 -->
              <button
                type="button"
                @click="openNewProfileModal"
                class="w-6 h-5 flex items-center justify-center rounded text-neutral-600 dark:text-neutral-300 hover:bg-black/5 dark:hover:bg-white/5 transition-colors cursor-pointer"
                :title="t('addProfileBtn')"
              >
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
              </button>

              <!-- 细分割竖线 -->
              <div class="h-3 w-[1px] bg-[#d1d1d6] dark:bg-[#38383a] mx-0.5"></div>

              <!-- — 按钮 -->
              <button
                type="button"
                :disabled="!selectedProfileId"
                @click="handleDeleteSelectedProfile"
                class="w-6 h-5 flex items-center justify-center rounded text-neutral-600 dark:text-neutral-300 hover:bg-black/5 dark:hover:bg-white/5 disabled:opacity-30 transition-colors cursor-pointer"
                :title="t('deleteProfileBtn')"
              >
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12h-15" />
                </svg>
              </button>
            </div>

          </div>

        </div>

        <!-- ────────────────────────────────────────── -->
        <!-- 页面 3: 通用与关于 (About) -->
        <!-- ────────────────────────────────────────── -->
        <div v-else-if="selectedNav === 'about'" class="space-y-3 max-w-2xl mx-auto">
          
          <div class="flex items-center gap-3 px-1 py-0.5">
            <div class="w-10 h-10 rounded-xl bg-neutral-600 flex items-center justify-center text-xl shadow-xs text-white flex-shrink-0">
              ⚙️
            </div>
            <div>
              <h2 class="text-[14px] font-bold tracking-tight text-[#1d1d1f] dark:text-[#f5f5f7] leading-tight">
                {{ t("aboutTitle") }}
              </h2>
              <p class="text-[11px] text-[#8e8e93] mt-0.5 leading-snug">
                {{ t("aboutSubtitle") }}
              </p>
            </div>
          </div>

          <div class="bg-white dark:bg-[#252528] rounded-lg border border-[#e5e5ea] dark:border-[#38383a] shadow-xs divide-y divide-[#f2f2f7] dark:divide-[#2c2c2e]">
            
            <!-- 语言切换选项 -->
            <div class="px-3.5 py-2 flex items-center justify-between text-[11px]">
              <span class="text-[#8e8e93]">{{ t("languageSetting") }}</span>
              <div class="flex items-center gap-1 bg-black/5 dark:bg-white/10 p-0.5 rounded-[5px]">
                <button
                  type="button"
                  @click="setLocale('zh')"
                  class="px-2 py-0.5 rounded-[4px] text-[10px] font-medium transition-all cursor-pointer"
                  :class="locale === 'zh' ? 'bg-white dark:bg-[#3a3a3c] shadow-xs text-[#007aff]' : 'text-[#8e8e93] hover:text-[#1d1d1f] dark:hover:text-white'"
                >
                  简体中文
                </button>
                <button
                  type="button"
                  @click="setLocale('en')"
                  class="px-2 py-0.5 rounded-[4px] text-[10px] font-medium transition-all cursor-pointer"
                  :class="locale === 'en' ? 'bg-white dark:bg-[#3a3a3c] shadow-xs text-[#007aff]' : 'text-[#8e8e93] hover:text-[#1d1d1f] dark:hover:text-white'"
                >
                  English
                </button>
              </div>
            </div>

            <div class="px-3.5 py-2 flex items-center justify-between text-[11px]">
              <span class="text-[#8e8e93]">{{ t("shortcutSearch") }}</span>
              <span class="font-mono bg-black/5 dark:bg-white/10 px-2 py-0.5 rounded text-[10px]">⌘ + F</span>
            </div>
            <div class="px-3.5 py-2 flex items-center justify-between text-[11px]">
              <span class="text-[#8e8e93]">{{ t("configFilePath") }}</span>
              <span class="font-mono text-[10px] text-[#1d1d1f] dark:text-[#f5f5f7]">~/.config/multi-env/profiles.json</span>
            </div>
            <div class="px-3.5 py-2 flex items-center justify-between text-[11px]">
              <span class="text-[#8e8e93]">{{ t("sdkmanPath") }}</span>
              <span class="font-mono text-[10px] text-[#1d1d1f] dark:text-[#f5f5f7]">~/.sdkman</span>
            </div>
            <div class="px-3.5 py-2 flex items-center justify-between text-[11px]">
              <span class="text-[#8e8e93]">{{ t("nvmPath") }}</span>
              <span class="font-mono text-[10px] text-[#1d1d1f] dark:text-[#f5f5f7]">~/.nvm</span>
            </div>
            <div class="px-3.5 py-2 flex items-center justify-between text-[11px]">
              <span class="text-[#8e8e93]">{{ t("uvExecutable") }}</span>
              <span class="font-mono text-[10px] text-[#1d1d1f] dark:text-[#f5f5f7]">/opt/homebrew/bin/uv</span>
            </div>
          </div>

        </div>

      </div>

    </main>

    <!-- Profile 编辑/新建模态框 -->
    <div
      v-if="showProfileModal && editingProfile"
      class="fixed inset-0 bg-black/45 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    >
      <div class="w-full max-w-md animate-in fade-in zoom-in-95 duration-150">
        <ProfileForm
          :initial-profile="editingProfile"
          :runtimes="runtimes"
          @save="handleSaveProfile"
          @cancel="showProfileModal = false"
        />
      </div>
    </div>

    <!-- macOS 原生风格: 卸载二次确认弹窗 -->
    <div
      v-if="confirmDialog.show"
      class="fixed inset-0 bg-black/45 backdrop-blur-sm z-50 flex items-center justify-center p-4 select-none"
    >
      <div class="bg-white dark:bg-[#28282a] rounded-2xl shadow-2xl border border-black/10 dark:border-white/10 w-full max-w-sm overflow-hidden p-5 text-center space-y-3.5 animate-in fade-in zoom-in-95 duration-150">
        
        <!-- 警示图标 -->
        <div class="w-12 h-12 rounded-full bg-rose-100 dark:bg-rose-950/50 text-rose-600 dark:text-rose-400 mx-auto flex items-center justify-center shadow-inner">
          <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
          </svg>
        </div>

        <div>
          <h3 class="font-bold text-[14px] text-[#1d1d1f] dark:text-[#f5f5f7]">
            {{ t("confirmUninstallTitle") }}
          </h3>
          <p class="text-[12px] text-[#8e8e93] mt-1.5 px-2">
            {{ t("confirmUninstallDesc", { name: confirmDialog.displayName }) }}
          </p>
        </div>

        <!-- 弹窗底部操作按钮 -->
        <div class="grid grid-cols-2 gap-2 pt-2">
          <button
            type="button"
            @click="cancelUninstallConfirm"
            class="w-full py-1.5 rounded-lg border border-[#d1d1d6] dark:border-[#38383a] text-xs font-medium text-neutral-700 dark:text-neutral-300 hover:bg-black/5 dark:hover:bg-white/5 transition-colors cursor-pointer"
          >
            {{ t("cancel") }}
          </button>
          <button
            type="button"
            @click="executeUninstall"
            class="w-full py-1.5 rounded-lg bg-rose-600 hover:bg-rose-500 text-white text-xs font-semibold shadow-sm transition-all active:scale-95 flex items-center justify-center gap-1.5 cursor-pointer"
          >
            <span>{{ t("confirmUninstallBtn") }}</span>
          </button>
        </div>

      </div>
    </div>

    <!-- macOS 风格底部悬浮通知 (Toast) -->
    <StatusNotification
      :show="toast.show"
      :success="toast.success"
      :message="toast.message"
      @close="closeNotification"
    />

  </div>
</template>
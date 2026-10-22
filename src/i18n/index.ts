import { ref } from "vue";

export type Locale = "zh" | "en";

const STORAGE_KEY = "multi_env_locale";

// 获取初始语言设置：优先读取 localStorage，其次读取系统语言
const getInitialLocale = (): Locale => {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved === "zh" || saved === "en") {
    return saved;
  }
  if (typeof navigator !== "undefined" && navigator.language) {
    if (navigator.language.toLowerCase().startsWith("zh")) {
      return "zh";
    }
  }
  return "zh"; // 默认中文
};

export const currentLocale = ref<Locale>(getInitialLocale());

export const setLocale = (locale: Locale) => {
  currentLocale.value = locale;
  localStorage.setItem(STORAGE_KEY, locale);
};

export const toggleLocale = () => {
  setLocale(currentLocale.value === "zh" ? "en" : "zh");
};

export const messages = {
  zh: {
    // 侧边栏
    searchPlaceholder: "搜索",
    categoryRuntimes: "开发环境运行时",
    categoryTools: "管理与工具",
    javaEnv: "Java 环境",
    nodeEnv: "Node.js 环境",
    pythonEnv: "Python 环境",
    profiles: "环境预设 (Profiles)",
    about: "通用与信息",
    systemReady: "系统就绪",
    refreshAll: "刷新全部环境",

    // 顶栏
    refresh: "刷新",
    langSwitch: "English",

    // 运行时概览与管理器安装
    runtimeDesc: "由 {manager} 管理与分发，切换后自动更新终端及全局默认版本。",
    managerNotDetected: "未检测到 {manager}",
    managerNotDetectedDesc: "请确认 {manager} 已正确安装并配置在您的 Shell 环境中。",
    managerNotInstalledTitle: "未检测到 {manager} 环境管理器",
    managerNotInstalledDesc: "当前系统尚未安装 {manager}，无法管理与切换 {runtime} 版本。点击下方按钮即可全自动一键安装。",
    oneClickInstallManager: "一键安装 {manager}",
    installingManager: "正在安装 {manager}，请稍候...",
    installManagerSuccessNotice: "{manager} 已成功安装！",
    installManagerFailedNotice: "安装 {manager} 失败: {msg}",
    officialInstallCmdTip: "官方安装脚本",

    // 卡片 1: 当前版本
    currentVersionTitle: "当前使用版本",
    notConfigured: "未配置 / 未激活",
    managerLabel: "环境管理器",

    // 卡片 2: 已安装版本
    installedVersionsTitle: "已安装版本 ({count})",
    noVersionsInstalled: "未发现本地安装的 {runtime} 版本。",
    activeBadge: "当前激活",
    inUse: "使用中",
    switchBtn: "切换",
    uninstallTitle: "卸载 {name}",

    // 卡片 3: 下载与安装
    installCardTitle: "下载与安装新版本",
    recommendedVersions: "推荐安装版本：",
    enterVersionTitle: "输入版本号安装：",
    placeholderNode: "例如 20.18.0 或 22",
    placeholderPython: "例如 3.11.8",
    placeholderJava: "例如 21.0.2-tem",
    startInstall: "开始安装",
    installing: "安装中...",
    installTip: "提示：通过 {manager} 在后台下载安装，过程视网络情况可能需要少许时间。",

    // 预设页面
    profilesTitle: "环境预设 (Profiles)",
    profilesSubtitle: "一键切换整套配置方案，将开发环境一键转为项目专用的 Java + Node + Python 组合。",
    profilesTip: "启用下方的环境预设方案以切换运行时配置。",
    noProfiles: "暂无已保存的环境预设，点击下方 「+」 添加。",
    editProfile: "编辑配置",
    profileActive: "已激活",
    profileClickToActive: "点击启用此预设",
    addProfileBtn: "添加新预设 (+)",
    deleteProfileBtn: "删除选中的预设 (—)",

    // 预设表单模态框
    newProfileTitle: "新建环境预设",
    editProfileTitle: "编辑环境预设",
    profileNameLabel: "预设方案名称",
    profileNamePlaceholder: "例如：全栈项目 A、Spring Boot 3 + Node 20...",
    javaRuntimeLabel: "Java 运行时 (SDKMAN)",
    nodeRuntimeLabel: "Node.js 运行时 (nvm)",
    pythonRuntimeLabel: "Python 运行时 (uv)",
    keepUnchanged: "-- 保持不变 (不切换) --",
    cancel: "取消",
    saveProfile: "保存预设",

    // 卸载确认模态框
    confirmUninstallTitle: "确定要卸载该版本吗？",
    confirmUninstallDesc: "即将从系统彻底移除 {name}。此操作无法撤销。",
    confirmUninstallBtn: "确定卸载",

    // 通用与关于
    aboutTitle: "关于 Multi-Env Switcher",
    aboutSubtitle: "版本 0.1.0 • 基于 Tauri v2 + Vue 3 打造的原生 macOS 环境助手",
    languageSetting: "界面语言",
    shortcutSearch: "快捷搜索",
    configFilePath: "预设配置文件路径",
    sdkmanPath: "SDKMAN 路径",
    nvmPath: "nvm 路径",
    uvExecutable: "uv 可执行程序",

    // 通知提示
    refreshedNotice: "已刷新最新环境与配置",
    installingNotice: "正在安装 {runtime} {version}，请稍候...",
    installSuccessNotice: "已成功安装 {version}",
    installFailedNotice: "安装失败: {msg}",
    uninstallingNotice: "正在卸载 {name}...",
    uninstallSuccessNotice: "已成功卸载 {name}",
    uninstallFailedNotice: "卸载失败: {msg}",
    switchSuccessNotice: "已成功切换为 {version}",
    switchFailedNotice: "切换失败: {version}",
    profileAppliedNotice: "已应用环境预设 \"{name}\"",
    profileApplyFailedNotice: "应用环境预设 \"{name}\" 失败",
    profileApplyException: "应用预设异常: {msg}",
    profileSavedNotice: "预设 \"{name}\" 已保存",
    profileSaveFailedNotice: "保存预设失败",
    profileDeletedNotice: "预设已删除",
    profileDeleteFailedNotice: "删除预设失败",

    // Node.js 镜像源管理
    nodeMirrorsCardTitle: "Node.js 镜像源与加速配置",
    nodeMirrorsDesc: "一键配置 npm registry 和 nvm node 下载镜像，大幅提升依赖安装与 Node 版本下载速度。",
    quickSetChinaMirrors: "⚡ 一键换国内镜像 (淘宝源)",
    quickRestoreOfficial: "🌐 恢复官方默认源",
    npmRegistryLabel: "npm 镜像源 (registry)",
    nvmMirrorLabel: "nvm Node 下载源 (dist mirror)",
    currentSetting: "当前：",
    mirrorTaobao: "淘宝源",
    mirrorTencent: "腾讯云",
    mirrorOfficial: "官方源",
    customPlaceholder: "输入自定义镜像 URL...",
    applyBtn: "设置",
    npmRegistrySuccessNotice: "npm 镜像已成功设置为: {url}",
    nvmMirrorSuccessNotice: "nvm 镜像已成功设置为: {url}",
    allChinaMirrorsSuccessNotice: "已成功将 npm 与 nvm 切换为淘宝/npmmirror 加速源！",
    allOfficialMirrorsSuccessNotice: "已恢复 npm 与 nvm 官方源！",
  },
  en: {
    // Sidebar
    searchPlaceholder: "Search",
    categoryRuntimes: "RUNTIMES",
    categoryTools: "TOOLS & PRESETS",
    javaEnv: "Java Environment",
    nodeEnv: "Node.js Environment",
    pythonEnv: "Python Environment",
    profiles: "Profiles",
    about: "General & About",
    systemReady: "Ready",
    refreshAll: "Refresh all environments",

    // Top bar
    refresh: "Refresh",
    langSwitch: "中文",

    // Runtime overview & manager installation
    runtimeDesc: "Managed by {manager}. Switching updates terminal and global default versions.",
    managerNotDetected: "{manager} Not Detected",
    managerNotDetectedDesc: "Please make sure {manager} is installed and configured in your shell environment.",
    managerNotInstalledTitle: "{manager} Not Detected",
    managerNotInstalledDesc: "{manager} is not installed on this system. You can install it automatically with one click below.",
    oneClickInstallManager: "Install {manager} with One Click",
    installingManager: "Installing {manager}, please wait...",
    installManagerSuccessNotice: "{manager} has been successfully installed!",
    installManagerFailedNotice: "Failed to install {manager}: {msg}",
    officialInstallCmdTip: "Official install command",

    // Card 1: Current version
    currentVersionTitle: "Active Version",
    notConfigured: "Not configured / Inactive",
    managerLabel: "Manager",

    // Card 2: Installed versions
    installedVersionsTitle: "Installed Versions ({count})",
    noVersionsInstalled: "No local versions of {runtime} found.",
    activeBadge: "Active",
    inUse: "In Use",
    switchBtn: "Switch",
    uninstallTitle: "Uninstall {name}",

    // Card 3: Download & install
    installCardTitle: "Download & Install New Version",
    recommendedVersions: "Recommended versions:",
    enterVersionTitle: "Enter version to install:",
    placeholderNode: "e.g. 20.18.0 or 22",
    placeholderPython: "e.g. 3.11.8",
    placeholderJava: "e.g. 21.0.2-tem",
    startInstall: "Install",
    installing: "Installing...",
    installTip: "Note: Downloads and installs via {manager} in background. May take a moment depending on network.",

    // Profiles page
    profilesTitle: "Profiles",
    profilesSubtitle: "Switch entire runtime configurations with one click for dedicated project environments.",
    profilesTip: "Enable a profile below to switch your runtime configuration.",
    noProfiles: "No saved profiles yet. Click '+' below to add one.",
    editProfile: "Edit configuration",
    profileActive: "Active",
    profileClickToActive: "Click to activate this profile",
    addProfileBtn: "Add new profile (+)",
    deleteProfileBtn: "Delete selected profile (—)",

    // Profile form modal
    newProfileTitle: "New Environment Profile",
    editProfileTitle: "Edit Environment Profile",
    profileNameLabel: "Profile Name",
    profileNamePlaceholder: "e.g. Fullstack Project, Spring Boot 3 + Node 20...",
    javaRuntimeLabel: "Java Runtime (SDKMAN)",
    nodeRuntimeLabel: "Node.js Runtime (nvm)",
    pythonRuntimeLabel: "Python Runtime (uv)",
    keepUnchanged: "-- Keep unchanged (Don't switch) --",
    cancel: "Cancel",
    saveProfile: "Save Profile",

    // Uninstall confirmation modal
    confirmUninstallTitle: "Are you sure you want to uninstall?",
    confirmUninstallDesc: "This will permanently remove {name} from your system. This action cannot be undone.",
    confirmUninstallBtn: "Uninstall",

    // General & About
    aboutTitle: "About Multi-Env Switcher",
    aboutSubtitle: "Version 0.1.0 • Native macOS Environment Switcher built with Tauri v2 + Vue 3",
    languageSetting: "Language",
    shortcutSearch: "Quick Search",
    configFilePath: "Profiles Config Path",
    sdkmanPath: "SDKMAN Path",
    nvmPath: "nvm Path",
    uvExecutable: "uv Executable",

    // Notifications
    refreshedNotice: "Environments and profiles refreshed",
    installingNotice: "Installing {runtime} {version}, please wait...",
    installSuccessNotice: "Successfully installed {version}",
    installFailedNotice: "Installation failed: {msg}",
    uninstallingNotice: "Uninstalling {name}...",
    uninstallSuccessNotice: "Successfully uninstalled {name}",
    uninstallFailedNotice: "Uninstall failed: {msg}",
    switchSuccessNotice: "Switched to {version}",
    switchFailedNotice: "Failed to switch to {version}",
    profileAppliedNotice: "Applied profile \"{name}\"",
    profileApplyFailedNotice: "Failed to apply profile \"{name}\"",
    profileApplyException: "Error applying profile: {msg}",
    profileSavedNotice: "Profile \"{name}\" saved",
    profileSaveFailedNotice: "Failed to save profile",
    profileDeletedNotice: "Profile deleted",
    profileDeleteFailedNotice: "Failed to delete profile",

    // Node.js Mirrors Management
    nodeMirrorsCardTitle: "Node.js Mirrors & Acceleration",
    nodeMirrorsDesc: "One-click switch for npm registry and nvm download mirror for faster package and version downloads.",
    quickSetChinaMirrors: "⚡ Set China Mirrors (Taobao)",
    quickRestoreOfficial: "🌐 Restore Official Mirrors",
    npmRegistryLabel: "npm Registry",
    nvmMirrorLabel: "nvm Node Mirror (dist)",
    currentSetting: "Current: ",
    mirrorTaobao: "Taobao",
    mirrorTencent: "Tencent",
    mirrorOfficial: "Official",
    customPlaceholder: "Enter custom mirror URL...",
    applyBtn: "Set",
    npmRegistrySuccessNotice: "npm registry set to: {url}",
    nvmMirrorSuccessNotice: "nvm node mirror set to: {url}",
    allChinaMirrorsSuccessNotice: "Successfully switched npm and nvm to Taobao/npmmirror acceleration mirrors!",
    allOfficialMirrorsSuccessNotice: "Successfully restored npm and nvm to official mirrors!",
  },
};

export type MessageKey = keyof typeof messages.zh;

export const t = (key: MessageKey, params?: Record<string, string | number>): string => {
  const localeMessages = messages[currentLocale.value] || messages.zh;
  let text = (localeMessages as any)[key] || (messages.zh as any)[key] || key;
  if (params) {
    Object.entries(params).forEach(([k, v]) => {
      text = text.replace(new RegExp(`\\{${k}\\}`, "g"), String(v));
    });
  }
  return text;
};

export const useI18n = () => {
  return {
    locale: currentLocale,
    setLocale,
    toggleLocale,
    t,
  };
};

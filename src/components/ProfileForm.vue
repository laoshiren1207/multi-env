<script setup lang="ts">
import { ref, computed } from "vue";
import type { Profile, RuntimeInfo } from "../types";
import { useI18n } from "../i18n";

const { t } = useI18n();

const props = defineProps<{
  initialProfile?: Profile | null;
  runtimes: RuntimeInfo[];
}>();

const emit = defineEmits<{
  (e: "save", profile: Profile): void;
  (e: "cancel"): void;
}>();

const name = ref(props.initialProfile?.name || "");
const javaVersion = ref<string | null>(props.initialProfile?.java_version || null);
const nodeVersion = ref<string | null>(props.initialProfile?.node_version || null);
const pythonVersion = ref<string | null>(props.initialProfile?.python_version || null);

// Helpers to get versions for each runtime
const javaVersions = computed(() => {
  const rt = props.runtimes.find((r) => r.name.toLowerCase().includes("java"));
  return rt ? rt.versions : [];
});

const nodeVersions = computed(() => {
  const rt = props.runtimes.find((r) => r.name.toLowerCase().includes("node"));
  return rt ? rt.versions : [];
});

const pythonVersions = computed(() => {
  const rt = props.runtimes.find((r) => r.name.toLowerCase().includes("python"));
  return rt ? rt.versions : [];
});

const isFormValid = computed(() => {
  return name.value.trim().length > 0;
});

const handleSave = () => {
  if (!isFormValid.value) return;

  const profile: Profile = {
    id: props.initialProfile?.id || (typeof crypto !== "undefined" && crypto.randomUUID ? crypto.randomUUID() : `p_${Date.now()}`),
    name: name.value.trim(),
    java_version: javaVersion.value || null,
    node_version: nodeVersion.value || null,
    python_version: pythonVersion.value || null,
  };

  emit("save", profile);
};
</script>

<template>
  <div class="p-4 rounded-xl bg-white/95 dark:bg-[#252528]/95 backdrop-blur-2xl shadow-2xl border border-black/10 dark:border-white/10 space-y-3.5 select-none">
    
    <!-- 头部标题 -->
    <div class="flex items-center justify-between pb-1 border-b border-[#f2f2f7] dark:border-[#38383a]">
      <div class="flex items-center gap-2">
        <div class="w-6 h-6 rounded-md bg-gradient-to-tr from-blue-600 to-indigo-500 flex items-center justify-center text-white text-xs shadow-sm">
          📋
        </div>
        <h3 class="text-[13px] font-bold text-[#1d1d1f] dark:text-[#f5f5f7]">
          {{ initialProfile && initialProfile.id ? t("editProfileTitle") : t("newProfileTitle") }}
        </h3>
      </div>
      <button
        type="button"
        class="w-6 h-6 rounded-full flex items-center justify-center text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-200 hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
        @click="emit('cancel')"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- 方案名称输入 -->
    <div class="space-y-1">
      <label class="block text-[11px] font-semibold text-[#1d1d1f] dark:text-[#e5e5ea]">
        {{ t("profileNameLabel") }}
      </label>
      <input
        v-model="name"
        type="text"
        :placeholder="t('profileNamePlaceholder')"
        class="w-full px-2.5 py-1.5 text-[11px] rounded-lg border border-[#d1d1d6] dark:border-[#3a3a3c] bg-[#fbfbfd] dark:bg-[#1c1c1e] text-[#1d1d1f] dark:text-[#f5f5f7] placeholder-[#8e8e93] focus:outline-none focus:ring-2 focus:ring-[#007aff]/30 focus:border-[#007aff] transition-all shadow-inner"
        autofocus
      />
    </div>

    <!-- 分组选择表单卡片 (仿 macOS 原生设置项) -->
    <div class="rounded-lg border border-[#e5e5ea] dark:border-[#38383a] bg-[#f9f9fb] dark:bg-[#1e1e20] p-2.5 space-y-2.5">
      
      <!-- Java 版本选择 -->
      <div class="space-y-1">
        <label class="text-[10px] font-medium text-[#8e8e93] flex items-center gap-1.5">
          <span class="w-4 h-4 rounded-[4px] bg-blue-500/10 text-blue-600 dark:text-blue-400 flex items-center justify-center text-[10px]">☕</span>
          <span>{{ t("javaRuntimeLabel") }}</span>
        </label>
        <div class="relative">
          <select
            v-model="javaVersion"
            class="w-full appearance-none px-2.5 py-1 pr-7 text-[11px] rounded-md border border-[#d1d1d6] dark:border-[#3a3a3c] bg-white dark:bg-[#2c2c2e] text-[#1d1d1f] dark:text-[#f5f5f7] focus:outline-none focus:ring-2 focus:ring-[#007aff]/30 focus:border-[#007aff] transition-all font-mono shadow-xs cursor-pointer"
          >
            <option :value="null">{{ t("keepUnchanged") }}</option>
            <option v-for="ver in javaVersions" :key="ver.version" :value="ver.version">
              {{ ver.display_name || ver.version }}
            </option>
          </select>
          <div class="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 text-neutral-400">
            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 15L12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Node.js 版本选择 -->
      <div class="space-y-1">
        <label class="text-[10px] font-medium text-[#8e8e93] flex items-center gap-1.5">
          <span class="w-4 h-4 rounded-[4px] bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 flex items-center justify-center text-[10px]">⬢</span>
          <span>{{ t("nodeRuntimeLabel") }}</span>
        </label>
        <div class="relative">
          <select
            v-model="nodeVersion"
            class="w-full appearance-none px-2.5 py-1 pr-7 text-[11px] rounded-md border border-[#d1d1d6] dark:border-[#3a3a3c] bg-white dark:bg-[#2c2c2e] text-[#1d1d1f] dark:text-[#f5f5f7] focus:outline-none focus:ring-2 focus:ring-[#007aff]/30 focus:border-[#007aff] transition-all font-mono shadow-xs cursor-pointer"
          >
            <option :value="null">{{ t("keepUnchanged") }}</option>
            <option v-for="ver in nodeVersions" :key="ver.version" :value="ver.version">
              {{ ver.display_name || ver.version }}
            </option>
          </select>
          <div class="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 text-neutral-400">
            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 15L12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Python 版本选择 -->
      <div class="space-y-1">
        <label class="text-[10px] font-medium text-[#8e8e93] flex items-center gap-1.5">
          <span class="w-4 h-4 rounded-[4px] bg-amber-500/10 text-amber-600 dark:text-amber-400 flex items-center justify-center text-[10px]">🐍</span>
          <span>{{ t("pythonRuntimeLabel") }}</span>
        </label>
        <div class="relative">
          <select
            v-model="pythonVersion"
            class="w-full appearance-none px-2.5 py-1 pr-7 text-[11px] rounded-md border border-[#d1d1d6] dark:border-[#3a3a3c] bg-white dark:bg-[#2c2c2e] text-[#1d1d1f] dark:text-[#f5f5f7] focus:outline-none focus:ring-2 focus:ring-[#007aff]/30 focus:border-[#007aff] transition-all font-mono shadow-xs cursor-pointer"
          >
            <option :value="null">{{ t("keepUnchanged") }}</option>
            <option v-for="ver in pythonVersions" :key="ver.version" :value="ver.version">
              {{ ver.display_name || ver.version }}
            </option>
          </select>
          <div class="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 text-neutral-400">
            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 15L12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" />
            </svg>
          </div>
        </div>
      </div>

    </div>

    <!-- 底部操作按钮 -->
    <div class="flex items-center justify-end gap-2 pt-1.5 border-t border-[#f2f2f7] dark:border-[#38383a]">
      <button
        type="button"
        class="px-3 py-1 rounded-lg text-[11px] font-medium text-[#636366] dark:text-[#a1a1a6] hover:bg-black/5 dark:hover:bg-white/5 transition-colors"
        @click="emit('cancel')"
      >
        {{ t("cancel") }}
      </button>
      <button
        type="button"
        :disabled="!isFormValid"
        class="px-3.5 py-1 rounded-lg text-[11px] font-semibold bg-[#007aff] hover:bg-blue-600 text-white transition-all disabled:opacity-40 disabled:cursor-not-allowed shadow-xs active:scale-95"
        @click="handleSave"
      >
        {{ t("saveProfile") }}
      </button>
    </div>

  </div>
</template>

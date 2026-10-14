<script setup lang="ts">
import { ref } from "vue";
import type { RuntimeInfo } from "../types";
import VersionItem from "./VersionItem.vue";

const props = defineProps<{
  runtime: RuntimeInfo;
  switching: boolean;
}>();

const emit = defineEmits<{
  (e: "switch", payload: { runtime: string; version: string }): void;
}>();

const isExpanded = ref<boolean>(true);

const toggleExpanded = () => {
  isExpanded.value = !isExpanded.value;
};

const handleSwitch = (version: string) => {
  emit("switch", { runtime: props.runtime.name, version });
};
</script>

<template>
  <div
    class="border border-neutral-200/80 dark:border-neutral-800/80 rounded-xl bg-white/70 dark:bg-neutral-900/60 backdrop-blur-md shadow-sm transition-all duration-200 overflow-hidden"
  >
    <!-- Section Header -->
    <div
      class="flex items-center justify-between p-3.5 cursor-pointer select-none hover:bg-black/[0.02] dark:hover:bg-white/[0.02] transition-colors"
      @click="toggleExpanded"
    >
      <div class="flex items-center gap-2.5 min-w-0">
        <span class="text-xl leading-none flex-shrink-0" :title="runtime.display_name">
          {{ runtime.icon || "📦" }}
        </span>
        <div class="min-w-0">
          <div class="flex items-center gap-1.5">
            <span class="font-semibold text-xs text-neutral-800 dark:text-neutral-100">
              {{ runtime.display_name }}
            </span>
            <span
              class="text-[10px] px-1.5 py-0.5 rounded-full bg-neutral-100 dark:bg-neutral-800 text-neutral-500 dark:text-neutral-400 font-medium"
            >
              {{ runtime.manager }}
            </span>
          </div>
          <div class="text-[11px] text-neutral-500 dark:text-neutral-400 truncate mt-0.5 font-mono">
            <span v-if="runtime.current_version" class="text-blue-600 dark:text-blue-400 font-medium">
              {{ runtime.current_version }}
            </span>
            <span v-else class="text-neutral-400 dark:text-neutral-500 italic">
              {{ runtime.is_available ? "No active version" : "Not detected" }}
            </span>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2 flex-shrink-0">
        <span
          v-if="runtime.is_available"
          class="text-[11px] font-medium text-neutral-400 dark:text-neutral-500"
        >
          {{ runtime.versions.length }} {{ runtime.versions.length === 1 ? 'version' : 'versions' }}
        </span>
        <span
          v-else
          class="text-[10px] px-2 py-0.5 rounded-full bg-amber-100 dark:bg-amber-950/60 text-amber-700 dark:text-amber-400 font-medium"
        >
          Unavailable
        </span>

        <!-- Chevron -->
        <svg
          class="w-4 h-4 text-neutral-400 transition-transform duration-200"
          :class="isExpanded ? 'rotate-180' : 'rotate-0'"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
        </svg>
      </div>
    </div>

    <!-- Collapsible Versions List -->
    <div
      v-show="isExpanded"
      class="border-t border-neutral-100 dark:border-neutral-800/60 p-2 space-y-1 bg-neutral-50/50 dark:bg-black/20"
    >
      <div v-if="!runtime.is_available" class="px-3 py-3 text-center text-xs text-neutral-500 dark:text-neutral-400">
        {{ runtime.manager }} is not installed or not in PATH.
      </div>
      <div
        v-else-if="runtime.versions.length === 0"
        class="px-3 py-3 text-center text-xs text-neutral-500 dark:text-neutral-400"
      >
        No versions installed via {{ runtime.manager }}.
      </div>
      <template v-else>
        <VersionItem
          v-for="ver in runtime.versions"
          :key="ver.version"
          :version="ver"
          :switching="switching"
          @switch="handleSwitch"
        />
      </template>
    </div>
  </div>
</template>

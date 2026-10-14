<script setup lang="ts">
import type { RuntimeVersion } from "../types";

defineProps<{
  version: RuntimeVersion;
  switching: boolean;
}>();

const emit = defineEmits<{
  (e: "switch", version: string): void;
}>();
</script>

<template>
  <div
    class="flex items-center justify-between px-3 py-2 rounded-lg text-xs transition-colors duration-150"
    :class="
      version.is_current
        ? 'bg-blue-50/70 dark:bg-blue-950/40 text-blue-900 dark:text-blue-200'
        : 'hover:bg-black/[0.03] dark:hover:bg-white/[0.04] text-neutral-700 dark:text-neutral-300'
    "
  >
    <div class="flex items-center gap-2 min-w-0">
      <span
        class="w-1.5 h-1.5 rounded-full flex-shrink-0"
        :class="version.is_current ? 'bg-blue-500 animate-pulse' : 'bg-transparent'"
      />
      <span class="font-mono text-xs truncate" :title="version.display_name || version.version">
        {{ version.display_name || version.version }}
      </span>
    </div>

    <div class="flex items-center gap-1.5 flex-shrink-0">
      <span
        v-if="version.is_current"
        class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-semibold bg-blue-100 dark:bg-blue-900/60 text-blue-700 dark:text-blue-300 border border-blue-200 dark:border-blue-800"
      >
        <svg class="w-2.5 h-2.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
          <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
        </svg>
        Current
      </span>

      <button
        v-else
        type="button"
        :disabled="switching"
        class="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-[11px] font-medium transition-all duration-150 disabled:opacity-50 disabled:cursor-not-allowed bg-neutral-200/60 hover:bg-blue-500 hover:text-white dark:bg-neutral-700/60 dark:hover:bg-blue-600 dark:hover:text-white active:scale-95"
        @click="emit('switch', version.version)"
      >
        <svg
          v-if="switching"
          class="w-2.5 h-2.5 animate-spin"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="3"
        >
          <circle cx="12" cy="12" r="10" stroke-opacity="0.25" />
          <path d="M12 2a10 10 0 0 1 10 10" />
        </svg>
        <span>{{ switching ? "Switching..." : "Switch" }}</span>
      </button>
    </div>
  </div>
</template>

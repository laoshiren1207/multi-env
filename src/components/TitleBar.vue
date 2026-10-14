<script setup lang="ts">
defineProps<{
  loading: boolean;
}>();

const emit = defineEmits<{
  (e: "refresh"): void;
}>();
</script>

<template>
  <header
    data-tauri-drag-region
    class="h-10 px-3 flex items-center justify-between select-none border-b border-neutral-200/60 dark:border-neutral-800/80 bg-white/80 dark:bg-neutral-900/80 backdrop-blur-md flex-shrink-0"
  >
    <!-- App icon + title (draggable area) -->
    <div data-tauri-drag-region class="flex items-center gap-2 pointer-events-none">
      <div
        class="w-5 h-5 rounded-md bg-gradient-to-tr from-blue-600 to-indigo-500 flex items-center justify-center text-white shadow-sm"
      >
        <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
        </svg>
      </div>
      <h1 class="text-xs font-bold text-neutral-800 dark:text-neutral-100 tracking-tight">
        Multi-Env Switcher
      </h1>
    </div>

    <!-- Actions (non-draggable) -->
    <div class="flex items-center gap-1">
      <button
        type="button"
        :disabled="loading"
        class="w-6 h-6 flex items-center justify-center rounded-md text-neutral-500 hover:text-neutral-800 dark:text-neutral-400 dark:hover:text-neutral-100 hover:bg-neutral-200/50 dark:hover:bg-neutral-800 transition-colors disabled:opacity-50"
        title="Refresh environments"
        @click="emit('refresh')"
      >
        <svg
          class="w-3.5 h-3.5"
          :class="loading ? 'animate-spin' : ''"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
        </svg>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { watch } from "vue";

const props = withDefaults(
  defineProps<{
    show: boolean;
    success?: boolean;
    message: string;
    duration?: number;
  }>(),
  {
    success: true,
    duration: 3000,
  }
);

const emit = defineEmits<{
  (e: "close"): void;
}>();

let timer: ReturnType<typeof setTimeout> | null = null;

watch(
  () => props.show,
  (newVal) => {
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
    if (newVal) {
      timer = setTimeout(() => {
        emit("close");
      }, props.duration);
    }
  },
  { immediate: true }
);
</script>

<template>
  <Transition
    enter-active-class="transition duration-200 ease-out"
    enter-from-class="transform translate-y-2 opacity-0"
    enter-to-class="transform translate-y-0 opacity-100"
    leave-active-class="transition duration-150 ease-in"
    leave-from-class="transform translate-y-0 opacity-100"
    leave-to-class="transform translate-y-2 opacity-0"
  >
    <div
      v-if="show"
      class="fixed bottom-3 left-3 right-3 z-50 flex items-center justify-between gap-2 px-3 py-2 rounded-xl text-xs font-medium shadow-lg backdrop-blur-md border"
      :class="
        success
          ? 'bg-emerald-50/95 dark:bg-emerald-950/90 text-emerald-800 dark:text-emerald-200 border-emerald-200 dark:border-emerald-800 shadow-emerald-500/10'
          : 'bg-rose-50/95 dark:bg-rose-950/90 text-rose-800 dark:text-rose-200 border-rose-200 dark:border-rose-800 shadow-rose-500/10'
      "
    >
      <div class="flex items-center gap-2 min-w-0">
        <span
          class="flex-shrink-0 w-4 h-4 rounded-full flex items-center justify-center text-[10px]"
          :class="
            success
              ? 'bg-emerald-200 dark:bg-emerald-800 text-emerald-900 dark:text-emerald-100'
              : 'bg-rose-200 dark:bg-rose-800 text-rose-900 dark:text-rose-100'
          "
        >
          <svg
            v-if="success"
            class="w-2.5 h-2.5 fill-none stroke-current stroke-2"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
          </svg>
          <svg
            v-else
            class="w-2.5 h-2.5 fill-none stroke-current stroke-2"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </span>
        <span class="truncate">{{ message }}</span>
      </div>

      <button
        type="button"
        class="flex-shrink-0 p-1 rounded-md text-current opacity-70 hover:opacity-100 hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
        @click="emit('close')"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  </Transition>
</template>

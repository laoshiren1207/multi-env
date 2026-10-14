<script setup lang="ts">
import { ref } from "vue";
import type { Profile, RuntimeInfo } from "../types";
import ProfileForm from "./ProfileForm.vue";

defineProps<{
  profiles: Profile[];
  runtimes: RuntimeInfo[];
  loading: boolean;
}>();

const emit = defineEmits<{
  (e: "apply", id: string): void;
  (e: "delete", id: string): void;
  (e: "save", profile: Profile): void;
}>();

const isCreating = ref<boolean>(false);
const editingProfile = ref<Profile | null>(null);
const deletingId = ref<string | null>(null);
const applyingId = ref<string | null>(null);

const startCreate = () => {
  editingProfile.value = null;
  isCreating.value = true;
};

const cancelForm = () => {
  isCreating.value = false;
  editingProfile.value = null;
};

const handleSave = (profile: Profile) => {
  emit("save", profile);
  isCreating.value = false;
  editingProfile.value = null;
};

const handleApply = (id: string) => {
  applyingId.value = id;
  emit("apply", id);
  // Reset applyingId after a delay or parent handles it
  setTimeout(() => {
    if (applyingId.value === id) applyingId.value = null;
  }, 1000);
};

const confirmDelete = (id: string) => {
  deletingId.value = id;
};

const executeDelete = (id: string) => {
  emit("delete", id);
  deletingId.value = null;
};

const cancelDelete = () => {
  deletingId.value = null;
};
</script>

<template>
  <div class="space-y-3">
    <!-- Profile Form (Modal/Inline) -->
    <div v-if="isCreating || editingProfile">
      <ProfileForm
        :initial-profile="editingProfile"
        :runtimes="runtimes"
        @save="handleSave"
        @cancel="cancelForm"
      />
    </div>

    <!-- Header Actions (when not creating) -->
    <div v-else class="flex items-center justify-between">
      <span class="text-xs font-semibold text-neutral-600 dark:text-neutral-400">
        Saved Profiles ({{ profiles.length }})
      </span>
      <button
        type="button"
        class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold bg-blue-600 hover:bg-blue-500 text-white shadow-sm transition-all active:scale-95"
        @click="startCreate"
      >
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
        New Profile
      </button>
    </div>

    <!-- Empty State -->
    <div
      v-if="!isCreating && !editingProfile && profiles.length === 0"
      class="text-center py-10 px-4 rounded-xl border border-dashed border-neutral-200 dark:border-neutral-800 bg-neutral-50/50 dark:bg-neutral-900/40"
    >
      <div class="text-3xl mb-2">🔖</div>
      <h4 class="text-xs font-semibold text-neutral-800 dark:text-neutral-200">No profiles yet</h4>
      <p class="text-[11px] text-neutral-500 dark:text-neutral-400 mt-1 max-w-xs mx-auto">
        Create profiles to switch Java, Node.js, and Python versions all at once with a single click.
      </p>
      <button
        type="button"
        class="mt-3 inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-xs font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-950/50 transition-colors"
        @click="startCreate"
      >
        + Create your first profile
      </button>
    </div>

    <!-- Profiles List -->
    <div v-if="!isCreating && !editingProfile" class="space-y-2.5">
      <div
        v-for="profile in profiles"
        :key="profile.id"
        class="p-3.5 rounded-xl border border-neutral-200/80 dark:border-neutral-800/80 bg-white/70 dark:bg-neutral-900/60 backdrop-blur-md shadow-sm transition-all hover:shadow-md"
      >
        <div class="flex items-start justify-between gap-2 mb-2.5">
          <div>
            <h4 class="text-xs font-bold text-neutral-800 dark:text-neutral-100">
              {{ profile.name }}
            </h4>
          </div>

          <!-- Apply & Delete Buttons -->
          <div class="flex items-center gap-1.5 flex-shrink-0">
            <!-- Normal Delete vs Confirm Delete -->
            <template v-if="deletingId === profile.id">
              <span class="text-[10px] text-rose-500 font-medium">Delete?</span>
              <button
                type="button"
                class="px-2 py-0.5 rounded text-[10px] font-semibold bg-rose-600 text-white hover:bg-rose-700 transition-colors"
                @click="executeDelete(profile.id)"
              >
                Yes
              </button>
              <button
                type="button"
                class="px-2 py-0.5 rounded text-[10px] font-medium bg-neutral-200 dark:bg-neutral-700 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-300 dark:hover:bg-neutral-600 transition-colors"
                @click="cancelDelete"
              >
                No
              </button>
            </template>
            <template v-else>
              <button
                type="button"
                class="p-1 rounded-md text-neutral-400 hover:text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950/40 transition-colors"
                title="Delete profile"
                @click="confirmDelete(profile.id)"
              >
                <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
              <button
                type="button"
                :disabled="loading || applyingId === profile.id"
                class="inline-flex items-center gap-1 px-2.5 py-1 rounded-md text-[11px] font-semibold bg-blue-500 hover:bg-blue-600 text-white disabled:opacity-50 transition-all active:scale-95 shadow-sm"
                @click="handleApply(profile.id)"
              >
                <svg
                  v-if="applyingId === profile.id"
                  class="w-3 h-3 animate-spin"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="3"
                >
                  <circle cx="12" cy="12" r="10" stroke-opacity="0.25" />
                  <path d="M12 2a10 10 0 0 1 10 10" />
                </svg>
                <svg
                  v-else
                  class="w-3 h-3"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2.5"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                </svg>
                <span>{{ applyingId === profile.id ? "Applying..." : "Apply" }}</span>
              </button>
            </template>
          </div>
        </div>

        <!-- Configured versions tags -->
        <div class="grid grid-cols-3 gap-1.5 text-[10px]">
          <div
            class="px-2 py-1 rounded-md bg-neutral-100/70 dark:bg-neutral-800/60 border border-neutral-200/50 dark:border-neutral-700/50 flex flex-col"
          >
            <span class="text-neutral-400 dark:text-neutral-500 flex items-center gap-1">☕ Java</span>
            <span class="font-mono font-medium truncate text-neutral-700 dark:text-neutral-200 mt-0.5">
              {{ profile.java_version || "—" }}
            </span>
          </div>

          <div
            class="px-2 py-1 rounded-md bg-neutral-100/70 dark:bg-neutral-800/60 border border-neutral-200/50 dark:border-neutral-700/50 flex flex-col"
          >
            <span class="text-neutral-400 dark:text-neutral-500 flex items-center gap-1">⬢ Node</span>
            <span class="font-mono font-medium truncate text-neutral-700 dark:text-neutral-200 mt-0.5">
              {{ profile.node_version || "—" }}
            </span>
          </div>

          <div
            class="px-2 py-1 rounded-md bg-neutral-100/70 dark:bg-neutral-800/60 border border-neutral-200/50 dark:border-neutral-700/50 flex flex-col"
          >
            <span class="text-neutral-400 dark:text-neutral-500 flex items-center gap-1">🐍 Python</span>
            <span class="font-mono font-medium truncate text-neutral-700 dark:text-neutral-200 mt-0.5">
              {{ profile.python_version || "—" }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

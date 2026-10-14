import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { RuntimeInfo, Profile, SwitchResult } from "../types";

export function useEnvManager() {
  const runtimes = ref<RuntimeInfo[]>([]);
  const profiles = ref<Profile[]>([]);
  const loading = ref<boolean>(false);
  const switching = ref<Record<string, boolean>>({});
  const error = ref<string | null>(null);

  const fetchRuntimes = async (): Promise<RuntimeInfo[]> => {
    loading.value = true;
    error.value = null;
    try {
      const data = await invoke<RuntimeInfo[]>("get_all_runtimes");
      runtimes.value = data;
      return data;
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error("Failed to fetch runtimes:", err);
      return [];
    } finally {
      loading.value = false;
    }
  };

  const switchVersion = async (
    runtime: string,
    version: string
  ): Promise<SwitchResult> => {
    switching.value[runtime] = true;
    error.value = null;
    try {
      const result = await invoke<SwitchResult>("switch_version", {
        runtime,
        version,
      });
      // Refresh runtimes to update active versions
      await fetchRuntimes();
      return result;
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error(`Failed to switch version for ${runtime}:`, err);
      return {
        success: false,
        message: message,
        runtime,
        from_version: null,
        to_version: version,
      };
    } finally {
      switching.value[runtime] = false;
    }
  };

  const installing = ref<Record<string, boolean>>({});

  const installVersion = async (
    runtime: string,
    version: string
  ): Promise<{ success: boolean; message: string }> => {
    installing.value[runtime] = true;
    error.value = null;
    try {
      const msg = await invoke<string>("install_version", {
        runtime,
        version,
      });
      await fetchRuntimes();
      return { success: true, message: msg };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error(`Failed to install version ${version} for ${runtime}:`, err);
      return { success: false, message };
    } finally {
      installing.value[runtime] = false;
    }
  };

  const uninstalling = ref<Record<string, boolean>>({});

  const uninstallVersion = async (
    runtime: string,
    version: string
  ): Promise<{ success: boolean; message: string }> => {
    const key = `${runtime}:${version}`;
    uninstalling.value[key] = true;
    error.value = null;
    try {
      const msg = await invoke<string>("uninstall_version", {
        runtime,
        version,
      });
      await fetchRuntimes();
      return { success: true, message: msg };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error(`Failed to uninstall version ${version} for ${runtime}:`, err);
      return { success: false, message };
    } finally {
      uninstalling.value[key] = false;
    }
  };

  const fetchProfiles = async (): Promise<Profile[]> => {
    error.value = null;
    try {
      const data = await invoke<Profile[]>("get_profiles");
      profiles.value = data;
      return data;
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error("Failed to fetch profiles:", err);
      return [];
    }
  };

  const saveProfile = async (profile: Profile): Promise<boolean> => {
    error.value = null;
    try {
      await invoke("save_profile", { profile });
      await fetchProfiles();
      return true;
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error("Failed to save profile:", err);
      return false;
    }
  };

  const deleteProfile = async (id: string): Promise<boolean> => {
    error.value = null;
    try {
      await invoke("delete_profile", { id });
      await fetchProfiles();
      return true;
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error(`Failed to delete profile ${id}:`, err);
      return false;
    }
  };

  const applyProfile = async (id: string): Promise<boolean> => {
    loading.value = true;
    error.value = null;
    try {
      await invoke("apply_profile", { id });
      await fetchRuntimes();
      return true;
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error(`Failed to apply profile ${id}:`, err);
      return false;
    } finally {
      loading.value = false;
    }
  };

  return {
    runtimes,
    profiles,
    loading,
    switching,
    installing,
    uninstalling,
    error,
    fetchRuntimes,
    switchVersion,
    installVersion,
    uninstallVersion,
    fetchProfiles,
    saveProfile,
    deleteProfile,
    applyProfile,
  };
}

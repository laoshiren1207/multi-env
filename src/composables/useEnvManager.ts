import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { RuntimeInfo, Profile, SwitchResult, NodeMirrors } from "../types";

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

  const installingManager = ref<Record<string, boolean>>({});

  const installManager = async (
    manager: string
  ): Promise<{ success: boolean; message: string }> => {
    installingManager.value[manager] = true;
    error.value = null;
    try {
      const msg = await invoke<string>("install_manager", { manager });
      await fetchRuntimes();
      return { success: true, message: msg };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      console.error(`Failed to install manager ${manager}:`, err);
      return { success: false, message };
    } finally {
      installingManager.value[manager] = false;
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

  const nodeMirrors = ref<NodeMirrors>({
    npm_registry: "",
    nvm_node_mirror: "",
  });
  const settingMirror = ref<boolean>(false);

  const fetchNodeMirrors = async (): Promise<NodeMirrors> => {
    try {
      const data = await invoke<NodeMirrors>("get_node_mirrors");
      nodeMirrors.value = data;
      return data;
    } catch (err: unknown) {
      console.error("Failed to fetch node mirrors:", err);
      return nodeMirrors.value;
    }
  };

  const setNpmRegistry = async (
    registry: string
  ): Promise<{ success: boolean; message: string }> => {
    settingMirror.value = true;
    error.value = null;
    try {
      const msg = await invoke<string>("set_npm_registry", { registry });
      await fetchNodeMirrors();
      return { success: true, message: msg };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      return { success: false, message };
    } finally {
      settingMirror.value = false;
    }
  };

  const setNvmMirror = async (
    mirror: string
  ): Promise<{ success: boolean; message: string }> => {
    settingMirror.value = true;
    error.value = null;
    try {
      const msg = await invoke<string>("set_nvm_mirror", { mirror });
      await fetchNodeMirrors();
      return { success: true, message: msg };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      return { success: false, message };
    } finally {
      settingMirror.value = false;
    }
  };

  const setBothNodeMirrors = async (
    registry: string,
    mirror: string
  ): Promise<{ success: boolean; message: string }> => {
    settingMirror.value = true;
    error.value = null;
    try {
      await invoke<string>("set_npm_registry", { registry });
      await invoke<string>("set_nvm_mirror", { mirror });
      await fetchNodeMirrors();
      return { success: true, message: "OK" };
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      error.value = message;
      return { success: false, message };
    } finally {
      settingMirror.value = false;
    }
  };

  return {
    runtimes,
    profiles,
    nodeMirrors,
    settingMirror,
    loading,
    switching,
    installing,
    uninstalling,
    installingManager,
    error,
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
  };
}

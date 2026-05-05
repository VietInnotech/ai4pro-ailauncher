import { writable } from "svelte/store";
import { getAppSettings, updateAppSettings } from "$lib/api/settings";
import type { AppSettingsDto, UpdateAppSettingsDto } from "$lib/types/settings";

function errorMessage(error: unknown): string {
  if (typeof error === "string") {
    try {
      const parsed = JSON.parse(error) as { message?: string };
      return parsed.message ?? error;
    } catch {
      return error;
    }
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "Unexpected error";
}

function createAppSettingsStore() {
  const { subscribe, update, set } = writable<{
    value?: AppSettingsDto;
    loading: boolean;
    error?: string;
  }>({ loading: false });

  return {
    subscribe,
    async refresh() {
      update((state) => ({ ...state, loading: true, error: undefined }));
      try {
        const value = await getAppSettings();
        set({ value, loading: false });
      } catch (error) {
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage(error)
        }));
      }
    },
    async save(input: UpdateAppSettingsDto) {
      update((state) => ({ ...state, loading: true, error: undefined }));
      try {
        const value = await updateAppSettings(input);
        set({ value, loading: false });
      } catch (error) {
        update((state) => ({
          ...state,
          loading: false,
          error: errorMessage(error)
        }));
      }
    }
  };
}

export const appSettingsStore = createAppSettingsStore();

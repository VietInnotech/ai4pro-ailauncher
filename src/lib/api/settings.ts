import { invoke } from "@tauri-apps/api/core";
import type { AppSettingsDto, UpdateAppSettingsDto } from "$lib/types/settings";

export function getAppSettings(): Promise<AppSettingsDto> {
  return invoke("get_app_settings");
}

export function updateAppSettings(input: UpdateAppSettingsDto): Promise<AppSettingsDto> {
  return invoke("update_app_settings", { input });
}

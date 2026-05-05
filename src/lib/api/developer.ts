import { invoke } from "@tauri-apps/api/core";
import type {
  DeveloperEngineProfileDto,
  DeveloperLogType,
  DeveloperModelPackageDto,
  DiagnosticsBundleDto,
  UpdateEngineProfileDto,
  ValidationResultDto
} from "$lib/types/developer";

export function enableDeveloperModeForSession(): Promise<void> {
  return invoke("enable_developer_mode_for_session");
}

export function disableDeveloperModeForSession(): Promise<void> {
  return invoke("disable_developer_mode_for_session");
}

export function devListEngineProfiles(): Promise<DeveloperEngineProfileDto[]> {
  return invoke("dev_list_engine_profiles");
}

export function devGetEngineProfile(id: string): Promise<DeveloperEngineProfileDto> {
  return invoke("dev_get_engine_profile", { id });
}

export function devUpdateEngineProfile(
  id: string,
  input: UpdateEngineProfileDto
): Promise<DeveloperEngineProfileDto> {
  return invoke("dev_update_engine_profile", { id, input });
}

export function devValidateEngineProfile(id: string): Promise<ValidationResultDto> {
  return invoke("dev_validate_engine_profile", { id });
}

export function devStartEngineProfile(id: string): Promise<DeveloperEngineProfileDto> {
  return invoke("dev_start_engine_profile", { id });
}

export function devStopEngineProfile(id: string): Promise<DeveloperEngineProfileDto> {
  return invoke("dev_stop_engine_profile", { id });
}

export function devRestartEngineProfile(id: string): Promise<DeveloperEngineProfileDto> {
  return invoke("dev_restart_engine_profile", { id });
}

export function devReadEngineLog(
  id: string,
  logType: DeveloperLogType,
  tailLines?: number
): Promise<string> {
  return invoke("dev_read_engine_log", { id, logType, tailLines });
}

export function devGetDiagnosticsBundle(): Promise<DiagnosticsBundleDto> {
  return invoke("dev_get_diagnostics_bundle");
}

export function devOpenLogsFolder(id: string): Promise<void> {
  return invoke("dev_open_logs_folder", { id });
}

export function devListModelPackages(): Promise<DeveloperModelPackageDto[]> {
  return invoke("dev_list_model_packages");
}

export function devValidateModelPackage(id: string): Promise<ValidationResultDto> {
  return invoke("dev_validate_model_package", { id });
}

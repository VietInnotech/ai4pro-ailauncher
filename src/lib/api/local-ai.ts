import { invoke } from "@tauri-apps/api/core";
import type { SimpleLocalAiStatusDto } from "$lib/types/local-ai";

export function getSimpleLocalAiStatus(): Promise<SimpleLocalAiStatusDto> {
  return invoke("get_simple_local_ai_status");
}

export function checkSimpleModelStatus(id: string): Promise<SimpleLocalAiStatusDto> {
  return invoke("check_simple_model_status", { id });
}

export function startLocalAi(): Promise<SimpleLocalAiStatusDto> {
  return invoke("start_local_ai");
}

export function stopLocalAi(): Promise<SimpleLocalAiStatusDto> {
  return invoke("stop_local_ai");
}

export function restartLocalAi(): Promise<SimpleLocalAiStatusDto> {
  return invoke("restart_local_ai");
}

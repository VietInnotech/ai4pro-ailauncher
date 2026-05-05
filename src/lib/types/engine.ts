export type EngineKind = "llama_cpp" | "sherpa_onnx";

export type EngineStatus =
  | "stopped"
  | "starting"
  | "running"
  | "unhealthy"
  | "stopping"
  | "crashed"
  | "missing_binary"
  | "missing_model"
  | "invalid_config"
  | "port_conflict";

export type BinaryMode = "bundled" | "custom";

export type HealthCheckType = "http" | "tcp" | "process";

export type EngineRuntimeState = {
  engineId: string;
  status: EngineStatus;
  pid?: number;
  healthUrl?: string;
  startedAt?: string;
  stoppedAt?: string;
  lastError?: string;
  lastExitCode?: number;
  updatedAt: string;
};

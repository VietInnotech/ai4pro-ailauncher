import type { BinaryMode, EngineKind, EngineRuntimeState, EngineStatus } from "./engine";

export type DeveloperLogType = "stdout" | "stderr";

export type DeveloperEngineProfileDto = {
  id: string;
  kind: EngineKind;
  name: string;
  enabled: boolean;
  binaryMode: BinaryMode;
  binaryName: string;
  binaryPath?: string;
  resolvedBinaryPath?: string;
  modelPackageId?: string;
  resolvedModelPath?: string;
  resolvedModelDir?: string;
  resolvedTokensPath?: string;
  host: string;
  port: number;
  healthUrl?: string;
  runtime: Record<string, unknown>;
  extraArgs: string[];
  generatedArgs?: string[];
  status: EngineStatus | string;
  pid?: number;
  lastError?: string;
  lastExitCode?: number;
  autoStart?: boolean;
};

export type UpdateEngineProfileDto = {
  enabled: boolean;
  binaryMode: BinaryMode;
  binaryPath?: string | null;
  host: string;
  port: number;
  runtime: Record<string, unknown>;
  extraArgs: string[];
  autoStart: boolean;
};

export type ValidationIssueDto = {
  severity: "error" | "warning" | "info" | string;
  code: string;
  message: string;
};

export type ValidationResultDto = {
  engineId: string;
  valid: boolean;
  issues: ValidationIssueDto[];
  generatedArgs?: string[];
};

export type DeveloperModelPackageDto = {
  id: string;
  kind: EngineKind;
  displayName: string;
  internalName: string;
  relativePath: string;
  resolvedPath?: string;
  installed: boolean;
  verified: boolean;
  lastVerifiedAt?: string;
  requiredFiles: string[];
  manifest: Record<string, unknown>;
};

export type DiagnosticsBundleDto = {
  appVersion: string;
  os: string;
  arch: string;
  appDataRoot: string;
  sqlitePath: string;
  logsRoot: string;
  machineConfigured: boolean;
  appSettings?: Record<string, unknown>;
  appPaths?: Record<string, unknown>;
  engineProfiles: DeveloperEngineProfileDto[];
  modelPackages: DeveloperModelPackageDto[];
  validation: ValidationResultDto[];
  recentCrashes: Array<{
    engineId: string;
    lastError?: string;
    lastExitCode?: number;
    updatedAt: string;
  }>;
  runtimeState?: EngineRuntimeState[];
  recentLogs?: Array<{
    name: string;
    path: string;
    content: string;
  }>;
};

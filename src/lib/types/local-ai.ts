export type SimpleLocalAiStatus =
  | "not_running"
  | "starting"
  | "ready"
  | "stopping"
  | "needs_attention";

export type SimpleModelStatus = "unchecked" | "ready" | "needs_attention";

export type SimpleModelSummaryDto = {
  id: string;
  displayName: string;
  status: SimpleModelStatus;
  lastCheckedAt?: string;
};

export type SimpleLocalAiStatusDto = {
  status: SimpleLocalAiStatus;
  title: string;
  message: string;
  canStart: boolean;
  canStop: boolean;
  canRestart: boolean;
  modelSummaries: SimpleModelSummaryDto[];
};

export const defaultSimpleStatus: SimpleLocalAiStatusDto = {
  status: "not_running",
  title: "Local AI đã sẵn sàng",
  message: "Dịch vụ Local AI luôn sẵn sàng khi bạn cần.",
  canStart: true,
  canStop: false,
  canRestart: false,
  modelSummaries: []
};

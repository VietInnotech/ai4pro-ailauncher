export type SimpleLocalAiStatus =
  | "not_running"
  | "starting"
  | "ready"
  | "stopping"
  | "needs_attention";

export type SimpleLocalAiStatusDto = {
  status: SimpleLocalAiStatus;
  title: string;
  message: string;
  canStart: boolean;
  canStop: boolean;
  canRestart: boolean;
};

export const defaultSimpleStatus: SimpleLocalAiStatusDto = {
  status: "not_running",
  title: "Local AI đã sẵn sàng",
  message: "Dịch vụ Local AI luôn sẵn sàng khi bạn cần.",
  canStart: true,
  canStop: false,
  canRestart: false
};

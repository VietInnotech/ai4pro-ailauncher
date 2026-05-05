import type { SimpleLocalAiStatus } from "$lib/types/local-ai";

export const UNAVAILABLE_LABEL = "Không có";

const simpleStatusLabels: Record<SimpleLocalAiStatus, string> = {
  not_running: "chưa chạy",
  starting: "đang khởi động",
  ready: "sẵn sàng",
  stopping: "đang dừng",
  needs_attention: "cần chú ý"
};

const engineStatusLabels: Record<string, string> = {
  stopped: "Đã dừng",
  starting: "Đang khởi động",
  running: "Đang hoạt động",
  unhealthy: "Không ổn định",
  stopping: "Đang dừng",
  crashed: "Đã gặp sự cố",
  missing_binary: "Thiếu tệp nhị phân",
  missing_model: "Thiếu mô hình",
  invalid_config: "Cấu hình không hợp lệ",
  port_conflict: "Xung đột cổng"
};

const binaryModeLabels: Record<string, string> = {
  bundled: "Đóng gói sẵn",
  custom: "Tùy chỉnh"
};

const logTypeLabels: Record<string, string> = {
  stdout: "Đầu ra chuẩn",
  stderr: "Lỗi chuẩn"
};

export function formatSimpleStatusLabel(status: SimpleLocalAiStatus): string {
  return simpleStatusLabels[status];
}

export function formatEngineStatusLabel(status: string): string {
  return engineStatusLabels[status] ?? status;
}

export function formatBinaryModeLabel(mode?: string | null): string {
  if (!mode) return UNAVAILABLE_LABEL;
  return binaryModeLabels[mode] ?? mode;
}

export function formatLogTypeLabel(type: string): string {
  return logTypeLabels[type] ?? type;
}

export function formatYesNo(value: boolean): string {
  return value ? "Có" : "Không";
}

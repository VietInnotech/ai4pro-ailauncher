<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { formatEngineStatusLabel, formatYesNo } from "$lib/labels";
  import type { DiagnosticsBundleDto } from "$lib/types/developer";

  export let bundle: DiagnosticsBundleDto | null = null;

  const dispatch = createEventDispatcher<{ export: void }>();
</script>

<section class="panel p-6">
  <div class="mb-6 flex items-center justify-between gap-4">
    <div>
      <h2 class="text-lg font-semibold text-[#1b2430]">Chẩn đoán</h2>
      <p class="mt-1 text-sm text-[#5e6a79]">Ảnh chụp nhanh về môi trường và kết quả kiểm tra dành cho nhà phát triển.</p>
    </div>
    <button class="action-button-secondary" on:click={() => dispatch("export")}>Xuất gói chẩn đoán</button>
  </div>

  {#if bundle}
    <div class="space-y-6">
      <dl class="grid gap-4 sm:grid-cols-2">
        <div><dt class="field-label">Phiên bản ứng dụng</dt><dd>{bundle.appVersion}</dd></div>
        <div><dt class="field-label">Hệ điều hành / Kiến trúc</dt><dd>{bundle.os} / {bundle.arch}</dd></div>
        <div><dt class="field-label">Thư mục dữ liệu ứng dụng gốc</dt><dd class="break-all text-sm text-[#5e6a79]">{bundle.appDataRoot}</dd></div>
        <div><dt class="field-label">Đường dẫn SQLite</dt><dd class="break-all text-sm text-[#5e6a79]">{bundle.sqlitePath}</dd></div>
        <div><dt class="field-label">Thư mục nhật ký gốc</dt><dd class="break-all text-sm text-[#5e6a79]">{bundle.logsRoot}</dd></div>
        <div><dt class="field-label">Máy đã được cấu hình</dt><dd>{formatYesNo(bundle.machineConfigured)}</dd></div>
      </dl>

      <div>
        <h3 class="text-base font-semibold text-[#1b2430]">Kết quả kiểm tra</h3>
        <ul class="mt-3 space-y-2 text-sm text-[#5e6a79]">
          {#each bundle.validation as result}
            <li class="rounded-md border px-3 py-2" style="border-color: #d5dce3;">
              <span class="font-medium text-[#1b2430]">{result.engineId}</span>
              : {result.valid ? "hợp lệ" : `${result.issues.length} vấn đề`}
            </li>
          {/each}
        </ul>
      </div>

      {#if bundle.runtimeState?.length}
        <div>
          <h3 class="text-base font-semibold text-[#1b2430]">Trạng thái thời gian chạy</h3>
          <ul class="mt-3 space-y-2 text-sm text-[#5e6a79]">
            {#each bundle.runtimeState as entry}
              <li class="rounded-md border px-3 py-2" style="border-color: #d5dce3;">
                <span class="font-medium text-[#1b2430]">{entry.engineId}</span>
                : {formatEngineStatusLabel(entry.status)} {entry.pid ? `(PID ${entry.pid})` : ""}
              </li>
            {/each}
          </ul>
        </div>
      {/if}

      {#if bundle.recentLogs?.length}
        <div>
          <h3 class="text-base font-semibold text-[#1b2430]">Nhật ký gần đây</h3>
          <div class="mt-3 space-y-3">
            {#each bundle.recentLogs as log}
              <div>
                <p class="mb-2 text-sm font-medium text-[#1b2430]">{log.name}</p>
                <pre class="code-block max-h-48">{log.content}</pre>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <p class="text-sm text-[#5e6a79]">Tải chẩn đoán để xem cấu hình máy, kết quả kiểm tra, nhật ký gần đây và trạng thái thời gian chạy.</p>
  {/if}
</section>

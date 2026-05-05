<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { formatEngineStatusLabel, UNAVAILABLE_LABEL } from "$lib/labels";
  import type { DeveloperEngineProfileDto } from "$lib/types/developer";

  export let engine: DeveloperEngineProfileDto | null = null;
  export let busy = false;

  const dispatch = createEventDispatcher<{
    refresh: string;
    validate: string;
    start: string;
    stop: string;
    restart: string;
  }>();
</script>

<section class="panel p-6">
  <div class="mb-6 flex items-start justify-between gap-4">
    <div>
      <h2 class="text-lg font-semibold text-[#1b2430]">Chi tiết động cơ</h2>
      <p class="mt-1 text-sm text-[#5e6a79]">Các giá trị đã phân giải chỉ hiển thị trong Chế độ nhà phát triển.</p>
    </div>
    {#if engine}
      <div class="flex flex-wrap gap-2">
        <button class="action-button-secondary" disabled={busy} on:click={() => dispatch("validate", engine.id)}>Kiểm tra</button>
        <button class="action-button-secondary" disabled={busy} on:click={() => dispatch("refresh", engine.id)}>Tải lại</button>
        <button class="action-button-primary" disabled={busy} on:click={() => dispatch("start", engine.id)}>Bắt đầu</button>
        <button class="action-button-danger" disabled={busy} on:click={() => dispatch("stop", engine.id)}>Dừng</button>
        <button class="action-button-secondary" disabled={busy} on:click={() => dispatch("restart", engine.id)}>Khởi động lại</button>
      </div>
    {/if}
  </div>

  {#if engine}
    <dl class="grid gap-4 sm:grid-cols-2">
      <div><dt class="field-label">Mã động cơ</dt><dd>{engine.id}</dd></div>
      <div><dt class="field-label">Loại</dt><dd>{engine.kind}</dd></div>
      <div><dt class="field-label">Tên tệp nhị phân</dt><dd>{engine.binaryName}</dd></div>
      <div><dt class="field-label">Đường dẫn nhị phân đã phân giải</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.resolvedBinaryPath ?? UNAVAILABLE_LABEL}</dd></div>
      <div><dt class="field-label">Đường dẫn mô hình đã phân giải</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.resolvedModelPath ?? UNAVAILABLE_LABEL}</dd></div>
      <div><dt class="field-label">Thư mục mô hình đã phân giải</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.resolvedModelDir ?? UNAVAILABLE_LABEL}</dd></div>
      <div><dt class="field-label">Đường dẫn tokens đã phân giải</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.resolvedTokensPath ?? UNAVAILABLE_LABEL}</dd></div>
      <div><dt class="field-label">URL kiểm tra tình trạng</dt><dd>{engine.healthUrl ?? UNAVAILABLE_LABEL}</dd></div>
      <div><dt class="field-label">Trạng thái</dt><dd>{formatEngineStatusLabel(engine.status)}</dd></div>
      <div><dt class="field-label">PID</dt><dd>{engine.pid ?? UNAVAILABLE_LABEL}</dd></div>
      <div><dt class="field-label">Lỗi gần nhất</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.lastError ?? UNAVAILABLE_LABEL}</dd></div>
      <div><dt class="field-label">Mã thoát gần nhất</dt><dd>{engine.lastExitCode ?? UNAVAILABLE_LABEL}</dd></div>
      <div class="sm:col-span-2"><dt class="field-label">JSON thời gian chạy</dt><dd><pre class="code-block">{JSON.stringify(engine.runtime, null, 2)}</pre></dd></div>
      <div class="sm:col-span-2"><dt class="field-label">Đối số bổ sung</dt><dd><pre class="code-block">{JSON.stringify(engine.extraArgs, null, 2)}</pre></dd></div>
      <div class="sm:col-span-2"><dt class="field-label">Đối số được tạo</dt><dd><pre class="code-block">{JSON.stringify(engine.generatedArgs ?? [], null, 2)}</pre></dd></div>
    </dl>
  {:else}
    <p class="text-sm text-[#5e6a79]">Chọn một động cơ để xem các chi tiết đã phân giải.</p>
  {/if}
</section>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { formatEngineStatusLabel, UNAVAILABLE_LABEL } from "$lib/labels";
  import type { DeveloperEngineProfileDto, UpdateEngineProfileDto } from "$lib/types/developer";

  export let engine: DeveloperEngineProfileDto | null = null;
  export let busy = false;
  export let saveError = "";

  const dispatch = createEventDispatcher<{
    refresh: string;
    validate: string;
    start: string;
    stop: string;
    restart: string;
    save: { id: string; input: UpdateEngineProfileDto };
  }>();

  let editingId = "";
  let binaryPath = "";
  let modelPath = "";
  let modelDir = "";

  $: if (engine && engine.id !== editingId) {
    editingId = engine.id;
    binaryPath = engine.binaryPath ?? "";
    modelPath = engine.modelPath ?? "";
    modelDir = engine.modelDir ?? engine.modelPath ?? "";
  }

  function nullable(value: string): string | null {
    const trimmed = value.trim();
    return trimmed.length > 0 ? trimmed : null;
  }

  function saveConfiguration() {
    if (!engine) return;
    dispatch("save", {
      id: engine.id,
      input: {
        binaryMode: nullable(binaryPath) ? "custom" : "bundled",
        binaryPath: nullable(binaryPath),
        modelPath: engine.kind === "llama_cpp" ? nullable(modelPath) : null,
        modelDir: engine.kind === "sherpa_onnx" ? nullable(modelDir) : null,
        tokensPath: null
      }
    });
  }

  async function browseBinaryPath() {
    const result = await open({
      multiple: false,
      title: "Chọn tệp nhị phân"
    });
    if (typeof result === "string") binaryPath = result;
  }

  async function browseModelPath() {
    const result = await open({
      multiple: false,
      title: "Chọn tệp mô hình GGUF",
      filters: [{ name: "GGUF", extensions: ["gguf"] }]
    });
    if (typeof result === "string") modelPath = result;
  }

  async function browseModelDir() {
    const result = await open({
      directory: true,
      multiple: false,
      title: "Chọn thư mục mô hình"
    });
    if (typeof result === "string") modelDir = result;
  }
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
    <div class="grid gap-6 xl:grid-cols-[minmax(0,1fr)_minmax(22rem,0.7fr)]">
      <form class="space-y-4" on:submit|preventDefault={saveConfiguration}>
        <div>
          <h3 class="text-base font-semibold text-[#1b2430]">Cấu hình khởi chạy</h3>
          <p class="mt-1 text-sm text-[#5e6a79]">Các đường dẫn này chỉ được chỉnh trong Chế độ nhà phát triển.</p>
        </div>

        <label class="block">
          <span class="field-label">Đường dẫn nhị phân tùy chỉnh</span>
          <div class="mt-1 flex gap-2">
            <input class="field min-w-0 flex-1 font-mono text-xs" bind:value={binaryPath} placeholder="/path/to/llama-server hoặc /path/to/python" />
            <button class="action-button-secondary shrink-0" disabled={busy} type="button" on:click={browseBinaryPath}>Duyệt</button>
          </div>
        </label>

        {#if engine.kind === "llama_cpp"}
          <label class="block">
            <span class="field-label">Tệp mô hình GGUF</span>
            <div class="mt-1 flex gap-2">
              <input class="field min-w-0 flex-1 font-mono text-xs" bind:value={modelPath} placeholder="models/llama/default/model.gguf hoặc đường dẫn tuyệt đối" />
              <button class="action-button-secondary shrink-0" disabled={busy} type="button" on:click={browseModelPath}>Duyệt</button>
            </div>
          </label>
        {:else if engine.kind === "sherpa_onnx"}
          <label class="block">
            <span class="field-label">Thư mục mô hình Sherpa</span>
            <div class="mt-1 flex gap-2">
              <input class="field min-w-0 flex-1 font-mono text-xs" bind:value={modelDir} placeholder="models/sherpa/default hoặc đường dẫn tuyệt đối" />
              <button class="action-button-secondary shrink-0" disabled={busy} type="button" on:click={browseModelDir}>Duyệt</button>
            </div>
          </label>
        {/if}

        <div class="flex flex-wrap items-center gap-3">
          <button class="action-button-primary" disabled={busy} type="submit">Lưu cấu hình</button>
          {#if saveError}
            <span class="text-sm text-[#9a3d3d]">{saveError}</span>
          {/if}
        </div>
      </form>

      <dl class="grid content-start gap-4 sm:grid-cols-2 xl:grid-cols-1">
        <div><dt class="field-label">Mã động cơ</dt><dd>{engine.id}</dd></div>
        <div><dt class="field-label">Loại</dt><dd>{engine.kind}</dd></div>
        <div><dt class="field-label">Trạng thái</dt><dd>{formatEngineStatusLabel(engine.status)}</dd></div>
        <div><dt class="field-label">PID</dt><dd>{engine.pid ?? UNAVAILABLE_LABEL}</dd></div>
        <div><dt class="field-label">URL kiểm tra tình trạng</dt><dd>{engine.healthUrl ?? UNAVAILABLE_LABEL}</dd></div>
        <div><dt class="field-label">Lỗi gần nhất</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.lastError ?? UNAVAILABLE_LABEL}</dd></div>
        <div><dt class="field-label">Mã thoát gần nhất</dt><dd>{engine.lastExitCode ?? UNAVAILABLE_LABEL}</dd></div>
      </dl>
    </div>

    <div class="mt-6 grid gap-4 lg:grid-cols-2">
      <div>
        <h3 class="mb-3 text-base font-semibold text-[#1b2430]">Giá trị đã phân giải</h3>
        <dl class="space-y-3">
          <div><dt class="field-label">Nhị phân</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.resolvedBinaryPath ?? UNAVAILABLE_LABEL}</dd></div>
          {#if engine.kind === "llama_cpp"}
            <div><dt class="field-label">Tệp GGUF</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.resolvedModelPath ?? UNAVAILABLE_LABEL}</dd></div>
          {:else if engine.kind === "sherpa_onnx"}
            <div><dt class="field-label">Thư mục Sherpa</dt><dd class="break-all text-sm text-[#5e6a79]">{engine.resolvedModelDir ?? engine.resolvedModelPath ?? UNAVAILABLE_LABEL}</dd></div>
          {/if}
        </dl>
      </div>

      <div>
        <h3 class="mb-3 text-base font-semibold text-[#1b2430]">Đối số khởi chạy</h3>
        <pre class="code-block">{JSON.stringify(engine.generatedArgs ?? [], null, 2)}</pre>
      </div>
    </div>

    <details class="mt-5">
      <summary class="cursor-pointer text-sm font-semibold text-[#355c7d]">Xem JSON thời gian chạy và đối số bổ sung</summary>
      <dl class="mt-3 grid gap-4 lg:grid-cols-2">
        <div><dt class="field-label">JSON thời gian chạy</dt><dd><pre class="code-block">{JSON.stringify(engine.runtime, null, 2)}</pre></dd></div>
        <div><dt class="field-label">Đối số bổ sung</dt><dd><pre class="code-block">{JSON.stringify(engine.extraArgs, null, 2)}</pre></dd></div>
      </dl>
    </details>
  {:else}
    <p class="text-sm text-[#5e6a79]">Chọn một động cơ để xem các chi tiết đã phân giải.</p>
  {/if}
</section>

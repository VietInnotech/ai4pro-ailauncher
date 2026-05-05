<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { formatEngineStatusLabel, UNAVAILABLE_LABEL } from "$lib/labels";
  import type { DeveloperEngineProfileDto } from "$lib/types/developer";

  export let engines: DeveloperEngineProfileDto[] = [];

  const dispatch = createEventDispatcher<{ reload: void }>();

  $: runningCount = engines.filter((engine) => engine.status === "running").length;
  $: attentionCount = engines.filter((engine) => ["unhealthy", "crashed", "missing_binary", "missing_model", "invalid_config", "port_conflict"].includes(String(engine.status))).length;
</script>

<section class="space-y-4">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-lg font-semibold text-[#1b2430]">Bảng điều khiển nhà phát triển</h2>
      <p class="text-sm text-[#5e6a79]">Ảnh chụp nhanh về trạng thái tổng hợp, trạng thái động cơ và mức độ sẵn sàng của máy.</p>
    </div>
    <button class="action-button-secondary" on:click={() => dispatch("reload")}>Làm mới</button>
  </div>

  <section class="grid gap-4 md:grid-cols-3">
    <div class="panel p-5">
      <p class="field-label">Local AI tổng thể</p>
      <p class="text-xl font-semibold text-[#1b2430]">{engines.length > 0 && runningCount === engines.length ? "Đang hoạt động" : attentionCount > 0 ? "Cần chú ý" : "Chưa hoạt động"}</p>
    </div>
    <div class="panel p-5">
      <p class="field-label">Động cơ đang hoạt động</p>
      <p class="text-xl font-semibold text-[#1b2430]">{runningCount}</p>
    </div>
    <div class="panel p-5">
      <p class="field-label">Mục cần chú ý</p>
      <p class="text-xl font-semibold text-[#1b2430]">{attentionCount}</p>
    </div>
  </section>

  <section class="grid gap-4 md:grid-cols-2">
    {#each engines as engine}
      <div class="panel p-5">
        <p class="field-label">{engine.name}</p>
        <p class="text-xl font-semibold text-[#1b2430]">{formatEngineStatusLabel(engine.status)}</p>
        <p class="mt-2 text-sm text-[#5e6a79]">URL kiểm tra tình trạng: {engine.healthUrl ?? UNAVAILABLE_LABEL}</p>
        <p class="mt-1 text-sm text-[#5e6a79]">PID: {engine.pid ?? UNAVAILABLE_LABEL}</p>
      </div>
    {/each}
  </section>
</section>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { formatBinaryModeLabel, formatEngineStatusLabel, UNAVAILABLE_LABEL } from "$lib/labels";
  import StatusBadge from "$lib/components/StatusBadge.svelte";
  import type { DeveloperEngineProfileDto } from "$lib/types/developer";

  export let engines: DeveloperEngineProfileDto[] = [];
  export let selectedId: string | null = null;

  const dispatch = createEventDispatcher<{ select: DeveloperEngineProfileDto }>();

  function toneFor(status: string): "neutral" | "ok" | "warn" | "danger" {
    if (["running"].includes(status)) return "ok";
    if (["starting", "stopping"].includes(status)) return "warn";
    if (["crashed", "unhealthy", "missing_binary", "missing_model", "invalid_config", "port_conflict"].includes(status)) {
      return "danger";
    }

    return "neutral";
  }
</script>

<section class="panel overflow-hidden">
  <div class="border-b px-6 py-4" style="border-color: #d5dce3;">
    <h2 class="text-lg font-semibold text-[#1b2430]">Hồ sơ động cơ</h2>
  </div>

  <div class="overflow-x-auto">
    <table class="min-w-full divide-y text-sm" style="--tw-divide-opacity: 1; border-color: #d5dce3;">
      <thead class="text-left text-[#5e6a79]" style="background-color: #eef2f5;">
        <tr>
          <th class="px-4 py-3 font-semibold">Động cơ</th>
          <th class="px-4 py-3 font-semibold">Loại</th>
          <th class="px-4 py-3 font-semibold">Trạng thái</th>
          <th class="px-4 py-3 font-semibold">PID</th>
          <th class="px-4 py-3 font-semibold">Máy chủ</th>
          <th class="px-4 py-3 font-semibold">Cổng</th>
          <th class="px-4 py-3 font-semibold">Tệp nhị phân</th>
          <th class="px-4 py-3 font-semibold">Mô hình</th>
        </tr>
      </thead>
      <tbody class="divide-y bg-white" style="--tw-divide-opacity: 1; border-color: #d5dce3;">
        {#each engines as engine}
          <tr class={selectedId === engine.id ? "bg-[#edf2f6]" : ""}>
            <td class="px-4 py-3 align-top">
              <button class="font-semibold text-[#1b2430] hover:text-[#355c7d]" on:click={() => dispatch("select", engine)}>{engine.name}</button>
              <div class="text-xs text-[#5e6a79]">{engine.id}</div>
            </td>
            <td class="px-4 py-3">{engine.kind}</td>
            <td class="px-4 py-3"><StatusBadge tone={toneFor(engine.status)} text={formatEngineStatusLabel(engine.status)} /></td>
            <td class="px-4 py-3">{engine.pid ?? UNAVAILABLE_LABEL}</td>
            <td class="px-4 py-3">{engine.host}</td>
            <td class="px-4 py-3">{engine.port}</td>
            <td class="px-4 py-3">{formatBinaryModeLabel(engine.binaryMode)}</td>
            <td class="px-4 py-3">{engine.modelPackageId ?? UNAVAILABLE_LABEL}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

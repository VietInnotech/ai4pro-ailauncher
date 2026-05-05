<script lang="ts">
  import { createEventDispatcher } from "svelte";
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
  <div class="border-b border-slate-200 px-6 py-4">
    <h2 class="text-lg font-semibold text-slate-950">Engine profiles</h2>
  </div>

  <div class="overflow-x-auto">
    <table class="min-w-full divide-y divide-slate-200 text-sm">
      <thead class="bg-slate-50 text-left text-slate-500">
        <tr>
          <th class="px-4 py-3 font-semibold">Engine</th>
          <th class="px-4 py-3 font-semibold">Kind</th>
          <th class="px-4 py-3 font-semibold">Status</th>
          <th class="px-4 py-3 font-semibold">PID</th>
          <th class="px-4 py-3 font-semibold">Host</th>
          <th class="px-4 py-3 font-semibold">Port</th>
          <th class="px-4 py-3 font-semibold">Binary</th>
          <th class="px-4 py-3 font-semibold">Model</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200 bg-white">
        {#each engines as engine}
          <tr class={selectedId === engine.id ? "bg-indigo-50/60" : ""}>
            <td class="px-4 py-3 align-top">
              <button class="font-semibold text-slate-900 hover:text-indigo-700" on:click={() => dispatch("select", engine)}>{engine.name}</button>
              <div class="text-xs text-slate-500">{engine.id}</div>
            </td>
            <td class="px-4 py-3">{engine.kind}</td>
            <td class="px-4 py-3"><StatusBadge tone={toneFor(engine.status)} text={engine.status} /></td>
            <td class="px-4 py-3">{engine.pid ?? "—"}</td>
            <td class="px-4 py-3">{engine.host}</td>
            <td class="px-4 py-3">{engine.port}</td>
            <td class="px-4 py-3">{engine.binaryMode}</td>
            <td class="px-4 py-3">{engine.modelPackageId ?? "—"}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

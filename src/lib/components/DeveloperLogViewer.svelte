<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { DeveloperEngineProfileDto, DeveloperLogType } from "$lib/types/developer";

  export let engines: DeveloperEngineProfileDto[] = [];
  export let selectedEngineId = "";
  export let logType: DeveloperLogType = "stdout";
  export let logText = "";
  export let loading = false;
  export let error = "";
  export let title = "Engine log";

  const dispatch = createEventDispatcher<{ reload: void; openFolder: void; selectionChange: void }>();
</script>

<section class="panel overflow-hidden">
  <div class="flex flex-wrap items-center justify-between gap-3 border-b border-slate-200 px-6 py-4">
    <h2 class="text-lg font-semibold text-slate-950">{title}</h2>

    <div class="flex flex-wrap items-center gap-2">
      <select class="rounded-xl border border-slate-300 bg-white px-3 py-2 text-sm" bind:value={selectedEngineId} on:change={() => dispatch("selectionChange")}>
        {#each engines as engine}
          <option value={engine.id}>{engine.name}</option>
        {/each}
      </select>

      <select class="rounded-xl border border-slate-300 bg-white px-3 py-2 text-sm" bind:value={logType} on:change={() => dispatch("selectionChange")}>
        <option value="stdout">stdout</option>
        <option value="stderr">stderr</option>
      </select>

      <button class="action-button-secondary" disabled={loading} on:click={() => dispatch("reload")}>Reload</button>
      <button class="action-button-secondary" on:click={() => dispatch("openFolder")}>Open folder</button>
    </div>
  </div>

  {#if error}
    <div class="border-b border-red-200 bg-red-50 px-6 py-3 text-sm text-red-700">{error}</div>
  {/if}

  <pre class="max-h-[28rem] overflow-auto bg-slate-950 p-6 text-xs leading-6 text-slate-100">{loading ? "Loading log..." : logText || "No log output yet."}</pre>
</section>

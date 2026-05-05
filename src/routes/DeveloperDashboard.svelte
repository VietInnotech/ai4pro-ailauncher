<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { DeveloperEngineProfileDto } from "$lib/types/developer";

  export let engines: DeveloperEngineProfileDto[] = [];

  const dispatch = createEventDispatcher<{ reload: void }>();

  $: runningCount = engines.filter((engine) => engine.status === "running").length;
  $: attentionCount = engines.filter((engine) => ["unhealthy", "crashed", "missing_binary", "missing_model", "invalid_config", "port_conflict"].includes(String(engine.status))).length;
</script>

<section class="space-y-4">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-lg font-semibold text-slate-950">Developer dashboard</h2>
      <p class="text-sm text-slate-500">Aggregate state, engine status, and machine readiness snapshot.</p>
    </div>
    <button class="action-button-secondary" on:click={() => dispatch("reload")}>Refresh</button>
  </div>

  <section class="grid gap-4 md:grid-cols-3">
    <div class="panel p-5">
      <p class="field-label">Aggregate Local AI</p>
      <p class="text-xl font-semibold text-slate-950">{engines.length > 0 && runningCount === engines.length ? "Running" : attentionCount > 0 ? "Needs attention" : "Idle"}</p>
    </div>
    <div class="panel p-5">
      <p class="field-label">Running engines</p>
      <p class="text-xl font-semibold text-slate-950">{runningCount}</p>
    </div>
    <div class="panel p-5">
      <p class="field-label">Needs attention</p>
      <p class="text-xl font-semibold text-slate-950">{attentionCount}</p>
    </div>
  </section>

  <section class="grid gap-4 md:grid-cols-2">
    {#each engines as engine}
      <div class="panel p-5">
        <p class="field-label">{engine.name}</p>
        <p class="text-xl font-semibold text-slate-950">{engine.status}</p>
        <p class="mt-2 text-sm text-slate-500">Health URL: {engine.healthUrl ?? "Unavailable"}</p>
        <p class="mt-1 text-sm text-slate-500">PID: {engine.pid ?? "—"}</p>
      </div>
    {/each}
  </section>
</section>

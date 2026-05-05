<script lang="ts">
  import { createEventDispatcher } from "svelte";
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
      <h2 class="text-lg font-semibold text-slate-950">Engine detail</h2>
      <p class="mt-1 text-sm text-slate-500">Resolved values are visible only in Developer Mode.</p>
    </div>
    {#if engine}
      <div class="flex flex-wrap gap-2">
        <button class="action-button-secondary" disabled={busy} on:click={() => dispatch("validate", engine.id)}>Validate</button>
        <button class="action-button-secondary" disabled={busy} on:click={() => dispatch("refresh", engine.id)}>Reload</button>
        <button class="action-button-primary" disabled={busy} on:click={() => dispatch("start", engine.id)}>Start</button>
        <button class="action-button-danger" disabled={busy} on:click={() => dispatch("stop", engine.id)}>Stop</button>
        <button class="action-button-secondary" disabled={busy} on:click={() => dispatch("restart", engine.id)}>Restart</button>
      </div>
    {/if}
  </div>

  {#if engine}
    <dl class="grid gap-4 sm:grid-cols-2">
      <div><dt class="field-label">Engine ID</dt><dd>{engine.id}</dd></div>
      <div><dt class="field-label">Kind</dt><dd>{engine.kind}</dd></div>
      <div><dt class="field-label">Binary name</dt><dd>{engine.binaryName}</dd></div>
      <div><dt class="field-label">Resolved binary path</dt><dd class="break-all text-sm text-slate-600">{engine.resolvedBinaryPath ?? "—"}</dd></div>
      <div><dt class="field-label">Resolved model path</dt><dd class="break-all text-sm text-slate-600">{engine.resolvedModelPath ?? "—"}</dd></div>
      <div><dt class="field-label">Resolved model dir</dt><dd class="break-all text-sm text-slate-600">{engine.resolvedModelDir ?? "—"}</dd></div>
      <div><dt class="field-label">Resolved tokens path</dt><dd class="break-all text-sm text-slate-600">{engine.resolvedTokensPath ?? "—"}</dd></div>
      <div><dt class="field-label">Health URL</dt><dd>{engine.healthUrl ?? "—"}</dd></div>
      <div><dt class="field-label">Status</dt><dd>{engine.status}</dd></div>
      <div><dt class="field-label">PID</dt><dd>{engine.pid ?? "—"}</dd></div>
      <div><dt class="field-label">Last error</dt><dd class="break-all text-sm text-slate-600">{engine.lastError ?? "—"}</dd></div>
      <div><dt class="field-label">Last exit code</dt><dd>{engine.lastExitCode ?? "—"}</dd></div>
      <div class="sm:col-span-2"><dt class="field-label">Runtime JSON</dt><dd><pre class="overflow-x-auto rounded-2xl bg-slate-950 p-4 text-xs text-slate-100">{JSON.stringify(engine.runtime, null, 2)}</pre></dd></div>
      <div class="sm:col-span-2"><dt class="field-label">Extra args</dt><dd><pre class="overflow-x-auto rounded-2xl bg-slate-950 p-4 text-xs text-slate-100">{JSON.stringify(engine.extraArgs, null, 2)}</pre></dd></div>
      <div class="sm:col-span-2"><dt class="field-label">Generated args</dt><dd><pre class="overflow-x-auto rounded-2xl bg-slate-950 p-4 text-xs text-slate-100">{JSON.stringify(engine.generatedArgs ?? [], null, 2)}</pre></dd></div>
    </dl>
  {:else}
    <p class="text-sm text-slate-500">Select an engine to inspect its resolved details.</p>
  {/if}
</section>

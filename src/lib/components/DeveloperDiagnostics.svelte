<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { DiagnosticsBundleDto } from "$lib/types/developer";

  export let bundle: DiagnosticsBundleDto | null = null;

  const dispatch = createEventDispatcher<{ export: void }>();
</script>

<section class="panel p-6">
  <div class="mb-6 flex items-center justify-between gap-4">
    <div>
      <h2 class="text-lg font-semibold text-slate-950">Diagnostics</h2>
      <p class="mt-1 text-sm text-slate-500">Developer-facing environment and validation snapshot.</p>
    </div>
    <button class="action-button-secondary" on:click={() => dispatch("export")}>Export diagnostics bundle</button>
  </div>

  {#if bundle}
    <div class="space-y-6">
      <dl class="grid gap-4 sm:grid-cols-2">
        <div><dt class="field-label">App version</dt><dd>{bundle.appVersion}</dd></div>
        <div><dt class="field-label">OS / Arch</dt><dd>{bundle.os} / {bundle.arch}</dd></div>
        <div><dt class="field-label">App data root</dt><dd class="break-all text-sm text-slate-600">{bundle.appDataRoot}</dd></div>
        <div><dt class="field-label">SQLite path</dt><dd class="break-all text-sm text-slate-600">{bundle.sqlitePath}</dd></div>
        <div><dt class="field-label">Logs root</dt><dd class="break-all text-sm text-slate-600">{bundle.logsRoot}</dd></div>
        <div><dt class="field-label">Machine configured</dt><dd>{bundle.machineConfigured ? "Yes" : "No"}</dd></div>
      </dl>

      <div>
        <h3 class="text-base font-semibold text-slate-950">Validation results</h3>
        <ul class="mt-3 space-y-2 text-sm text-slate-600">
          {#each bundle.validation as result}
            <li class="rounded-xl border border-slate-200 px-3 py-2">
              <span class="font-medium text-slate-900">{result.engineId}</span>
              — {result.valid ? "valid" : `${result.issues.length} issue(s)`}
            </li>
          {/each}
        </ul>
      </div>

      {#if bundle.runtimeState?.length}
        <div>
          <h3 class="text-base font-semibold text-slate-950">Runtime state</h3>
          <ul class="mt-3 space-y-2 text-sm text-slate-600">
            {#each bundle.runtimeState as entry}
              <li class="rounded-xl border border-slate-200 px-3 py-2">
                <span class="font-medium text-slate-900">{entry.engineId}</span>
                — {entry.status} {entry.pid ? `(pid ${entry.pid})` : ""}
              </li>
            {/each}
          </ul>
        </div>
      {/if}

      {#if bundle.recentLogs?.length}
        <div>
          <h3 class="text-base font-semibold text-slate-950">Recent logs</h3>
          <div class="mt-3 space-y-3">
            {#each bundle.recentLogs as log}
              <div>
                <p class="mb-2 text-sm font-medium text-slate-900">{log.name}</p>
                <pre class="max-h-48 overflow-auto rounded-2xl bg-slate-950 p-4 text-xs text-slate-100">{log.content}</pre>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <p class="text-sm text-slate-500">Load diagnostics to inspect machine configuration, validation, recent logs, and runtime state.</p>
  {/if}
</section>

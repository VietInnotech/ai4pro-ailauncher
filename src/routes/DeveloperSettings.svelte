<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { appSettingsStore } from "$lib/stores/app-settings";
  import type { AppSettingsDto } from "$lib/types/settings";

  let state: {
    value?: AppSettingsDto;
    loading: boolean;
    error?: string;
  } = { loading: false };
  let stopOnExit = true;
  let autoStart = false;

  const unsubscribe = appSettingsStore.subscribe((value: typeof state) => {
    state = value;
  });

  onMount(() => {
    void appSettingsStore.refresh();
  });

  onDestroy(() => {
    unsubscribe();
  });

  $: if (state.value) {
    stopOnExit = state.value.stopEnginesOnExit;
    autoStart = state.value.autoStartLocalAi;
  }

  async function save() {
    await appSettingsStore.save({
      stopEnginesOnExit: stopOnExit,
      autoStartLocalAi: autoStart
    });
  }
</script>

<section class="panel p-6">
  <div class="mb-6">
    <h2 class="text-lg font-semibold text-slate-950">Settings</h2>
    <p class="mt-1 text-sm text-slate-500">Developer-only runtime behavior toggles.</p>
  </div>

  <div class="space-y-4">
    <label class="flex items-center justify-between gap-4 rounded-2xl border border-slate-200 px-4 py-3">
      <span>
        <span class="block font-medium text-slate-900">Stop engines when app exits</span>
        <span class="block text-sm text-slate-500">Avoid orphaned local services by default.</span>
      </span>
      <input bind:checked={stopOnExit} type="checkbox" />
    </label>

    <label class="flex items-center justify-between gap-4 rounded-2xl border border-slate-200 px-4 py-3">
      <span>
        <span class="block font-medium text-slate-900">Start Local AI when app opens</span>
        <span class="block text-sm text-slate-500">Runs validation before starting engines.</span>
      </span>
      <input bind:checked={autoStart} type="checkbox" />
    </label>
  </div>

  <div class="mt-6 flex items-center gap-3">
    <button class="action-button-primary" disabled={state.loading} on:click={save}>Save settings</button>
    {#if state.error}
      <span class="text-sm text-red-600">{state.error}</span>
    {/if}
  </div>
</section>

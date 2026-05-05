<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import {
    devGetEngineProfile,
    devRestartEngineProfile,
    devStartEngineProfile,
    devStopEngineProfile,
    devValidateEngineProfile
  } from "$lib/api/developer";
  import type { DeveloperEngineProfileDto, ValidationResultDto } from "$lib/types/developer";
  import DeveloperEngineDetail from "$lib/components/DeveloperEngineDetail.svelte";
  import DeveloperEngineTable from "$lib/components/DeveloperEngineTable.svelte";

  export let engines: DeveloperEngineProfileDto[] = [];

  const dispatch = createEventDispatcher<{ reload: void }>();

  let selected: DeveloperEngineProfileDto | null = null;
  let validation: ValidationResultDto | null = null;
  let validationError = "";
  let busy = false;

  $: if (!selected && engines[0]) selected = engines[0];
  $: if (selected && !engines.some((engine) => engine.id === selected?.id)) selected = engines[0] ?? null;

  function selectEngine(engine: DeveloperEngineProfileDto) {
    selected = engine;
    validation = null;
    validationError = "";
  }

  async function refreshSelected(engineId: string) {
    selected = await devGetEngineProfile(engineId);
    dispatch("reload");
  }

  async function validateSelected(engineId: string) {
    try {
      validationError = "";
      validation = await devValidateEngineProfile(engineId);
    } catch (error) {
      validationError = error instanceof Error ? error.message : String(error);
    }
  }

  async function runAction(action: "start" | "stop" | "restart") {
    if (!selected || busy) return;

    busy = true;
    validationError = "";
    try {
      if (action === "start") {
        selected = await devStartEngineProfile(selected.id);
      } else if (action === "stop") {
        selected = await devStopEngineProfile(selected.id);
      } else {
        selected = await devRestartEngineProfile(selected.id);
      }
      validation = await devValidateEngineProfile(selected.id);
      dispatch("reload");
    } catch (error) {
      validationError = error instanceof Error ? error.message : String(error);
    } finally {
      busy = false;
    }
  }
</script>

<section class="space-y-4">
  <DeveloperEngineTable {engines} selectedId={selected?.id ?? null} on:select={(event) => selectEngine(event.detail)} />

  <div class="grid gap-4 lg:grid-cols-[minmax(0,2fr)_minmax(20rem,1fr)]">
    <DeveloperEngineDetail
      engine={selected}
      {busy}
      on:refresh={(event) => refreshSelected(event.detail)}
      on:validate={(event) => validateSelected(event.detail)}
      on:start={() => runAction("start")}
      on:stop={() => runAction("stop")}
      on:restart={() => runAction("restart")}
    />

    <section class="panel p-6">
      <h3 class="text-base font-semibold text-slate-950">Validation</h3>
      {#if validationError}
        <p class="mt-3 rounded-xl border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">{validationError}</p>
      {:else if validation}
        <div class="mt-3 space-y-3">
          <p class={`text-sm font-medium ${validation.valid ? "text-emerald-700" : "text-amber-700"}`}>
            {validation.valid ? "Profile is valid." : "Profile has issues."}
          </p>
          {#if validation.issues.length > 0}
            <ul class="space-y-2 text-sm text-slate-600">
              {#each validation.issues as issue}
                <li class="rounded-xl border border-slate-200 px-3 py-2">
                  <span class="font-medium text-slate-900">{issue.code}</span>: {issue.message}
                </li>
              {/each}
            </ul>
          {:else}
            <p class="text-sm text-slate-500">No validation issues found.</p>
          {/if}
        </div>
      {:else}
        <p class="mt-3 text-sm text-slate-500">Select an engine and run validation to inspect launch readiness.</p>
      {/if}
    </section>
  </div>
</section>

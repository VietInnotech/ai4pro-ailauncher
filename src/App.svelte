<script lang="ts">
  import { onDestroy } from "svelte";
  import {
    devGetDiagnosticsBundle,
    devListEngineProfiles,
    devListModelPackages,
    disableDeveloperModeForSession
  } from "$lib/api/developer";
  import DeveloperLayout from "$lib/components/DeveloperLayout.svelte";
  import DeveloperModeGate from "$lib/components/DeveloperModeGate.svelte";
  import { developerMode } from "$lib/stores/developer-mode";
  import type {
    DeveloperEngineProfileDto,
    DeveloperModelPackageDto,
    DiagnosticsBundleDto
  } from "$lib/types/developer";
  import DeveloperDashboard from "$routes/DeveloperDashboard.svelte";
  import DeveloperDiagnostics from "$routes/DeveloperDiagnostics.svelte";
  import DeveloperEngines from "$routes/DeveloperEngines.svelte";
  import DeveloperLogs from "$routes/DeveloperLogs.svelte";
  import DeveloperModels from "$routes/DeveloperModels.svelte";
  import DeveloperSettings from "$routes/DeveloperSettings.svelte";
  import SimpleHome from "$routes/SimpleHome.svelte";

  const developerViews = ["dashboard", "engines", "models", "logs", "settings", "diagnostics"] as const;
  type DeveloperView = (typeof developerViews)[number];

  let devEnabled = false;
  let engines: DeveloperEngineProfileDto[] = [];
  let models: DeveloperModelPackageDto[] = [];
  let diagnostics: DiagnosticsBundleDto | null = null;
  let activeView: DeveloperView = "dashboard";
  let developerNotice = "";

  const unsubscribe = developerMode.subscribe((value) => {
    devEnabled = value;
    if (value) {
      developerNotice = "Developer Mode enabled. Advanced settings and diagnostics are now visible.";
      void loadDeveloperData();
    }
  });

  onDestroy(() => {
    unsubscribe();
  });

  async function loadDeveloperData() {
    try {
      const [loadedEngines, loadedModels, loadedDiagnostics] = await Promise.all([
        devListEngineProfiles(),
        devListModelPackages(),
        devGetDiagnosticsBundle()
      ]);

      engines = loadedEngines;
      models = loadedModels;
      diagnostics = loadedDiagnostics;
    } catch (error) {
      developerNotice = error instanceof Error ? error.message : "Failed to load developer data.";
    }
  }

  async function disableDeveloperMode() {
    await disableDeveloperModeForSession();
    developerMode.set(false);
    developerNotice = "Developer Mode disabled for this session.";
  }

  function setActiveView(view: DeveloperView) {
    activeView = view;
  }
</script>

<svelte:head>
  <title>AI Launcher</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-b from-slate-100 to-slate-200 px-6 py-10 text-slate-900">
  <div class="mx-auto flex max-w-6xl flex-col gap-8">
    <SimpleHome />

    <DeveloperModeGate enabled={devEnabled}>
      <section class="space-y-4">
        <div class="flex items-center justify-between gap-4 rounded-2xl border border-indigo-200 bg-indigo-50 px-5 py-4 text-sm text-indigo-900">
          <span>{developerNotice}</span>
          <div class="flex gap-2">
            <button class="action-button-secondary" on:click={loadDeveloperData}>Refresh developer data</button>
            <button class="action-button-secondary" on:click={disableDeveloperMode}>Hide developer tools</button>
          </div>
        </div>

        <div class="flex flex-wrap gap-2">
          {#each developerViews as view}
            <button
              class={`rounded-full px-4 py-2 text-sm font-semibold transition ${activeView === view ? "bg-slate-900 text-white" : "bg-white text-slate-600 hover:bg-slate-50"}`}
              on:click={() => setActiveView(view)}
            >
              {view}
            </button>
          {/each}
        </div>

        <DeveloperLayout active={activeView}>
          {#if activeView === "dashboard"}
            <DeveloperDashboard {engines} on:reload={loadDeveloperData} />
          {:else if activeView === "engines"}
            <DeveloperEngines {engines} on:reload={loadDeveloperData} />
          {:else if activeView === "models"}
            <DeveloperModels {models} on:reload={loadDeveloperData} />
          {:else if activeView === "logs"}
            <DeveloperLogs {engines} />
          {:else if activeView === "settings"}
            <DeveloperSettings />
          {:else if activeView === "diagnostics"}
            <DeveloperDiagnostics {diagnostics} />
          {/if}
        </DeveloperLayout>
      </section>
    </DeveloperModeGate>
  </div>
</div>

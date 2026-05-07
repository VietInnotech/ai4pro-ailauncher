<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    devGetDiagnosticsBundle,
    devListEngineProfiles,
    devListModelPackages,
    disableDeveloperModeForSession
  } from "$lib/api/developer";
  import DeveloperLayout from "$lib/components/DeveloperLayout.svelte";
  import DeveloperModeGate from "$lib/components/DeveloperModeGate.svelte";
  import { developerMode } from "$lib/stores/developer-mode";
  import { localAiStore, type LocalAiStoreState } from "$lib/stores/local-ai";
  import type {
    DeveloperEngineProfileDto,
    DeveloperModelPackageDto,
    DiagnosticsBundleDto
  } from "$lib/types/developer";
  import { defaultSimpleStatus, type SimpleLocalAiStatusDto } from "$lib/types/local-ai";
  import DeveloperDashboard from "$routes/DeveloperDashboard.svelte";
  import DeveloperDiagnostics from "$routes/DeveloperDiagnostics.svelte";
  import DeveloperEngines from "$routes/DeveloperEngines.svelte";
  import DeveloperLogs from "$routes/DeveloperLogs.svelte";
  import DeveloperModels from "$routes/DeveloperModels.svelte";
  import DeveloperSettings from "$routes/DeveloperSettings.svelte";
  import SimpleHome from "$routes/SimpleHome.svelte";

  const developerViews = [
    { id: "dashboard", label: "Tổng quan" },
    { id: "engines", label: "Động cơ" },
    { id: "models", label: "Mô hình" },
    { id: "logs", label: "Nhật ký" },
    { id: "settings", label: "Cài đặt" },
    { id: "diagnostics", label: "Chẩn đoán" }
  ] as const;
  type DeveloperView = (typeof developerViews)[number]["id"];

  let devEnabled = false;
  let engines: DeveloperEngineProfileDto[] = [];
  let models: DeveloperModelPackageDto[] = [];
  let diagnostics: DiagnosticsBundleDto | null = null;
  let activeView: DeveloperView = "dashboard";
  let developerNotice = "";
  let localAiState: LocalAiStoreState = {
    status: defaultSimpleStatus,
    loading: false,
    lastCheckedAt: undefined
  };
  let simpleStatus: SimpleLocalAiStatusDto = defaultSimpleStatus;

  const unsubscribeDeveloperMode = developerMode.subscribe((value) => {
    devEnabled = value;
    if (value) {
      developerNotice = "Chế độ nhà phát triển đã được bật. Các cài đặt nâng cao và chẩn đoán hiện đã hiển thị.";
      void loadDeveloperData();
    }
  });

  const unsubscribeLocalAi = localAiStore.subscribe((value) => {
    localAiState = value;
    simpleStatus = value.status;
  });

  onMount(() => {
    void refreshVisibleRuntimeData();
  });

  onDestroy(() => {
    unsubscribeDeveloperMode();
    unsubscribeLocalAi();
  });

  async function loadDeveloperData(refreshSimpleStatus = true) {
    try {
      const [loadedEngines, loadedModels, loadedDiagnostics] = await Promise.all([
        devListEngineProfiles(),
        devListModelPackages(),
        devGetDiagnosticsBundle()
      ]);

      engines = loadedEngines;
      models = loadedModels;
      diagnostics = loadedDiagnostics;

      if (refreshSimpleStatus) {
        await localAiStore.refresh();
      }
    } catch (error) {
      developerNotice = error instanceof Error ? error.message : "Không thể tải dữ liệu dành cho nhà phát triển.";
    }
  }

  async function refreshVisibleRuntimeData() {
    if (devEnabled) {
      await loadDeveloperData();
      return;
    }

    await localAiStore.refresh();
  }

  async function runLocalAiAction(action: "start" | "stop" | "restart") {
    if (action === "start") {
      await localAiStore.start();
    } else if (action === "stop") {
      await localAiStore.stop();
    } else {
      await localAiStore.restart();
    }

    if (devEnabled) {
      await loadDeveloperData();
    }
  }

  async function checkAllModels() {
    await localAiStore.checkAllModels();

    if (devEnabled) {
      await loadDeveloperData();
    }
  }

  async function disableDeveloperMode() {
    await disableDeveloperModeForSession();
    developerMode.set(false);
    developerNotice = "Chế độ nhà phát triển đã bị tắt cho phiên này.";
  }

  function setActiveView(view: DeveloperView) {
    activeView = view;
  }
</script>

<svelte:head>
  <title>Trình khởi chạy AI</title>
</svelte:head>

<div class="min-h-screen px-4 py-8 text-[#1b2430] sm:px-6 lg:px-8">
  <div class="mx-auto flex max-w-6xl flex-col gap-6">
    <SimpleHome
      state={localAiState}
      onRefresh={checkAllModels}
      onStart={() => runLocalAiAction("start")}
      onStop={() => runLocalAiAction("stop")}
      onRestart={() => runLocalAiAction("restart")}
    />

    <DeveloperModeGate enabled={devEnabled}>
      <section class="space-y-4">
        <div class="panel flex flex-col gap-3 px-4 py-3 text-sm sm:flex-row sm:items-center sm:justify-between">
          <span>{developerNotice}</span>
          <div class="flex gap-2">
            <button class="action-button-secondary" on:click={() => loadDeveloperData()}>Làm mới dữ liệu nhà phát triển</button>
            <button class="action-button-secondary" on:click={disableDeveloperMode}>Ẩn công cụ nhà phát triển</button>
          </div>
        </div>

        <div class="developer-tabs" role="tablist" aria-label="Công cụ nhà phát triển">
          {#each developerViews as view}
            <button
              class={`developer-tab ${activeView === view.id ? "developer-tab-active" : ""}`}
              aria-selected={activeView === view.id}
              role="tab"
              on:click={() => setActiveView(view.id)}
            >
              {view.label}
            </button>
          {/each}
        </div>

        <DeveloperLayout>
          {#if activeView === "dashboard"}
            <DeveloperDashboard {engines} {simpleStatus} on:reload={() => loadDeveloperData()} />
          {:else if activeView === "engines"}
            <DeveloperEngines {engines} on:reload={() => loadDeveloperData()} />
          {:else if activeView === "models"}
            <DeveloperModels {models} on:reload={() => loadDeveloperData()} />
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

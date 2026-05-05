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

  const unsubscribe = developerMode.subscribe((value) => {
    devEnabled = value;
    if (value) {
      developerNotice = "Chế độ nhà phát triển đã được bật. Các cài đặt nâng cao và chẩn đoán hiện đã hiển thị.";
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
      developerNotice = error instanceof Error ? error.message : "Không thể tải dữ liệu dành cho nhà phát triển.";
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
    <SimpleHome />

    <DeveloperModeGate enabled={devEnabled}>
      <section class="space-y-4">
        <div class="panel flex flex-col gap-3 px-4 py-3 text-sm sm:flex-row sm:items-center sm:justify-between">
          <span>{developerNotice}</span>
          <div class="flex gap-2">
            <button class="action-button-secondary" on:click={loadDeveloperData}>Làm mới dữ liệu nhà phát triển</button>
            <button class="action-button-secondary" on:click={disableDeveloperMode}>Ẩn công cụ nhà phát triển</button>
          </div>
        </div>

        <div class="flex flex-wrap gap-2">
          {#each developerViews as view}
            <button
              class={`rounded-full border px-4 py-2 text-sm font-semibold transition ${activeView === view.id ? "border-[#1b2430] bg-[#1b2430] text-white" : "border-[#d5dce3] bg-[#fcfcfd] text-[#5e6a79] hover:bg-[#eef2f5]"}`}
              on:click={() => setActiveView(view.id)}
            >
              {view.label}
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

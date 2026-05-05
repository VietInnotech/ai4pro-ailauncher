<script lang="ts">
  import { onMount } from "svelte";
  import { devOpenLogsFolder, devReadEngineLog } from "$lib/api/developer";
  import DeveloperLogViewer from "$lib/components/DeveloperLogViewer.svelte";
  import type { DeveloperEngineProfileDto, DeveloperLogType } from "$lib/types/developer";

  export let engines: DeveloperEngineProfileDto[] = [];

  let selectedEngineId = "";
  let logType: DeveloperLogType = "stdout";
  let logText = "";
  let error = "";
  let loading = false;

  $: if (!selectedEngineId && engines[0]) selectedEngineId = engines[0].id;

  onMount(() => {
    void loadLog();
  });

  async function loadLog() {
    if (!selectedEngineId) return;
    loading = true;
    error = "";
    try {
      logText = await devReadEngineLog(selectedEngineId, logType, 200);
    } catch (value) {
      error = value instanceof Error ? value.message : String(value);
      logText = "";
    } finally {
      loading = false;
    }
  }

  async function openFolder() {
    if (!selectedEngineId) return;
    await devOpenLogsFolder(selectedEngineId);
  }
</script>

<DeveloperLogViewer
  title="Engine logs"
  {engines}
  bind:selectedEngineId
  bind:logType
  {logText}
  {loading}
  {error}
  on:reload={loadLog}
  on:openFolder={openFolder}
  on:selectionChange={loadLog}
/>

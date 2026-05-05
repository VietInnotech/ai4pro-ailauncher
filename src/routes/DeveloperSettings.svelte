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
    <h2 class="text-lg font-semibold text-[#1b2430]">Cài đặt</h2>
    <p class="mt-1 text-sm text-[#5e6a79]">Các tùy chọn hành vi thời gian chạy chỉ dành cho nhà phát triển.</p>
  </div>

  <div class="space-y-4">
    <label class="flex items-center justify-between gap-4 rounded-md border px-4 py-3" style="border-color: #d5dce3;">
      <span>
        <span class="block font-medium text-[#1b2430]">Dừng các động cơ khi ứng dụng thoát</span>
        <span class="block text-sm text-[#5e6a79]">Mặc định tránh để lại các dịch vụ cục bộ chạy mồ côi.</span>
      </span>
      <input bind:checked={stopOnExit} type="checkbox" />
    </label>

    <label class="flex items-center justify-between gap-4 rounded-md border px-4 py-3" style="border-color: #d5dce3;">
      <span>
        <span class="block font-medium text-[#1b2430]">Khởi động Local AI khi mở ứng dụng</span>
        <span class="block text-sm text-[#5e6a79]">Chạy kiểm tra trước khi khởi động các động cơ.</span>
      </span>
      <input bind:checked={autoStart} type="checkbox" />
    </label>
  </div>

  <div class="mt-6 flex items-center gap-3">
    <button class="action-button-primary" disabled={state.loading} on:click={save}>Lưu cài đặt</button>
    {#if state.error}
      <span class="text-sm text-[#9a3d3d]">{state.error}</span>
    {/if}
  </div>
</section>

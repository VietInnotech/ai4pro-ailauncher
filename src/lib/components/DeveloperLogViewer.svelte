<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { formatLogTypeLabel } from "$lib/labels";
  import type { DeveloperEngineProfileDto, DeveloperLogType } from "$lib/types/developer";

  export let engines: DeveloperEngineProfileDto[] = [];
  export let selectedEngineId = "";
  export let logType: DeveloperLogType = "stdout";
  export let logText = "";
  export let loading = false;
  export let error = "";
  export let title = "Nhật ký động cơ";

  const dispatch = createEventDispatcher<{ reload: void; openFolder: void; selectionChange: void }>();
</script>

<section class="panel overflow-hidden">
  <div class="flex flex-wrap items-center justify-between gap-3 border-b px-6 py-4" style="border-color: #d5dce3;">
    <h2 class="text-lg font-semibold text-[#1b2430]">{title}</h2>

    <div class="flex flex-wrap items-center gap-2">
      <select class="select-field" bind:value={selectedEngineId} on:change={() => dispatch("selectionChange")}>
        {#each engines as engine}
          <option value={engine.id}>{engine.name}</option>
        {/each}
      </select>

      <select class="select-field" bind:value={logType} on:change={() => dispatch("selectionChange")}>
        <option value="stdout">{formatLogTypeLabel("stdout")}</option>
        <option value="stderr">{formatLogTypeLabel("stderr")}</option>
      </select>

      <button class="action-button-secondary" disabled={loading} on:click={() => dispatch("reload")}>Tải lại</button>
      <button class="action-button-secondary" on:click={() => dispatch("openFolder")}>Mở thư mục</button>
    </div>
  </div>

  {#if error}
    <div class="border-b px-6 py-3 text-sm text-[#9a3d3d]" style="border-color: #efcaca; background-color: #fbebeb;">{error}</div>
  {/if}

  <pre class="code-block max-h-[28rem] rounded-none border-0 p-6">{loading ? "Đang tải nhật ký..." : logText || "Chưa có đầu ra nhật ký."}</pre>
</section>

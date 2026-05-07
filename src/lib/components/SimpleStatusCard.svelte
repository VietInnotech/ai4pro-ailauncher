<script lang="ts">
  import { formatSimpleModelStatusLabel, formatSimpleStatusLabel } from "$lib/labels";
  import StatusBadge from "$lib/components/StatusBadge.svelte";
  import LocalAiControl from "$lib/components/LocalAiControl.svelte";
  import SimpleErrorMessage from "$lib/components/SimpleErrorMessage.svelte";
  import type { SimpleLocalAiStatusDto, SimpleModelStatus } from "$lib/types/local-ai";

  export let status: SimpleLocalAiStatusDto;
  export let busy = false;
  export let notice = "";
  export let lastCheckedAt: string | undefined = undefined;
  export let onLogoClick: () => void | Promise<void> = () => {};
  export let onRefresh: () => void | Promise<void> = () => {};
  export let onStart: () => void | Promise<void> = () => {};
  export let onStop: () => void | Promise<void> = () => {};
  export let onRestart: () => void | Promise<void> = () => {};

  const badgeTone: Record<SimpleLocalAiStatusDto["status"], "neutral" | "ok" | "warn" | "danger"> = {
    not_running: "neutral",
    starting: "warn",
    ready: "ok",
    stopping: "warn",
    needs_attention: "danger"
  };

  const modelBadgeTone: Record<SimpleModelStatus, "neutral" | "ok" | "warn" | "danger"> = {
    unchecked: "neutral",
    ready: "ok",
    needs_attention: "danger"
  };

  const lastCheckedFormatter = new Intl.DateTimeFormat("vi-VN", {
    dateStyle: "short",
    timeStyle: "medium"
  });

  function formatLastCheckedAt(value?: string): string {
    if (!value) return "Chưa kiểm tra";

    const numeric = Number(value);
    const date = Number.isFinite(numeric)
      ? new Date(value.length <= 10 ? numeric * 1000 : numeric)
      : new Date(value);
    if (Number.isNaN(date.getTime())) return "Chưa kiểm tra";

    return lastCheckedFormatter.format(date);
  }
</script>

<section class="panel mx-auto flex w-full max-w-3xl flex-col items-center gap-6 px-4 py-8 text-center sm:gap-7 sm:px-8 sm:py-10">
  <button
    class="flex h-16 w-16 items-center justify-center rounded-lg border border-[#d5dce3] bg-[#1b2430] text-lg font-bold tracking-[0.12em] text-white transition hover:bg-[#263241]"
    aria-label="Logo ứng dụng"
    on:click={onLogoClick}
  >
    AI
  </button>

  <div class="w-full min-w-0 space-y-4">
    <StatusBadge tone={badgeTone[status.status]} text={formatSimpleStatusLabel(status.status)} />
    <div class="space-y-2">
      <h1 class="mx-auto max-w-2xl text-2xl font-semibold leading-tight text-[#1b2430] sm:text-[32px]">{status.title}</h1>
      <p class="mx-auto max-w-2xl text-sm leading-6 text-[#5e6a79]">{status.message}</p>
      <p class="text-xs font-medium uppercase tracking-[0.08em] text-[#7a8796]">
        Kiểm tra lần cuối: {formatLastCheckedAt(lastCheckedAt)}
      </p>
    </div>
  </div>

  {#if status.modelSummaries.length > 0}
    <section class="w-full rounded-lg border text-left" style="border-color: #d5dce3; background-color: #fcfcfd;">
      <div class="border-b px-4 py-3 sm:px-5" style="border-color: #d5dce3;">
        <h2 class="text-sm font-semibold text-[#1b2430]">Trạng thái mô hình</h2>
      </div>
      <ul class="divide-y" style="--tw-divide-opacity: 1; border-color: #d5dce3;">
        {#each status.modelSummaries as model}
          <li class="px-4 py-4 sm:px-5">
            <div class="min-w-0 space-y-1">
              <div class="flex min-w-0 flex-wrap items-center gap-2">
                <p class="min-w-0 break-words text-sm font-semibold text-[#1b2430]">{model.displayName}</p>
                <StatusBadge tone={modelBadgeTone[model.status]} text={formatSimpleModelStatusLabel(model.status)} />
              </div>
              <p class="text-xs font-medium uppercase tracking-[0.08em] text-[#7a8796]">
                Kiểm tra: {formatLastCheckedAt(model.lastCheckedAt)}
              </p>
            </div>
          </li>
        {/each}
      </ul>
    </section>
  {/if}

  <LocalAiControl
    canStart={status.canStart}
    canStop={status.canStop}
    canRestart={status.canRestart}
    {busy}
    {onRefresh}
    {onStart}
    {onStop}
    {onRestart}
  />

  {#if busy}
    <p class="text-sm font-medium text-[#5e6a79]">Vui lòng chờ...</p>
  {/if}

  <div class="w-full max-w-lg space-y-3">
    <SimpleErrorMessage message={notice} />
    {#if status.status === "needs_attention"}
      <button class="text-sm font-semibold text-[#5e6a79] underline-offset-4 hover:text-[#1b2430] hover:underline">
        Liên hệ bộ phận hỗ trợ
      </button>
    {/if}
  </div>
</section>

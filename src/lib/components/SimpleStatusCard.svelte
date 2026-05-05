<script lang="ts">
  import StatusBadge from "$lib/components/StatusBadge.svelte";
  import LocalAiControl from "$lib/components/LocalAiControl.svelte";
  import SimpleErrorMessage from "$lib/components/SimpleErrorMessage.svelte";
  import type { SimpleLocalAiStatusDto } from "$lib/types/local-ai";

  export let status: SimpleLocalAiStatusDto;
  export let busy = false;
  export let notice = "";
  export let onLogoClick: () => void | Promise<void> = () => {};
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
</script>

<section class="panel mx-auto flex w-full max-w-2xl flex-col items-center gap-8 px-8 py-12 text-center">
  <button
    class="flex h-20 w-20 items-center justify-center rounded-[28px] border border-slate-200 bg-slate-900 text-2xl font-black tracking-[0.2em] text-white"
    aria-label="App logo"
    on:click={onLogoClick}
  >
    AI
  </button>

  <div class="space-y-4">
    <StatusBadge tone={badgeTone[status.status]} text={status.status.replaceAll("_", " ")} />
    <div class="space-y-2">
      <h1 class="text-3xl font-semibold tracking-tight text-slate-950">{status.title}</h1>
      <p class="max-w-lg text-base leading-7 text-slate-600">{status.message}</p>
    </div>
  </div>

  <LocalAiControl
    canStart={status.canStart}
    canStop={status.canStop}
    canRestart={status.canRestart}
    {busy}
    {onStart}
    {onStop}
    {onRestart}
  />

  {#if busy}
    <p class="text-sm font-medium text-slate-500">Please wait...</p>
  {/if}

  <div class="w-full max-w-lg space-y-3">
    <SimpleErrorMessage message={notice} />
    {#if status.status === "needs_attention"}
      <button class="text-sm font-semibold text-slate-500 underline-offset-4 hover:text-slate-700 hover:underline">
        Contact support
      </button>
    {/if}
  </div>
</section>

<script lang="ts">
  export let canStart = false;
  export let canStop = false;
  export let canRestart = false;
  export let busy = false;
  export let onRefresh: () => void | Promise<void> = () => {};
  export let onStart: () => void | Promise<void> = () => {};
  export let onStop: () => void | Promise<void> = () => {};
  export let onRestart: () => void | Promise<void> = () => {};
</script>

<div class="flex w-full flex-col items-stretch gap-3 sm:w-auto sm:flex-row sm:items-center sm:justify-center">
  <button class="action-button-secondary min-w-36" disabled={busy} on:click={onRefresh}>Kiểm tra ngay</button>

  {#if canStart}
    <button class="action-button-primary min-w-36" disabled={busy} on:click={onStart}>Bắt đầu</button>
  {/if}

  {#if canStop}
    <button class="action-button-danger min-w-36" disabled={busy} on:click={onStop}>Dừng</button>
  {/if}

  {#if canRestart}
    <button class="action-button-secondary min-w-36" disabled={busy} on:click={onRestart}>Khởi động lại</button>
  {/if}
</div>

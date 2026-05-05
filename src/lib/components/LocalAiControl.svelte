<script lang="ts">
  export let canStart = false;
  export let canStop = false;
  export let canRestart = false;
  export let busy = false;
  export let onStart: () => void | Promise<void> = () => {};
  export let onStop: () => void | Promise<void> = () => {};
  export let onRestart: () => void | Promise<void> = () => {};
</script>

<div class="flex flex-col items-center gap-3 sm:flex-row sm:justify-center">
  {#if canStart}
    <button class="action-button-primary min-w-36" disabled={busy} on:click={onStart}>Start</button>
  {/if}

  {#if canStop}
    <button class="action-button-danger min-w-36" disabled={busy} on:click={onStop}>Stop</button>
  {/if}

  {#if canRestart}
    <button class="action-button-secondary min-w-36" disabled={busy} on:click={onRestart}>Restart</button>
  {/if}
</div>

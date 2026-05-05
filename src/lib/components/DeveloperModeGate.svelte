<script lang="ts">
  export let enabled = false;
</script>

{#if enabled}
  <slot />
{:else}
  <section class="panel p-8">
    <h2 class="text-lg font-semibold text-slate-950">Developer Mode is locked</h2>
    <p class="mt-2 text-sm leading-6 text-slate-600">
      Activate Developer Mode from the logo gesture to reveal engine details, logs, and diagnostics.
    </p>
  </section>
{/if}

<script lang="ts">
  export let enabled = false;
</script>

{#if enabled}
  <slot />
{/if}

<script lang="ts">
  export let message = "";
</script>

{#if message}
  <div class="rounded-2xl border border-amber-200 bg-amber-50 px-4 py-3 text-sm text-amber-800">
    {message}
  </div>
{/if}

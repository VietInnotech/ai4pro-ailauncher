<script lang="ts">
  export let message = "";
</script>

{#if message}
  <div class="rounded-md border px-4 py-3 text-sm text-[#8a6431]" style="border-color: #e7d6bb; background-color: #f8f2e9;">
    {message}
  </div>
{/if}

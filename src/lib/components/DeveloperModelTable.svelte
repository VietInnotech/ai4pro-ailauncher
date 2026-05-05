<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import StatusBadge from "$lib/components/StatusBadge.svelte";
  import type { DeveloperModelPackageDto } from "$lib/types/developer";

  export let models: DeveloperModelPackageDto[] = [];
  export let selectedId: string | null = null;

  const dispatch = createEventDispatcher<{ validate: string }>();
</script>

<section class="panel overflow-hidden">
  <div class="border-b border-slate-200 px-6 py-4">
    <h2 class="text-lg font-semibold text-slate-950">Model packages</h2>
  </div>
  <div class="overflow-x-auto">
    <table class="min-w-full divide-y divide-slate-200 text-sm">
      <thead class="bg-slate-50 text-left text-slate-500">
        <tr>
          <th class="px-4 py-3 font-semibold">ID</th>
          <th class="px-4 py-3 font-semibold">Kind</th>
          <th class="px-4 py-3 font-semibold">Display</th>
          <th class="px-4 py-3 font-semibold">Relative path</th>
          <th class="px-4 py-3 font-semibold">Resolved path</th>
          <th class="px-4 py-3 font-semibold">Installed</th>
          <th class="px-4 py-3 font-semibold">Verified</th>
          <th class="px-4 py-3 font-semibold">Action</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-200 bg-white">
        {#each models as model}
          <tr class={selectedId === model.id ? "bg-indigo-50/60" : ""}>
            <td class="px-4 py-3">
              <div class="font-semibold text-slate-900">{model.id}</div>
              <div class="text-xs text-slate-500">{model.internalName}</div>
            </td>
            <td class="px-4 py-3">{model.kind}</td>
            <td class="px-4 py-3">{model.displayName}</td>
            <td class="px-4 py-3 text-xs text-slate-500">{model.relativePath}</td>
            <td class="px-4 py-3 text-xs text-slate-500">{model.resolvedPath ?? "—"}</td>
            <td class="px-4 py-3">
              <StatusBadge tone={model.installed ? "ok" : "danger"} text={model.installed ? "installed" : "missing"} />
            </td>
            <td class="px-4 py-3">
              <StatusBadge tone={model.verified ? "ok" : "warn"} text={model.verified ? "verified" : "unverified"} />
            </td>
            <td class="px-4 py-3">
              <button class="action-button-secondary" on:click={() => dispatch("validate", model.id)}>Validate</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

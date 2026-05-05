<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { devValidateModelPackage } from "$lib/api/developer";
  import type { DeveloperModelPackageDto, ValidationResultDto } from "$lib/types/developer";
  import DeveloperModelTable from "$lib/components/DeveloperModelTable.svelte";

  export let models: DeveloperModelPackageDto[] = [];

  const dispatch = createEventDispatcher<{ reload: void }>();

  let validation: ValidationResultDto | null = null;
  let validationError = "";
  let selectedModelId: string | null = null;

  async function validateModel(id: string) {
    selectedModelId = id;
    validationError = "";
    try {
      validation = await devValidateModelPackage(id);
      dispatch("reload");
    } catch (error) {
      validationError = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<section class="space-y-4">
  <DeveloperModelTable {models} selectedId={selectedModelId} on:validate={(event) => validateModel(event.detail)} />

  <section class="panel p-6">
    <h3 class="text-base font-semibold text-slate-950">Model validation</h3>
    {#if validationError}
      <p class="mt-3 rounded-xl border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">{validationError}</p>
    {:else if validation}
      <div class="mt-3 space-y-3">
        <p class={`text-sm font-medium ${validation.valid ? "text-emerald-700" : "text-amber-700"}`}>
          {validation.valid ? "Model package is valid." : "Model package has issues."}
        </p>
        {#if validation.issues.length > 0}
          <ul class="space-y-2 text-sm text-slate-600">
            {#each validation.issues as issue}
              <li class="rounded-xl border border-slate-200 px-3 py-2">
                <span class="font-medium text-slate-900">{issue.code}</span>: {issue.message}
              </li>
            {/each}
          </ul>
        {:else}
          <p class="text-sm text-slate-500">All required files are present.</p>
        {/if}
      </div>
    {:else}
      <p class="mt-3 text-sm text-slate-500">Run a model validation to inspect the pinned file contract.</p>
    {/if}
  </section>
</section>

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
    <h3 class="text-base font-semibold text-[#1b2430]">Kiểm tra mô hình</h3>
    {#if validationError}
      <p class="mt-3 rounded-md border px-3 py-2 text-sm text-[#9a3d3d]" style="border-color: #efcaca; background-color: #fbebeb;">{validationError}</p>
    {:else if validation}
      <div class="mt-3 space-y-3">
        <p class={`text-sm font-medium ${validation.valid ? "text-[#2f6b57]" : "text-[#8a6431]"}`}>
          {validation.valid ? "Gói mô hình hợp lệ." : "Gói mô hình có vấn đề."}
        </p>
        {#if validation.issues.length > 0}
          <ul class="space-y-2 text-sm text-[#5e6a79]">
            {#each validation.issues as issue}
              <li class="rounded-md border px-3 py-2" style="border-color: #d5dce3;">
                <span class="font-medium text-[#1b2430]">{issue.code}</span>: {issue.message}
              </li>
            {/each}
          </ul>
        {:else}
          <p class="text-sm text-[#5e6a79]">Tất cả các tệp bắt buộc đều có mặt.</p>
        {/if}
      </div>
    {:else}
      <p class="mt-3 text-sm text-[#5e6a79]">Chạy kiểm tra mô hình để xem cấu trúc tệp bắt buộc đã ghim.</p>
    {/if}
  </section>
</section>

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import StatusBadge from "$lib/components/StatusBadge.svelte";
  import type { DeveloperModelPackageDto } from "$lib/types/developer";

  export let models: DeveloperModelPackageDto[] = [];
  export let selectedId: string | null = null;

  const dispatch = createEventDispatcher<{ validate: string }>();
</script>

<section class="panel overflow-hidden">
  <div class="border-b px-6 py-4" style="border-color: #d5dce3;">
    <h2 class="text-lg font-semibold text-[#1b2430]">Gói mô hình</h2>
  </div>
  <div class="overflow-x-auto">
    <table class="min-w-full divide-y text-sm" style="--tw-divide-opacity: 1; border-color: #d5dce3;">
      <thead class="text-left text-[#5e6a79]" style="background-color: #eef2f5;">
        <tr>
          <th class="px-4 py-3 font-semibold">ID</th>
          <th class="px-4 py-3 font-semibold">Loại</th>
          <th class="px-4 py-3 font-semibold">Tên hiển thị</th>
          <th class="px-4 py-3 font-semibold">Đường dẫn tương đối</th>
          <th class="px-4 py-3 font-semibold">Đường dẫn đã phân giải</th>
          <th class="px-4 py-3 font-semibold">Đã cài đặt</th>
          <th class="px-4 py-3 font-semibold">Đã xác minh</th>
          <th class="px-4 py-3 font-semibold">Thao tác</th>
        </tr>
      </thead>
      <tbody class="divide-y bg-white" style="--tw-divide-opacity: 1; border-color: #d5dce3;">
        {#each models as model}
          <tr class={selectedId === model.id ? "bg-[#edf2f6]" : ""}>
            <td class="px-4 py-3">
              <div class="font-semibold text-[#1b2430]">{model.id}</div>
              <div class="text-xs text-[#5e6a79]">{model.internalName}</div>
            </td>
            <td class="px-4 py-3">{model.kind}</td>
            <td class="px-4 py-3">{model.displayName}</td>
            <td class="px-4 py-3 text-xs text-[#5e6a79]">{model.relativePath}</td>
            <td class="px-4 py-3 text-xs text-[#5e6a79]">{model.resolvedPath ?? "Không có"}</td>
            <td class="px-4 py-3">
              <StatusBadge tone={model.installed ? "ok" : "danger"} text={model.installed ? "Đã cài" : "Thiếu"} />
            </td>
            <td class="px-4 py-3">
              <StatusBadge tone={model.verified ? "ok" : "warn"} text={model.verified ? "Đã xác minh" : "Chưa xác minh"} />
            </td>
            <td class="px-4 py-3">
              <button class="action-button-secondary" on:click={() => dispatch("validate", model.id)}>Kiểm tra</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

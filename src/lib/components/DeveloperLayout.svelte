<script lang="ts">
  export let active: string;

  const items = [
    { id: "dashboard", label: "Tổng quan" },
    { id: "engines", label: "Động cơ" },
    { id: "models", label: "Mô hình" },
    { id: "logs", label: "Nhật ký" },
    { id: "settings", label: "Cài đặt" },
    { id: "diagnostics", label: "Chẩn đoán" }
  ];
</script>

<div class="grid gap-5 lg:grid-cols-[220px_minmax(0,1fr)]">
  <aside class="panel p-4">
    <p class="px-3 pb-3 text-xs font-semibold uppercase tracking-[0.12em] text-[#5e6a79]">Chế độ nhà phát triển</p>
    <nav class="space-y-1">
      {#each items as item}
        <button
          class={`w-full rounded-md px-3 py-2 text-left text-sm font-medium transition ${active === item.id ? "bg-[#1b2430] text-white" : "text-[#5e6a79] hover:bg-[#eef2f5] hover:text-[#1b2430]"}`}
        >
          {item.label}
        </button>
      {/each}
    </nav>
  </aside>

  <main class="space-y-5">
    <slot />
  </main>
</div>

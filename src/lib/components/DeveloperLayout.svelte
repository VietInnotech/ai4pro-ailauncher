<script lang="ts">
  export let active: string;

  const items = [
    { id: "dashboard", label: "Dashboard" },
    { id: "engines", label: "Engines" },
    { id: "models", label: "Models" },
    { id: "logs", label: "Logs" },
    { id: "settings", label: "Settings" },
    { id: "diagnostics", label: "Diagnostics" }
  ];
</script>

<div class="grid gap-6 lg:grid-cols-[220px_minmax(0,1fr)]">
  <aside class="panel p-4">
    <p class="px-3 pb-3 text-xs font-semibold uppercase tracking-[0.18em] text-slate-500">Developer Mode</p>
    <nav class="space-y-1">
      {#each items as item}
        <button
          class={`w-full rounded-xl px-3 py-2 text-left text-sm font-medium transition ${active === item.id ? "bg-slate-900 text-white" : "text-slate-700 hover:bg-slate-100"}`}
        >
          {item.label}
        </button>
      {/each}
    </nav>
  </aside>

  <main class="space-y-6">
    <slot />
  </main>
</div>

<script lang="ts">
  export let tone: "neutral" | "ok" | "warn" | "danger" = "neutral";
  export let text = "";

  const classes = {
    neutral: "bg-slate-100 text-slate-700 border-slate-200",
    ok: "bg-emerald-50 text-emerald-700 border-emerald-200",
    warn: "bg-amber-50 text-amber-700 border-amber-200",
    danger: "bg-red-50 text-red-700 border-red-200"
  };
</script>

<span class={`inline-flex rounded-full border px-2.5 py-1 text-xs font-semibold ${classes[tone]}`}>
  {text}
</span>

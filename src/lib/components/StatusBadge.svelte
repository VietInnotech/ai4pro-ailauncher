<script lang="ts">
  export let tone: "neutral" | "ok" | "warn" | "danger" = "neutral";
  export let text = "";

  const classes = {
    neutral: "status-badge-neutral",
    ok: "status-badge-ok",
    warn: "status-badge-warn",
    danger: "status-badge-danger"
  };
</script>

<span class={`status-badge ${classes[tone]}`}>
  {text}
</span>

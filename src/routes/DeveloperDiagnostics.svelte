<script lang="ts">
  import type { DiagnosticsBundleDto } from "$lib/types/developer";
  import DeveloperDiagnosticsPanel from "$lib/components/DeveloperDiagnostics.svelte";

  export let diagnostics: DiagnosticsBundleDto | null = null;

  function exportBundle() {
    if (!diagnostics) return;
    const blob = new Blob([JSON.stringify(diagnostics, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "local-ai-diagnostics.json";
    link.click();
    URL.revokeObjectURL(url);
  }
</script>

<DeveloperDiagnosticsPanel bundle={diagnostics} on:export={exportBundle} />

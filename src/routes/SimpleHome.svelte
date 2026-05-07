<script lang="ts">
  import { enableDeveloperModeForSession } from "$lib/api/developer";
  import SimpleStatusCard from "$lib/components/SimpleStatusCard.svelte";
  import { developerMode, registerDeveloperModeClick } from "$lib/stores/developer-mode";
  import type { LocalAiStoreState } from "$lib/stores/local-ai";
  import { defaultSimpleStatus } from "$lib/types/local-ai";

  export let state: LocalAiStoreState = {
    status: defaultSimpleStatus,
    loading: false,
    lastCheckedAt: undefined
  };
  export let onRefresh: () => void | Promise<void> = () => {};
  export let onStart: () => void | Promise<void> = () => {};
  export let onStop: () => void | Promise<void> = () => {};
  export let onRestart: () => void | Promise<void> = () => {};

  async function handleLogoClick() {
    if (!registerDeveloperModeClick()) return;

    await enableDeveloperModeForSession();
    developerMode.set(true);
  }
</script>

<SimpleStatusCard
  status={state.status}
  busy={state.loading}
  notice={state.lastError ?? ""}
  lastCheckedAt={state.lastCheckedAt}
  onLogoClick={handleLogoClick}
  {onRefresh}
  {onStart}
  {onStop}
  {onRestart}
/>

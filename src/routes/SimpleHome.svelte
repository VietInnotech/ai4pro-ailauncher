<script lang="ts">
  import { onMount } from "svelte";
  import { enableDeveloperModeForSession } from "$lib/api/developer";
  import SimpleStatusCard from "$lib/components/SimpleStatusCard.svelte";
  import { developerMode, registerDeveloperModeClick } from "$lib/stores/developer-mode";
  import { localAiStore } from "$lib/stores/local-ai";
  import { defaultSimpleStatus, type SimpleLocalAiStatusDto } from "$lib/types/local-ai";

  let state: {
    status: SimpleLocalAiStatusDto;
    loading: boolean;
    lastError?: string;
  } = {
    status: defaultSimpleStatus,
    loading: false
  };

  const unsubscribe = localAiStore.subscribe((value: typeof state) => {
    state = value;
  });

  onMount(() => {
    void localAiStore.refresh();
    return () => unsubscribe();
  });

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
  onLogoClick={handleLogoClick}
  onStart={() => localAiStore.start()}
  onStop={() => localAiStore.stop()}
  onRestart={() => localAiStore.restart()}
/>

import { writable } from "svelte/store";
import {
  getSimpleLocalAiStatus,
  restartLocalAi,
  startLocalAi,
  stopLocalAi
} from "$lib/api/local-ai";
import { defaultSimpleStatus, type SimpleLocalAiStatusDto } from "$lib/types/local-ai";

function errorMessage(error: unknown): string {
  if (typeof error === "string") {
    try {
      const parsed = JSON.parse(error) as { message?: string };
      return parsed.message ?? error;
    } catch {
      return error;
    }
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "Unexpected error";
}

type LocalAiStoreState = {
  status: SimpleLocalAiStatusDto;
  loading: boolean;
  lastError?: string;
};

function createLocalAiStore() {
  const { subscribe, update, set } = writable<LocalAiStoreState>({
    status: defaultSimpleStatus,
    loading: false
  });

  async function run(action: () => Promise<SimpleLocalAiStatusDto>) {
    update((state) => ({ ...state, loading: true, lastError: undefined }));

    try {
      const status = await action();
      update(() => ({ status, loading: false }));
    } catch (error) {
      update((state) => ({
        ...state,
        loading: false,
        lastError: errorMessage(error)
      }));
    }
  }

  return {
    subscribe,
    reset() {
      set({ status: defaultSimpleStatus, loading: false });
    },
    refresh() {
      return run(getSimpleLocalAiStatus);
    },
    start() {
      return run(startLocalAi);
    },
    stop() {
      return run(stopLocalAi);
    },
    restart() {
      return run(restartLocalAi);
    }
  };
}

export const localAiStore = createLocalAiStore();

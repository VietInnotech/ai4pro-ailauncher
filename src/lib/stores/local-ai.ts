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

  return "Lỗi không mong đợi";
}

type LocalAiStoreState = {
  status: SimpleLocalAiStatusDto;
  loading: boolean;
  lastError?: string;
  lastCheckedAt?: string;
};

function createLocalAiStore() {
  const { subscribe, update, set } = writable<LocalAiStoreState>({
    status: defaultSimpleStatus,
    loading: false,
    lastCheckedAt: undefined
  });

  async function run(action: () => Promise<SimpleLocalAiStatusDto>) {
    update((state) => ({ ...state, loading: true, lastError: undefined }));

    try {
      const status = await action();
      update(() => ({
        status,
        loading: false,
        lastCheckedAt: new Date().toISOString(),
        lastError: undefined
      }));
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
      set({ status: defaultSimpleStatus, loading: false, lastCheckedAt: undefined });
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

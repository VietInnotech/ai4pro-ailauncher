import { get, writable } from "svelte/store";
import {
  checkSimpleModelStatus,
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

export type LocalAiStoreState = {
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
      const message = errorMessage(error);

      try {
        const status = await getSimpleLocalAiStatus();
        update(() => ({
          status,
          loading: false,
          lastCheckedAt: new Date().toISOString(),
          lastError: message
        }));
        return;
      } catch {
        // Keep the original action error visible. A failed recovery refresh should not replace it.
      }

      update((state) => ({
        ...state,
        loading: false,
        lastError: message
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
    checkAllModels() {
      return run(async () => {
        let status = await getSimpleLocalAiStatus();
        const current = get({ subscribe });
        const modelIds = current.status.modelSummaries.length > 0
          ? current.status.modelSummaries.map((model) => model.id)
          : status.modelSummaries.map((model) => model.id);

        for (const id of modelIds) {
          status = await checkSimpleModelStatus(id);
        }

        return status;
      });
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

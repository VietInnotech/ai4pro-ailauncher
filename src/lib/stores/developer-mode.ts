import { writable } from "svelte/store";

export const developerMode = writable(false);

let logoClickCount = 0;
let firstClickAt = 0;

export function resetDeveloperModeSequence(): void {
  logoClickCount = 0;
  firstClickAt = 0;
}

export function registerDeveloperModeClick(now = Date.now()): boolean {
  if (!firstClickAt || now - firstClickAt > 5_000) {
    firstClickAt = now;
    logoClickCount = 1;
    return false;
  }

  logoClickCount += 1;

  if (logoClickCount >= 7) {
    resetDeveloperModeSequence();
    return true;
  }

  return false;
}

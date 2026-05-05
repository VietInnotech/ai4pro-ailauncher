export type AppSettingsDto = {
  appDataRoot: string;
  machineConfigured: boolean;
  developerModePersisted: boolean;
  stopEnginesOnExit: boolean;
  autoStartLocalAi: boolean;
  simpleModeOnly: boolean;
  setupVersion: string;
};

export type UpdateAppSettingsDto = Partial<AppSettingsDto>;

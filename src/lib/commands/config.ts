import { invoke } from "@tauri-apps/api/core";
import type { AppConfig } from "$lib/types";

export async function loadConfig(): Promise<AppConfig> {
  return invoke<AppConfig>("load_config");
}

export async function saveConfig(config: AppConfig): Promise<void> {
  return invoke<void>("save_config", { config });
}

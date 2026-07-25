import { load } from "@tauri-apps/plugin-store";
import type { TrackedStation, AppSettings } from "./types";

const STORE_FILE = "benz_detector.json";

let _store: Awaited<ReturnType<typeof load>> | null = null;

async function getStore() {
  if (!_store) {
    _store = await load(STORE_FILE, { autoSave: true });
  }
  return _store;
}

export async function loadTrackedStations(): Promise<TrackedStation[]> {
  try {
    const store = await getStore();
    const val = await store.get<TrackedStation[]>("trackedStations");
    return val ?? [];
  } catch {
    return [];
  }
}

export async function saveTrackedStations(list: TrackedStation[]): Promise<void> {
  const store = await getStore();
  await store.set("trackedStations", list);
}

export async function loadSettings(): Promise<AppSettings> {
  try {
    const store = await getStore();
    const val = await store.get<AppSettings>("settings");
    return val ?? { devMode: false, monitorInterval: 5 };
  } catch {
    return { devMode: false, monitorInterval: 5 };
  }
}

export async function saveSettings(s: AppSettings): Promise<void> {
  const store = await getStore();
  await store.set("settings", s);
}

export interface LastLocation {
  lat: number;
  lon: number;
  zoom: number;
}

export async function loadLastLocation(): Promise<LastLocation | null> {
  try {
    const store = await getStore();
    return (await store.get<LastLocation>("lastLocation")) ?? null;
  } catch {
    return null;
  }
}

export async function saveLastLocation(loc: LastLocation): Promise<void> {
  const store = await getStore();
  await store.set("lastLocation", loc);
}

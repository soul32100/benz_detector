import {
  getCurrentPosition,
  watchPosition as wp,
  clearWatch as cw,
  type Position,
} from "@tauri-apps/plugin-geolocation";

export interface Coords {
  latitude: number;
  longitude: number;
  accuracy: number;
}

export async function getGpsPosition(): Promise<Coords> {
  const pos = await getCurrentPosition({
    enableHighAccuracy: true,
    timeout: 20000,
    maximumAge: 0,
  });
  return {
    latitude: pos.coords.latitude,
    longitude: pos.coords.longitude,
    accuracy: pos.coords.accuracy,
  };
}

export type WatchCallback = (coords: Coords | null, error?: string) => void;

export async function startWatching(cb: WatchCallback): Promise<number> {
  const id = await wp(
    { enableHighAccuracy: true, timeout: 30000, maximumAge: 5000 },
    (location: Position | null, error?: string) => {
      if (error) { cb(null, error); return; }
      if (!location) return;
      cb({
        latitude: location.coords.latitude,
        longitude: location.coords.longitude,
        accuracy: location.coords.accuracy,
      });
    },
  );
  return id;
}

export async function stopWatching(channelId: number): Promise<void> {
  await cw(channelId);
}

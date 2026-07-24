import { getCurrentPosition } from "@tauri-apps/plugin-geolocation";

export interface Coords {
  latitude: number;
  longitude: number;
}

export async function getGpsPosition(): Promise<Coords> {
  const position = await getCurrentPosition();
  return {
    latitude: position.coords.latitude,
    longitude: position.coords.longitude,
  };
}

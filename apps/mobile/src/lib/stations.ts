import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { Station, LocationDto, StationChangeEvent } from "./types";

export async function fetchStations(
  lat: number,
  lon: number,
  radiusKm: number,
): Promise<Station[]> {
  return invoke("fetch_stations", { lat, lon, radiusKm });
}

export async function searchCity(query: string): Promise<LocationDto> {
  return invoke("search_city", { query });
}

export async function startMonitor(
  stationIds: string[],
  intervalMinutes: number,
): Promise<void> {
  return invoke("start_monitor", {
    stationIds,
    intervalMinutes,
  });
}

export async function stopMonitor(): Promise<void> {
  return invoke("stop_monitor");
}

export function onStationChange(
  callback: (event: StationChangeEvent) => void,
): Promise<UnlistenFn> {
  return listen<StationChangeEvent>("station-change", (e) => {
    callback(e.payload);
  });
}

export function onMonitorError(
  callback: (error: string) => void,
): Promise<UnlistenFn> {
  return listen<string>("monitor-error", (e) => {
    callback(e.payload);
  });
}

export function fuelLabel(fuelType: string): string {
  const map: Record<string, string> = {
    Ai92: "92",
    Ai95: "95",
    Ai95Puls: "95+",
    Diesel: "ДТ",
    Ai98: "98",
    Ai100: "100",
    Gas: "Газ",
    Unknown: "?",
  };
  return map[fuelType] ?? fuelType;
}

export function fuelIcon(status: string): string {
  switch (status) {
    case "Available":
      return "✅";
    case "Low":
      return "🟡";
    case "Unavailable":
      return "❌";
    default:
      return "❓";
  }
}

export function fuelCell(status: string, price: number | null): string {
  const icon = fuelIcon(status);
  if (price !== null) {
    return `${icon} ${price.toFixed(2)}`;
  }
  return icon;
}

export function statusDisplay(
  tags: { tag: StationTag }[],
  overallStatus: string,
): string {
  const labels = tags.slice(0, 3).map((t) => tagLabel(t.tag));
  if (labels.length > 0) {
    return labels.join(" ");
  }
  switch (overallStatus) {
    case "Works":
      return "✅";
    case "NotWorking":
      return "🚫";
    default:
      return "—";
  }
}

export function tagLabel(tag: StationTag): string {
  const key = Object.keys(tag)[0] as keyof StationTag;
  switch (key) {
    case "CardsOnly":
      return "🔒карты";
    case "CashAndCards":
      return "💳нал/карты";
    case "FuelLimit":
      return `⛽${(tag as any).FuelLimit}л`;
    case "NoLimit":
      return "⛽без лим";
    case "Closed":
      return "🚫закрыта";
    case "CanisterOk":
      return "📦кан";
    case "TankOnly":
      return "🚧только бак";
    case "EvenOdd":
      return "2/4";
    case "BigQueue":
      return "📶!!";
    case "Queue20Plus":
      return "📶20+";
    case "Queue50Plus":
      return "📶50+";
    default:
      return key;
  }
}

export function stationName(s: Station): string {
  const brandKey = Object.keys(s.brand)[0];
  if (brandKey === "Unknown") {
    const val = (s.brand as any).Unknown;
    return val || "—";
  }
  const brandName = brandKey;
  return s.name || brandName;
}

export interface Comment {
  text: string;
  created_at: string | null;
  provider: string;
}

export interface Station {
  id: string;
  osm_id: string;
  name: string;
  brand: Brand;
  address: string;
  latitude: number | null;
  longitude: number | null;
  fuels: FuelAvailability[];
  tags: StationTagInfo[];
  overall_status: string;
  last_updated: string | null;
  reports_24h: number | null;
  comments: Comment[];
}

export type Brand =
  | { Lukoil: null }
  | { Gazprom: null }
  | { Rosneft: null }
  | { Bashneft: null }
  | { Tatneft: null }
  | { Unknown: string };

export interface FuelAvailability {
  fuel_type: string;
  status: string;
  provider: string;
  checked_at: string;
  price: number | null;
}

export interface StationTagInfo {
  tag: StationTag;
  source: string;
  updated_at: string;
}

export type StationTag =
  | { CardsOnly: null }
  | { CashAndCards: null }
  | { FuelLimit: number }
  | { NoLimit: null }
  | { Closed: null }
  | { CanisterOk: null }
  | { TankOnly: null }
  | { EvenOdd: null }
  | { BigQueue: null }
  | { Queue20Plus: null }
  | { Queue50Plus: null };

export interface LocationDto {
  latitude: number;
  longitude: number;
  city: string | null;
  region: string | null;
}

export interface StationChangeEvent {
  station_id: string;
  station_name: string;
  station_address: string;
  fuel_changes: FuelChangeEvent[];
  tag_changes: TagChangeEvent[];
  timestamp: string;
}

export interface FuelChangeEvent {
  fuel_type: string;
  change_type: string;
  old_status: string | null;
  new_status: string;
  old_price: number | null;
  new_price: number | null;
}

export interface TagChangeEvent {
  tag: string;
  change_type: string;
}

export interface TrackedStation {
  id: string;
  name: string;
  address: string;
}

export type Page = "home" | "stations" | "detail" | "monitor" | "settings";

export interface AppSettings {
  devMode: boolean;
  monitorInterval: number;
}

<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import MapView from "./MapView.svelte";
  import { getGpsPosition, startWatching, stopWatching } from "../lib/location";
  import { fetchStations, searchCity, reverseGeocode, stationName, fuelLabel, fuelCell } from "../lib/stations";
  import { loadLastLocation, saveLastLocation } from "../lib/store";
  import type { Station } from "../lib/types";

  export let onOpenDetail: (s: Station) => void;
  export let onOpenMonitor: () => void;
  export let onOpenSettings: () => void;
  export let devMode = false;

  let lat = 55.75;
  let lon = 37.62;
  let zoom = 11;
  let userLat: number | null = null;
  let userLon: number | null = null;
  let cityName = "Москва";
  let radius = 30;
  let stations: Station[] = [];
  let loading = false;
  let error = "";
  let searchQuery = "";
  let searching = false;
  let gpsIcon = "📍";
  let gpsStatus = "нажмите для GPS";
  let watchId: number | null = null;
  let mapComponent: MapView;
  let gpsRequesting = false;
  let gpsWorking = false;
  let lastUserDrag = Date.now();
  let returnTimer: ReturnType<typeof setInterval> | null = null;
  let mapLat = lat;
  let mapLon = lon;

  onMount(async () => {
    const last = await loadLastLocation();
    if (last) {
      lat = last.lat;
      lon = last.lon;
      zoom = last.zoom;
      mapLat = last.lat;
      mapLon = last.lon;
    }
    findStations();
    returnTimer = setInterval(checkReturnToUser, 10000);
  });

  onDestroy(() => {
    if (watchId !== null) stopWatching(watchId);
    if (returnTimer !== null) clearInterval(returnTimer);
  });

  function persistLocation() {
    saveLastLocation({ lat: mapLat, lon: mapLon, zoom });
  }

  function checkReturnToUser() {
    if (!gpsWorking || !userLat || !userLon) return;
    const idle = (Date.now() - lastUserDrag) / 1000;
    if (idle < 55) return;
    const dx = mapLat - userLat;
    const dy = mapLon - userLon;
    if (Math.abs(dx) < 0.01 && Math.abs(dy) < 0.01) return;
    mapComponent?.flyTo(userLat, userLon, 13);
  }

  async function requestGps() {
    if (gpsRequesting) return;
    gpsRequesting = true;
    gpsIcon = "⏳";
    gpsStatus = "запрос…";

    try {
      const pos = await getGpsPosition();
      userLat = pos.latitude;
      userLon = pos.longitude;
      lat = userLat;
      lon = userLon;
      gpsIcon = "🟢";
      gpsStatus = `${lat.toFixed(4)}, ${lon.toFixed(4)}`;
      gpsWorking = true;
      lastUserDrag = Date.now();
      mapComponent?.flyTo(lat, lon, 13);

      reverseGeocode(lat, lon).then((city) => {
        if (city) { cityName = city; searchQuery = city; }
      }).catch(() => {});

      watchId = await startWatching((coords, err) => {
        if (err) { gpsIcon = "⚠️"; gpsStatus = `ошибка: ${err}`; return; }
        if (coords) {
          userLat = coords.latitude;
          userLon = coords.longitude;
          lat = coords.latitude;
          lon = coords.longitude;
          gpsIcon = "🟢";
          gpsStatus = `${lat.toFixed(4)}, ${lon.toFixed(4)}`;
        }
      });
    } catch (e) {
      gpsIcon = "⚠️";
      gpsStatus = `ошибка: ${e}`;
    }
    gpsRequesting = false;
  }

  let fetchTimer: ReturnType<typeof setTimeout> | null = null;

  function onMapMove(newLat: number, newLon: number) {
    mapLat = newLat;
    mapLon = newLon;
    persistLocation();
    if (fetchTimer) clearTimeout(fetchTimer);
    fetchTimer = setTimeout(() => {
      lat = mapLat;
      lon = mapLon;
      reverseGeocode(mapLat, mapLon).then((city) => {
        if (city) { cityName = city; searchQuery = city; }
      }).catch(() => {});
      findStations();
    }, 2000);
  }

  function onUserDrag(newLat: number, newLon: number) {
    lastUserDrag = Date.now();
  }

  async function findStations() {
    loading = true;
    error = "";
    try {
      const result = await fetchStations(lat, lon, radius);
      stations = result;
      if (result.length === 0) error = "Нет заправок в этом радиусе";
    } catch (e) {
      error = `Ошибка: ${e}`;
    }
    loading = false;
  }

  async function handleSearch() {
    if (!searchQuery.trim()) return;
    searching = true;
    error = "";
    try {
      const loc = await searchCity(searchQuery.trim());
      lat = loc.latitude;
      lon = loc.longitude;
      cityName = loc.city ?? searchQuery;
      zoom = 12;
      persistLocation();
      await findStations();
    } catch (e) {
      error = `Ошибка поиска: ${e}`;
    }
    searching = false;
  }
</script>

<div class="home">
  <div class="map-wrapper">
    <MapView
      bind:this={mapComponent}
      {stations}
      centerLat={lat}
      centerLon={lon}
      {zoom}
      onStationClick={onOpenDetail}
      onMapMove={onMapMove}
      onUserDrag={onUserDrag}
      {userLat}
      {userLon}
    />
  </div>

  <div class="top-bar">
    <input
      class="search-input"
      type="text"
      placeholder="🔍 Город…"
      bind:value={searchQuery}
      on:keydown={(e) => e.key === "Enter" && handleSearch()}
      disabled={searching}
    />
    <button class="icon-btn" on:click={handleSearch} disabled={searching}>
      {searching ? "⏳" : "🔍"}
    </button>
  </div>

  <div class="controls-right">
    <button class="icon-btn gps-btn" on:click={requestGps} title={gpsStatus}>
      {gpsIcon}
    </button>
    <button class="icon-btn" on:click={findStations} disabled={loading}>🔄</button>
    <button class="icon-btn" on:click={onOpenMonitor}>📡</button>
    <button class="icon-btn" on:click={onOpenSettings}>⚙️</button>
  </div>

  <div class="city-label">{cityName} · {radius} км</div>

  <div class="bottom-panel" class:expanded={stations.length > 0 || !!error}>
    <div class="panel-header">
      <div class="panel-handle"></div>
      <div class="flex-row" style="justify-content: space-between; width: 100%; padding: 0 4px;">
        <span style="font-weight: 600; font-size: 14px;">
          {loading ? "Загрузка…" : stations.length > 0 ? `АЗС: ${stations.length}` : "Нет заправок"}
        </span>
        <div class="flex-row gap-8">
          <span class="text-sm text-secondary">{gpsStatus}</span>
          <input type="range" min="5" max="100" step="5" bind:value={radius} style="width: 60px;" />
        </div>
      </div>
    </div>

    <div class="station-list">
      {#if error}
        <div class="error-msg">{error}</div>
      {/if}
      {#each stations as s (s.id)}
        <div class="station-card" role="button" tabindex="0"
          on:click={() => onOpenDetail(s)}
          on:keydown={(e) => e.key === "Enter" && onOpenDetail(s)}>
          <div class="flex-row" style="justify-content: space-between;">
            <div>
              <div class="card-title">{stationName(s)}</div>
              <div class="card-addr">{s.address || "—"}</div>
            </div>
            <div class="fuel-mini">
              {#each s.fuels.slice(0, 3) as f}
                <span class="fuel-dot">{fuelLabel(f.fuel_type)} {fuelCell(f.status, f.price)}</span>
              {/each}
            </div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .home { width: 100%; height: 100%; position: relative; overflow: hidden; }
  .map-wrapper { position: absolute; inset: 0; }
  .top-bar {
    position: absolute; top: calc(8px + env(safe-area-inset-top, 0px));
    left: 8px; right: 8px; display: flex; gap: 8px; z-index: 10;
  }
  .search-input {
    flex: 1; background: rgba(30, 30, 30, 0.95); color: #e0e0e0;
    border: 1px solid #444; padding: 12px 16px; border-radius: 12px;
    font-size: 16px; backdrop-filter: blur(10px);
  }
  .search-input:focus { border-color: #4caf50; outline: none; }
  .controls-right {
    position: absolute; top: calc(70px + env(safe-area-inset-top, 0px));
    right: 8px; display: flex; flex-direction: column; gap: 8px; z-index: 10;
  }
  .icon-btn {
    width: 44px; height: 44px; border-radius: 50%;
    background: rgba(30, 30, 30, 0.92); border: 1px solid #444;
    color: #e0e0e0; font-size: 20px; display: flex;
    align-items: center; justify-content: center; backdrop-filter: blur(10px);
  }
  .gps-btn { font-size: 22px; }
  .city-label {
    position: absolute; top: calc(64px + env(safe-area-inset-top, 0px));
    left: 50%; transform: translateX(-50%); z-index: 10;
    background: rgba(18, 18, 18, 0.85); border-radius: 8px;
    padding: 4px 12px; font-size: 13px; color: #e0e0e0;
    backdrop-filter: blur(10px); white-space: nowrap;
  }
  .bottom-panel {
    position: absolute; bottom: 0; left: 0; right: 0;
    max-height: 45vh;
    background: rgba(18, 18, 18, 0.95); backdrop-filter: blur(12px);
    border-radius: 16px 16px 0 0; border-top: 1px solid #333;
    display: flex; flex-direction: column; z-index: 10;
    transition: max-height 0.3s ease;
  }
  .bottom-panel:not(.expanded) { max-height: 60px; }
  .panel-header { padding: 8px 16px; display: flex; flex-direction: column; align-items: center; gap: 4px; cursor: pointer; }
  .panel-handle { width: 36px; height: 4px; border-radius: 2px; background: #555; margin-bottom: 4px; }
  .station-list { flex: 1; overflow-y: auto; padding: 0 12px 16px; }
  .station-card {
    background: #1e1e1e; border-radius: 12px; padding: 12px;
    margin-bottom: 8px; border: 1px solid #2a2a2a; cursor: pointer;
  }
  .card-title { font-size: 15px; font-weight: 600; margin-bottom: 2px; }
  .card-addr { font-size: 12px; color: #a0a0a0; }
  .fuel-mini { display: flex; gap: 4px; flex-wrap: wrap; align-items: flex-end; }
  .fuel-dot { background: #2a2a2a; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 500; }
  .error-msg { color: #ef5350; font-size: 13px; padding: 8px; text-align: center; }
  .text-sm { font-size: 13px; }
  .text-secondary { color: #a0a0a0; }
  .flex-row { display: flex; gap: 10px; align-items: center; }
  .gap-8 { gap: 8px; }
</style>

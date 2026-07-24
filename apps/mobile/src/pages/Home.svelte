<script lang="ts">
  import { onMount } from "svelte";
  import { getGpsPosition } from "../lib/location";
  import { fetchStations, searchCity } from "../lib/stations";
  import type { Page, Station } from "../lib/types";

  export let navigate: (p: Page, data?: Station[]) => void;

  let lat = 55.75;
  let lon = 37.62;
  let cityName = "определение…";
  let radius = 30;
  let loading = false;
  let error = "";
  let searchQuery = "";
  let searching = false;

  onMount(async () => {
    try {
      const pos = await getGpsPosition();
      lat = pos.latitude;
      lon = pos.longitude;
      cityName = `${lat.toFixed(4)}, ${lon.toFixed(4)}`;
    } catch {
      cityName = "Москва (GPS недоступен)";
    }
  });

  async function handleSearch() {
    if (!searchQuery.trim()) return;
    searching = true;
    error = "";
    try {
      const loc = await searchCity(searchQuery.trim());
      lat = loc.latitude;
      lon = loc.longitude;
      cityName = loc.city ?? searchQuery;
    } catch (e) {
      error = `Ошибка поиска: ${e}`;
    }
    searching = false;
  }

  async function handleFind() {
    loading = true;
    error = "";
    try {
      const result = await fetchStations(lat, lon, radius);
      if (result.length === 0) {
        error = "В этом радиусе заправок не найдено";
        loading = false;
        return;
      }
      navigate("stations", result);
    } catch (e) {
      error = `Ошибка загрузки: ${e}`;
    }
    loading = false;
  }
</script>

<div class="page">
  <h1 class="page-title">BenzDetector</h1>

  <div class="card">
    <div class="label">📍 Местоположение</div>
    <p class="text-sm text-secondary mt-8">{cityName}</p>
    <p class="text-sm text-secondary">{lat.toFixed(4)}, {lon.toFixed(4)}</p>
  </div>

  <div class="card">
    <div class="label">🔍 Поиск города</div>
    <div class="flex-row mt-8">
      <input
        class="input flex-1"
        type="text"
        placeholder="Название города…"
        bind:value={searchQuery}
        on:keydown={(e) => e.key === "Enter" && handleSearch()}
      />
      <button class="btn btn-sm" on:click={handleSearch} disabled={searching}>
        {searching ? "…" : "Найти"}
      </button>
    </div>
  </div>

  <div class="card">
    <div class="label">📏 Радиус поиска: {radius} км</div>
    <input
      class="slider"
      type="range"
      min="5"
      max="200"
      step="5"
      bind:value={radius}
    />
    <div class="flex-row" style="justify-content: space-between;">
      <span class="text-sm text-secondary">5 км</span>
      <span class="text-sm text-secondary">200 км</span>
    </div>
  </div>

  {#if error}
    <div class="card" style="border-color: var(--danger);">
      <p class="text-sm" style="color: var(--danger);">{error}</p>
    </div>
  {/if}

  <button class="btn mt-16" on:click={handleFind} disabled={loading}>
    {loading ? "Загрузка…" : `Найти АЗС в радиусе ${radius} км`}
  </button>
</div>

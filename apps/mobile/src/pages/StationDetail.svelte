<script lang="ts">
  import {
    fuelLabel,
    fuelCell,
    statusDisplay,
    stationName,
    tagLabel,
  } from "../lib/stations";
  import type { Page, Station } from "../lib/types";

  export let navigate: (p: Page, data?: Station[]) => void;
  export let station: Station;
</script>

<div class="page">
  <button class="btn btn-sm btn-secondary" on:click={() => navigate("stations")}>
    ← Назад
  </button>

  <h1 class="page-title mt-8">{stationName(station)}</h1>

  <div class="card">
    <div class="label">📍 Адрес</div>
    <p>{station.address || "—"}</p>
    <p class="text-sm text-secondary mt-8">
      {station.latitude?.toFixed(6)}, {station.longitude?.toFixed(6)}
    </p>
    {#if station.reports_24h !== null && station.reports_24h !== undefined}
      <p class="text-sm text-secondary mt-8">
        📊 {station.reports_24h} отметок за 24ч
      </p>
    {/if}
  </div>

  <div class="card">
    <div class="label">⛽ Топливо</div>
    {#each station.fuels as f}
      <div class="flex-row mt-8" style="justify-content: space-between;">
        <span style="font-weight: 600;">{fuelLabel(f.fuel_type)}</span>
        <span>{fuelCell(f.status, f.price)}</span>
      </div>
      <div class="text-sm text-secondary">
        {f.provider} · {new Date(f.checked_at).toLocaleString("ru-RU")}
      </div>
    {/each}
  </div>

  {#if station.tags.length > 0}
    <div class="card">
      <div class="label">🏷️ Теги</div>
      <div class="tag-row mt-8">
        {#each station.tags as t}
          <span class="tag-chip">{tagLabel(t.tag)}</span>
        {/each}
      </div>
    </div>
  {/if}

  <div class="card" style="border-color: var(--accent-dim);">
    <div class="label">📡 Статус</div>
    <p>{statusDisplay(station.tags, station.overall_status)}</p>
    {#if station.last_updated}
      <p class="text-sm text-secondary mt-8">
        Обновлено: {new Date(station.last_updated).toLocaleString("ru-RU")}
      </p>
    {/if}
  </div>
</div>

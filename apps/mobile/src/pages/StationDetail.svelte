<script lang="ts">
  import {
    fuelLabel,
    fuelCell,
    statusDisplay,
    stationName,
    tagLabel,
    providerIcon,
  } from "../lib/stations";
  import type { Station } from "../lib/types";

  export let onClose: () => void;
  export let station: Station;
  export let tracked = false;
  export let onToggleTrack: () => void;
  export let devMode = false;

  $: sortedComments = [...station.comments].sort((a, b) => {
    if (!a.created_at && !b.created_at) return 0;
    if (!a.created_at) return 1;
    if (!b.created_at) return -1;
    return b.created_at.localeCompare(a.created_at);
  });

  function openInMaps() {
    if (station.latitude && station.longitude) {
      const url = `https://2gis.ru/geo/${station.longitude},${station.latitude}`;
      window.open(url, "_blank");
    }
  }
</script>

<div class="page">
  <div class="flex-row" style="justify-content: space-between; align-items: center;">
    <button class="btn btn-sm btn-secondary" on:click={onClose}>
      ← Назад
    </button>
    <div class="flex-row gap-8">
      <button class="btn btn-sm btn-secondary" on:click={openInMaps} disabled={!station.latitude || !station.longitude}>
        📍 На карте
      </button>
      <button class="btn btn-sm {tracked ? 'btn-danger' : ''}" on:click={onToggleTrack}>
        {tracked ? "📡 Отписаться" : "📡 Отслеживать"}
      </button>
    </div>
  </div>

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
      {#if devMode}
        <div class="text-sm text-secondary">
          {providerIcon(f.provider)} {f.provider} · {new Date(f.checked_at).toLocaleString("ru-RU")}
        </div>
      {/if}
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

  {#if station.comments.length > 0}
    <div class="card">
      <div class="label">💬 Комментарии ({station.comments.length})</div>
      {#each sortedComments.slice(0, 10) as c}
        <div class="comment-item">
          <div class="comment-text">{c.text}</div>
          <div class="flex-row" style="justify-content: space-between;">
            <span class="text-sm text-secondary">
              {c.created_at ? new Date(c.created_at).toLocaleString("ru-RU") : "—"}
            </span>
            {#if devMode}
              <span class="text-sm text-secondary">
                {providerIcon(c.provider)} {c.provider}
              </span>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .comment-item {
    background: var(--surface2);
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    margin-bottom: 6px;
  }
  .comment-text {
    font-size: 14px;
    margin-bottom: 4px;
    line-height: 1.4;
  }
</style>

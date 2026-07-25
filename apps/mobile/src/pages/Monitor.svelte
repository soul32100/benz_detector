<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    startMonitor,
    stopMonitor,
    onStationChange,
    onMonitorError,
  } from "../lib/stations";
  import type { StationChangeEvent, TrackedStation } from "../lib/types";

  export let onClose: () => void;
  export let trackedStations: TrackedStation[] = [];
  export let onRemoveTrack: (id: string) => void;

  let monitoring = false;
  let interval = 5;
  let changes: StationChangeEvent[] = [];
  let errorMsg = "";
  let unlistenChange: (() => void) | null = null;
  let unlistenError: (() => void) | null = null;

  onMount(async () => {
    unlistenChange = await onStationChange((event) => {
      changes = [event, ...changes].slice(0, 100);
    });
    unlistenError = await onMonitorError((msg) => {
      errorMsg = msg;
    });
  });

  onDestroy(() => {
    stop();
    unlistenChange?.();
    unlistenError?.();
  });

  async function toggleMonitor() {
    if (monitoring) {
      await stop();
    } else {
      await start();
    }
  }

  async function start() {
    errorMsg = "";
    try {
      await startMonitor(trackedStations.map((t) => t.id), interval);
      monitoring = true;
    } catch (e) {
      errorMsg = `${e}`;
    }
  }

  async function stop() {
    try {
      await stopMonitor();
    } catch {}
    monitoring = false;
  }

  function changeSummary(event: StationChangeEvent): string {
    const parts: string[] = [];
    for (const f of event.fuel_changes) {
      switch (f.change_type) {
        case "appeared":
          parts.push(`✅ ${f.fuel_type} ${f.new_price?.toFixed(2) ?? ""}₽`);
          break;
        case "disappeared":
          parts.push(`❌ ${f.fuel_type}`);
          break;
        case "price_changed":
          parts.push(`💰 ${f.fuel_type}: ${f.old_price?.toFixed(2)}→${f.new_price?.toFixed(2)}₽`);
          break;
        default:
          parts.push(`${f.fuel_type}: ${f.old_status ?? "?"}→${f.new_status}`);
      }
    }
    for (const t of event.tag_changes) {
      if (t.change_type === "added") {
        parts.push(`🏷️ +${t.tag}`);
      } else {
        parts.push(`🏷️ -${t.tag}`);
      }
    }
    return parts.join("; ") || "нет изменений";
  }
</script>

<div class="page">
  <div class="flex-row" style="justify-content: space-between; align-items: center;">
    <h1 class="page-title">📡 Мониторинг</h1>
    <button class="btn btn-sm btn-secondary" on:click={onClose}>
      ← Назад
    </button>
  </div>

  <div class="card">
    <div class="label">Отслеживается станций: {trackedStations.length}</div>
    <div class="mt-8">
      {#each trackedStations as ts}
        <div class="flex-row" style="justify-content: space-between; padding: 4px 0;">
          <div>
            <div style="font-size: 14px; font-weight: 500;">{ts.name}</div>
            <div class="text-sm text-secondary">{ts.address}</div>
          </div>
          <button class="btn btn-sm btn-danger" on:click={() => onRemoveTrack(ts.id)}>
            🗑️
          </button>
        </div>
      {/each}
    </div>
  </div>

  {#if !monitoring}
    <div class="card">
      <div class="label">Интервал проверки (мин)</div>
      <input
        class="slider"
        type="range"
        min="1"
        max="30"
        step="1"
        bind:value={interval}
      />
      <div class="flex-row" style="justify-content: space-between;">
        <span class="text-sm text-secondary">1</span>
        <span class="text-sm text-secondary">{interval}</span>
        <span class="text-sm text-secondary">30</span>
      </div>
    </div>
  {/if}

  {#if errorMsg}
    <div class="card" style="border-color: var(--danger);">
      <p class="text-sm" style="color: var(--danger);">{errorMsg}</p>
    </div>
  {/if}

  <button
    class="btn mt-8 {monitoring ? 'btn-danger' : ''}"
    on:click={toggleMonitor}
  >
    {monitoring ? "⏹ Остановить мониторинг" : "▶️ Запустить мониторинг"}
  </button>

  {#if monitoring}
    <div class="card mt-8" style="border-color: var(--accent-dim);">
      <p class="text-sm" style="color: var(--accent);">🟢 Мониторинг активен</p>
    </div>
  {/if}

  <div class="mt-8" style="flex: 1; overflow-y: auto;">
    <div class="label">Лента изменений</div>

    {#if changes.length === 0}
      <p class="text-sm text-secondary mt-8">Пока нет изменений</p>
    {/if}

    {#each changes as c}
      <div class="card" style="padding: 10px;">
        <div class="flex-row" style="justify-content: space-between;">
          <span style="font-weight: 600; font-size: 14px;">{c.station_name}</span>
          <span class="text-sm text-secondary">{c.timestamp}</span>
        </div>
        <p class="text-sm mt-8">{changeSummary(c)}</p>
      </div>
    {/each}
  </div>
</div>

<script lang="ts">
  import { fuelLabel, fuelCell, statusDisplay, stationName } from "../lib/stations";
  import type { Page, Station } from "../lib/types";

  export let navigate: (p: Page, data?: Station[] | string[]) => void;
  export let stations: Station[] = [];

  let selected = new Set<string>();

  function toggle(id: string) {
    if (selected.has(id)) {
      selected.delete(id);
    } else {
      selected.add(id);
    }
    selected = selected;
  }

  function goDetail(s: Station) {
    navigate("detail", [{ station: s } as any]);
  }

  function goMonitor() {
    if (selected.size === 0) return;
    navigate("monitor", Array.from(selected));
  }

  function selectAll() {
    if (selected.size === stations.length) {
      selected.clear();
    } else {
      selected = new Set(stations.map((s) => s.id));
    }
    selected = selected;
  }
</script>

<div class="page">
  <div class="flex-row" style="justify-content: space-between; align-items: center;">
    <h1 class="page-title">АЗС</h1>
    <button class="btn btn-sm btn-secondary" on:click={() => navigate("home")}>
      ← Назад
    </button>
  </div>

  <div class="flex-row gap-8 mt-8">
    <button class="btn btn-sm btn-secondary" on:click={selectAll}>
      {selected.size === stations.length ? "Снять все" : "Выбрать все"}
    </button>
    <span class="text-sm text-secondary" style="line-height: 36px;">
      {stations.length} шт., выбрано {selected.size}
    </span>
  </div>

  <div class="mt-8" style="flex: 1; overflow-y: auto;">
    {#each stations as s (s.id)}
      <div class="card" role="button" tabindex="0" on:click={() => goDetail(s)} on:keydown={(e) => e.key === 'Enter' && goDetail(s)}>
        <div class="flex-row" style="justify-content: space-between;">
          <div>
            <div class="card-title">{stationName(s)}</div>
            <div class="card-addr">{s.address || "—"}</div>
          </div>
          <button
            class="btn btn-sm {selected.has(s.id) ? 'btn-danger' : 'btn-secondary'}"
            on:click|stopPropagation={() => toggle(s.id)}
          >
            {selected.has(s.id) ? "✓" : "+"}
          </button>
        </div>

        <div class="fuel-row">
          {#each s.fuels as f}
            <span class="fuel-chip">
              {fuelLabel(f.fuel_type)} {fuelCell(f.status, f.price)}
            </span>
          {/each}
        </div>

        {#if s.tags.length > 0}
          <div class="tag-row">
            {#each s.tags.slice(0, 4) as t}
              <span class="tag-chip">{statusDisplay([t], s.overall_status)}</span>
            {/each}
          </div>
        {/if}

        {#if s.reports_24h !== null && s.reports_24h !== undefined}
          <div class="text-sm text-secondary mt-8">📊 {s.reports_24h} отм. за 24ч</div>
        {/if}
      </div>
    {/each}
  </div>

  {#if selected.size > 0}
    <button class="btn mt-8" on:click={goMonitor}>
      📡 Отслеживать выбранные ({selected.size})
    </button>
  {/if}
</div>

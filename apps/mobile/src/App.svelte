<script lang="ts">
  import { onMount } from "svelte";
  import type { Station, TrackedStation, AppSettings } from "./lib/types";
  import { loadTrackedStations, saveTrackedStations, loadSettings } from "./lib/store";
  import Home from "./pages/Home.svelte";
  import StationDetail from "./pages/StationDetail.svelte";
  import Monitor from "./pages/Monitor.svelte";
  import Settings from "./pages/Settings.svelte";

  let trackedStations: TrackedStation[] = [];
  let devMode = false;
  let overlay: "detail" | "monitor" | "settings" | null = null;
  let detailStation: Station | null = null;

  onMount(async () => {
    trackedStations = await loadTrackedStations();
    const s = await loadSettings();
    devMode = s.devMode;
  });

  function saveTracked() {
    saveTrackedStations(trackedStations);
  }

  function toggleTrack(s: Station) {
    const idx = trackedStations.findIndex((t) => t.id === s.id);
    if (idx >= 0) {
      trackedStations = trackedStations.filter((t) => t.id !== s.id);
    } else {
      trackedStations = [
        ...trackedStations,
        { id: s.id, name: s.name || s.address || s.id, address: s.address || "" },
      ];
    }
    saveTracked();
  }

  function isTracked(id: string): boolean {
    return trackedStations.some((t) => t.id === id);
  }

  function removeTrack(id: string) {
    trackedStations = trackedStations.filter((t) => t.id !== id);
    saveTracked();
  }

  function handleToggleTrack() {
    if (detailStation) toggleTrack(detailStation);
  }

  function updateDevMode(v: boolean) {
    devMode = v;
  }

  function openDetail(s: Station) {
    detailStation = s;
    overlay = "detail";
  }

  function closeOverlay() {
    overlay = null;
  }

  function onBack() {
    if (overlay) {
      closeOverlay();
      return true;
    }
    return false;
  }

  onMount(() => {
    document.addEventListener("backbutton", onBack);
    return () => document.removeEventListener("backbutton", onBack);
  });
</script>

<div id="app-root">
  <Home
    onOpenDetail={openDetail}
    onOpenMonitor={() => overlay = "monitor"}
    onOpenSettings={() => overlay = "settings"}
    {devMode}
  />

  {#if overlay === "detail" && detailStation}
    <div class="overlay-backdrop" on:click={closeOverlay} role="presentation">
      <div class="overlay-panel" on:click|stopPropagation>
        <StationDetail
          station={detailStation}
          tracked={isTracked(detailStation.id)}
          onToggleTrack={handleToggleTrack}
          onClose={closeOverlay}
          {devMode}
        />
      </div>
    </div>
  {:else if overlay === "monitor"}
    <div class="overlay-backdrop" on:click={closeOverlay} role="presentation">
      <div class="overlay-panel" on:click|stopPropagation>
        <Monitor
          {trackedStations}
          onRemoveTrack={removeTrack}
          onClose={closeOverlay}
        />
      </div>
    </div>
  {:else if overlay === "settings"}
    <div class="overlay-backdrop" on:click={closeOverlay} role="presentation">
      <div class="overlay-panel" on:click|stopPropagation>
        <Settings
          {devMode}
          {updateDevMode}
          onClose={closeOverlay}
        />
      </div>
    </div>
  {/if}
</div>

<style>
  #app-root {
    height: 100%;
    width: 100%;
    position: relative;
    overflow: hidden;
  }

  .overlay-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 100;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    animation: fadeIn 0.2s;
  }

  .overlay-panel {
    width: 100%;
    max-height: 85vh;
    background: var(--bg);
    border-radius: 16px 16px 0 0;
    overflow-y: auto;
    animation: slideUp 0.25s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes slideUp {
    from { transform: translateY(100%); }
    to { transform: translateY(0); }
  }
</style>

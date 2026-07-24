<script lang="ts">
  import type { Page, Station, StationDetailParams } from "./lib/types";
  import Home from "./pages/Home.svelte";
  import StationList from "./pages/StationList.svelte";
  import StationDetail from "./pages/StationDetail.svelte";
  import Monitor from "./pages/Monitor.svelte";

  let page: Page = "home";
  let stations: Station[] = [];
  let trackedIds: string[] = [];
  let detailStation: Station | null = null;

  function navigate(p: Page, data?: Station[] | string[] | StationDetailParams) {
    page = p;
    if (data instanceof Array) {
      if (typeof data[0] === "string") {
        trackedIds = data as string[];
      } else {
        stations = data as Station[];
      }
    } else if (data && "station" in data) {
      detailStation = data.station;
    }
  }
</script>

<div id="app-root">
  {#if page === "home"}
    <Home {navigate} />
  {:else if page === "stations"}
    <StationList {navigate} {stations} />
  {:else if page === "detail" && detailStation}
    <StationDetail {navigate} station={detailStation} />
  {:else if page === "monitor"}
    <Monitor {navigate} {trackedIds} />
  {/if}
</div>

<style>
  #app-root {
    height: 100%;
    width: 100%;
  }
</style>

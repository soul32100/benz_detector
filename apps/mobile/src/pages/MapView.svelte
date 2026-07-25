<script lang="ts">
  import { onMount, afterUpdate } from "svelte";
  import maplibregl from "maplibre-gl";
  import type { Station } from "../lib/types";

  export let stations: Station[] = [];
  export let centerLon = 37.62;
  export let centerLat = 55.75;
  export let zoom = 10;
  export let onStationClick: (s: Station) => void = () => {};
  export let onMapMove: (lat: number, lon: number) => void = () => {};
  export let onUserDrag: (lat: number, lon: number) => void = () => {};
  export let userLat: number | null = null;
  export let userLon: number | null = null;

  let container: HTMLDivElement;
  let map: maplibregl.Map | null = null;
  let loaded = false;

  onMount(() => {
    map = new maplibregl.Map({
      container,
      style: "https://tiles.openfreemap.org/styles/liberty",
      center: [centerLon, centerLat],
      zoom,
      attributionControl: true,
    });

    map.addControl(new maplibregl.NavigationControl(), "bottom-right");

    map.on("load", () => {
      loaded = true;

      map!.addSource("stations", {
        type: "geojson",
        data: { type: "FeatureCollection", features: [] },
        cluster: false,
      });

      map!.addSource("user-location", {
        type: "geojson",
        data: { type: "FeatureCollection", features: [] },
      });

      map!.addLayer({
        id: "station-point",
        type: "circle",
        source: "stations",
        paint: {
          "circle-color": "#2196f3",
          "circle-radius": 6,
          "circle-stroke-width": 1.5,
          "circle-stroke-color": "#fff",
          "circle-opacity": 0.85,
        },
      });

      map!.addLayer({
        id: "user-location-outer",
        type: "circle",
        source: "user-location",
        paint: {
          "circle-radius": 18,
          "circle-color": "#2196f3",
          "circle-opacity": 0.2,
        },
      });

      map!.addLayer({
        id: "user-location-inner",
        type: "circle",
        source: "user-location",
        paint: {
          "circle-radius": 8,
          "circle-color": "#2196f3",
          "circle-stroke-width": 3,
          "circle-stroke-color": "#fff",
        },
      });

      map!.on("click", "station-point", (e) => {
        const feat = e.features?.[0];
        if (!feat) return;
        const sid = feat.properties?.station_id;
        const station = stations.find((s) => s.id === sid);
        if (station) onStationClick(station);
      });

      map!.on("mouseenter", "station-point", () => { map!.getCanvas().style.cursor = "pointer"; });
      map!.on("mouseleave", "station-point", () => { map!.getCanvas().style.cursor = ""; });

      map!.on("moveend", () => {
        const c = map!.getCenter();
        onMapMove(c.lat, c.lng);
      });

      map!.on("dragend", () => {
        const c = map!.getCenter();
        onUserDrag(c.lat, c.lng);
      });

      refreshStations();
      updateUserMarker();
    });

    return () => { map?.remove(); };
  });

  function updateUserMarker() {
    if (!map || !loaded) return;
    const source = map.getSource("user-location") as maplibregl.GeoJSONSource;
    if (!source) return;
    if (userLat !== null && userLon !== null) {
      source.setData({
        type: "FeatureCollection",
        features: [
          {
            type: "Feature",
            geometry: { type: "Point", coordinates: [userLon, userLat] },
            properties: {},
          },
        ],
      });
    }
  }

  function refreshStations() {
    if (!map || !loaded) return;
    const source = map.getSource("stations") as maplibregl.GeoJSONSource;
    if (!source) return;
    const features: GeoJSON.Feature[] = stations
      .filter((s) => s.latitude && s.longitude)
      .map((s) => ({
        type: "Feature",
        geometry: { type: "Point", coordinates: [s.longitude!, s.latitude!] },
        properties: { station_id: s.id, name: s.name, address: s.address },
      }));
    source.setData({ type: "FeatureCollection", features });
  }

  export function flyTo(lat: number, lon: number, z = 12) {
    if (map) map.flyTo({ center: [lon, lat], zoom: z });
  }

  afterUpdate(() => {
    refreshStations();
    updateUserMarker();
  });
</script>

<div bind:this={container} class="map-container"></div>

<style>
  .map-container { width: 100%; height: 100%; }
</style>

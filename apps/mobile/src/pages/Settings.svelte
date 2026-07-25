<script lang="ts">
  import { saveSettings } from "../lib/store";

  export let onClose: () => void;
  export let devMode = false;
  export let updateDevMode: (v: boolean) => void;

  let devModeLocal = devMode;

  async function toggleDev() {
    devModeLocal = !devModeLocal;
    updateDevMode(devModeLocal);
    await saveSettings({ devMode: devModeLocal, monitorInterval: 5 });
  }
</script>

<div class="page">
  <div class="flex-row" style="justify-content: space-between; align-items: center;">
    <h1 class="page-title">⚙️ Настройки</h1>
    <button class="btn btn-sm btn-secondary" on:click={onClose}>
      ← Назад
    </button>
  </div>

  <div class="card">
    <div class="flex-row" style="justify-content: space-between;">
      <div>
        <div class="card-title">🛠 Режим разработчика</div>
        <div class="text-sm text-secondary mt-8">
          Показывать источник данных (какое API предоставило информацию)
        </div>
      </div>
      <button
        class="btn btn-sm {devModeLocal ? 'btn-danger' : 'btn-secondary'}"
        on:click={toggleDev}
      >
        {devModeLocal ? "Вкл" : "Выкл"}
      </button>
    </div>
  </div>

  <div class="card">
    <div class="label">О приложении</div>
    <p class="text-sm text-secondary mt-8">BenzDetector v0.1.0</p>
    <p class="text-sm text-secondary">Источники: benzest.ru, gdebenz.ru</p>
  </div>
</div>

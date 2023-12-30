<script>
  import { invoke } from '@tauri-apps/api/tauri'
  async function run_surreal() {
    await invoke('run_surreal', {})
  }
</script>

<button on:click={run_surreal}>click me</button>

<script>
  import { invoke } from '@tauri-apps/api/tauri';

  let title = '';
  let firstName = '';
  let lastName = '';

  async function run_surreal() {
    await invoke('run_surreal', { title, firstName, lastName })
  }
</script>

<form on:submit={run_surreal}>
  <input type="text" placeholder="first name" bind:value={firstName}>
  <input type="text" placeholder="last name" bind:value={lastName}><br>
  <input type="text" placeholder="title" bind:value={title}>
  <input type="submit">
</form>

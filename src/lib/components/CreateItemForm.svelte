<script>
  export let tabs;
  export let activeTab;
  export let data;
</script>

<form>
  <h3>Create a new {activeTab}</h3>

  {#each Object.entries(data[tabs.indexOf(activeTab)]) as [columnName, columnType]}
    <div class="form-field">
      <label for={columnName}>{columnName}:</label>
      <input type={columnType} id="column" />
    </div>
  {/each}
</form>

<style>
</style>

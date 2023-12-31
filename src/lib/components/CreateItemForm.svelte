<script>
  export let tabs;
  export let activeTab;
  export let data;

  let newField = {};

  const handleSubmit = () => {
    console.log(newField);
  };
</script>

<form on:submit|preventDefault={handleSubmit}>
  <h3>Create a new {activeTab}</h3>

  {#each Object.entries(data[tabs.indexOf(activeTab)]) as [columnName, columnType]}
    <div class="form-field">
      <label for={columnName}>{columnName}:</label>
      {#if columnType === "text"}
        <input type="text" id={columnName} bind:value={newField[columnName]} />
      {:else if columnType === "date"}
        <input type="date" id={columnName} bind:value={newField[columnName]} />
      {:else if columnType === "datetime"}
        <input
          type="datetime-local"
          id={columnName}
          bind:value={newField[columnName]}
        />
      {:else}
        <input type="text" id={columnName} bind:value={newField[columnName]} />
      {/if}
    </div>
  {/each}
</form>

<style>
</style>

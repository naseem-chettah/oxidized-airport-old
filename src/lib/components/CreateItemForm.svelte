<script>
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";

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
        <Input type="text" id={columnName} bind:value={newField[columnName]} />
      {:else if columnType === "date"}
        <Input type="date" id={columnName} bind:value={newField[columnName]} />
      {:else if columnType === "datetime-local"}
        <Input
          type="datetime-local"
          id={columnName}
          bind:value={newField[columnName]}
        />
      {:else}
        <input type="text" id={columnName} bind:value={newField[columnName]} />
      {/if}
    </div>
  {/each}
  <Button>submit</Button>
</form>

<style>
  form {
    max-width: 400px;
    margin: 0 auto;
    text-align: center;
  }

  .form-field {
    margin: 18px auto;
  }

  input {
    width: 100%;
    border-radius: 6px;
  }

  label {
    margin: 10px auto;
    text-align: left;
  }
</style>

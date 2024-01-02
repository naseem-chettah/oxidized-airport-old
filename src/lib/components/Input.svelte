<script>
  export let id;
  export let type = "text";
</script>

<input {id} {type}/>

<style>
  input {
    border: 0;
    border-radius: 6px;
    padding: 8px 12px;
    box-shadow: 1px 2px 3px rgba(0, 0, 0, 0.2);
    outline: none;
  }
</style>

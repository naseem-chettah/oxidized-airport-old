<script>
  export let activeTab;
</script>

<form>
  <h3>Create a new {activeTab}</h3>
  <div class="form-field">
    <label for="first-name">{activeTab} first field:</label>
    <input type="text" id="first-name">
  </div>
  <div class="form-field">
    <label for="last-name">{activeTab} second field:</label>
    <input type="text" id="last-name">
  </div>
  <div class="form-field">
    <label for="date-of-birth">{activeTab} third field:</label>
    <input type="date" id="date-of-birth">
  </div>
</form>

<style>
</style>

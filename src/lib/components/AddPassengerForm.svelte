<script>
  import Button from "./Button.svelte";

  let fields = { first_name: "", last_name: "", date_of_birth: "" };
  let errors = { first_name: "", last_name: "", date_of_birth: "" };
  let valid = false;

  const submitHandle = () => {
    valid = true;

    if (fields.first_name.trim().length < 2) {
      valid = false;
      errors.first_name = "too short";
    } else {
      errors.first_name = "";
    }

    if (fields.last_name.trim().length < 2) {
      valid = false;
      errors.last_name = "too short";
    } else {
      errors.last_name = "";
    }

    if (fields.date_of_birth.trim().length < 10) {
      valid = false;
      errors.date_of_birth = "not valid";
    } else {
      errors.date_of_birth = "";
    }

    if (valid) {
      console.log("valid", fields);
    }
  };
</script>

<form on:submit|preventDefault={submitHandle}>
  <h4>Add a new passenger</h4>
  <div class="form-field">
    <label for="first-name">first name:</label>
    <input type="text" id="first-name" bind:value={fields.first_name} />
    <div class="error">{ errors.first_name }</div>
  </div>
  <div class="form-field">
    <label for="last-name">last name:</label>
    <input type="text" id="last-name" bind:value={fields.last_name} />
    <div class="error">{ errors.last_name }</div>
  </div>
  <div class="form-field">
    <label for="date-of-birth">date of birth:</label>
    <input type="date" id="date-of-birth" bind:value={fields.date_of_birth} />
    <div class="error">{ errors.date_of_birth }</div>
  </div>
  <Button>Submit</Button>
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
    border: 0;
    border-radius: 6px;
    padding: 8px 12px;
    box-shadow: 1px 2px 3px rgba(0, 0, 0, 0.2);
    outline: none;
  }

  label {
    margin: 10px auto;
    text-align: left;
  }

  .error {
    color: red;
  }
</style>

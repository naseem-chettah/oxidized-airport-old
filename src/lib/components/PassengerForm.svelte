<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  import PassengersStore from "../../stores/PassengersStore";

  import Button from "./Button.svelte";

  let fields = {
    firstName: "",
    lastName: "",
    dateOfBirth: "",
    gender: "",
    phoneNumber: "",
    email: "",
    nationality: "",
    passportNumber: "",
  };

  let errors = {
    firstName: "",
    lastName: "",
    dateOfBirth: "",
    gender: "",
    phoneNumber: "",
    email: "",
    nationality: "",
    passportNumber: "",
  };

  let valid = false;

  const submitHandle = () => {
    valid = true;

    if (fields.firstName.trim().length < 2) {
      valid = false;
      errors.firstName = "too short";
    } else {
      errors.firstName = "";
    }

    if (fields.lastName.trim().length < 2) {
      valid = false;
      errors.lastName = "too short";
    } else {
      errors.lastName = "";
    }

    if (fields.dateOfBirth.trim().length < 10) {
      valid = false;
      errors.dateOfBirth = "not valid";
    } else {
      errors.dateOfBirth = "";
    }

    if (valid) {
      let passenger = { ...fields, id: Math.floor(Math.random() * 10) };
      PassengersStore.update((currentPassengers) => {
        return [passenger, ...currentPassengers];
      });
      dispatch("pushPassenger");
    }
  };
</script>

<form on:submit|preventDefault={submitHandle}>
  <h4>Add a new passenger</h4>
  <div class="form-field">
    <label for="first-name">first name:</label>
    <input type="text" id="first-name" bind:value={fields.firstName} />
    <div class="error">{errors.firstName}</div>
  </div>
  <div class="form-field">
    <label for="last-name">last name:</label>
    <input type="text" id="last-name" bind:value={fields.lastName} />
    <div class="error">{errors.lastName}</div>
  </div>
  <div class="form-field">
    <label for="date-of-birth">date of birth:</label>
    <input type="date" id="date-of-birth" bind:value={fields.dateOfBirth} />
    <div class="error">{errors.dateOfBirth}</div>
  </div>
  <div class="form-field">
    <label for="gender">gender:</label>
    <input type="radio" id="gender" value="male" bind:group={fields.gender} /> Male
    <input type="radio" id="gender" value="female" bind:group={fields.gender} /> Female
    <div class="error">{errors.gender}</div>
  </div>
  <Button type={"secondary"}>Submit</Button>
</form>

<style>
  form {
    padding: 20px;
    border-radius: 10px;
    width: 40%;
    margin: 5% auto;
    background: #fff;
    text-align: center;
  }

  .form-field {
    margin: 18px auto;
  }

  input {
    width: 60%;
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
    font-size: 12px;
    color: red;
  }
</style>

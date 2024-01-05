<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  import PassengersStore from "../../../stores/PassengersStore";
  import Nationalities from "../../../stores/Nationalities";

  import Button from "../../shared/Button.svelte";

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
      dispatch("hideForm");
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
    <input type="radio" id="gender" value="male" bind:group={fields.gender} />
    Male
    <input type="radio" id="gender" value="female" bind:group={fields.gender} />
    Female
    <div class="error">{errors.gender}</div>
  </div>
  <div class="form-field">
    <label for="phone-number">phone number:</label>
    <input type="text" id="phone-number" bind:value={fields.phoneNumber} />
    <div class="error">{errors.phoneNumber}</div>
  </div>
  <div class="form-field">
    <label for="email">email:</label>
    <input type="text" id="email" bind:value={fields.email} />
    <div class="error">{errors.email}</div>
  </div>
  <div class="form-field">
    <label for="nationality">Nationality:</label>
    <select id="nationality" bind:value={fields.nationality}>
      {#each $Nationalities as { code, name }}
        <option value={code}>{name}</option>
      {/each}
    </select>
    <div class="error">{errors.nationality}</div>
  </div>
  <div class="form-field">
    <label for="passport-number">passport number:</label>
    <input type="text" id="passport-number" bind:value={fields.passportNumber} />
    <div class="error">{errors.passportNumber}</div>
  </div>

  <Button type={"secondary"}>Submit</Button>
</form>

<style>
  form {
    padding: 25px 40px;
    padding-top: 10px;
    border-radius: 10px;
    width: 30%;
    margin: 5% auto;
    background: #fff;
    text-align: left;
  }

  .form-field {
    margin: 18px auto;
  }

  input[type="text"] {
    width: 60%;
    border: 0;
    border-radius: 6px;
    padding: 8px 12px;
    box-shadow: 1px 2px 3px rgba(0, 0, 0, 0.2);
    outline: none;
  }

  input[type="date"] {
    width: 60%;
    border: 0;
    border-radius: 6px;
    padding: 8px 12px;
    box-shadow: 1px 2px 3px rgba(0, 0, 0, 0.2);
    outline: none;
  }

  select {
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

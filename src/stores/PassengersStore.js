import { writable } from 'svelte/store';

const PassengersStore = writable([
  {
    id: 1,
    firstName: "seth",
    lastName: "ossidian",
    dateOfBirth: "2003-12-31",
    gender: "male",
    phoneNumber: "+213559352980",
    email: "setheris@tuta.io",
    nationality: "DZ",
    passportNumber: "1101",
  },
])

export default PassengersStore;

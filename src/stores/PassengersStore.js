import { writable } from 'svelte/store';

const PassengersStore = writable([
    {
      id: 1,
      first_name: "seth",
      last_name: "ossidian",
      date_of_birth: "2003-12-31",
    },
])

export default PassengersStore;

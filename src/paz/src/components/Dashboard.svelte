<script lang="ts">

import { get } from 'svelte/store'
import { state } from '../store'
import Reminder from './Reminder.svelte'

let s = get(state)
console.log(s)

state.subscribe(val => {
    console.log("state updated", val)
    s = val
});

</script>

<div>
    <header>Dashboard</header>
    {#each s?.reminders ?? [] as reminder}
        <Reminder state={reminder} />
    {/each}
</div>


<style>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>
<script lang="ts">

import { get } from 'svelte/store'
import { state } from '../store'
import Reminder from './Reminder.svelte'

let s = get(state)

state.subscribe(val => {
    s = val
});

</script>

<div>
    <header>Dashboard</header>
    <div class="card-display">
        {#each s?.reminders ?? [] as reminder}
            <Reminder state={reminder} />
        {/each}
    </div>
</div>


<style>
	
	.card-display {
        display: flex;
        flex-direction: column;
    }
	
</style>
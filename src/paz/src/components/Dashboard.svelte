<script lang="ts">

import Accordion, { Panel, Header, Content } from '@smui-extra/accordion';
import { get } from 'svelte/store'
import { state } from '../store'
import Reminder from './Reminder.svelte'

let s = get(state)

state.subscribe(val => {
    s = val
});

</script>

<div>
    <div class="reminders-display">
        <Accordion class="reminders-child">
            {#each s?.reminders ?? [] as reminder}
                <Reminder state={reminder} />
            {/each}    
        </Accordion>
    </div>
</div>


<style>

    .reminders-display :global(.reminders-child) {
        /* justify-content: center; */
        display: flex;
        justify-content: center;
        flex-direction: column;
        margin: 30px 30px 30px 30px;
    }
	
</style>
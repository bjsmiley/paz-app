<script lang="ts">

import Accordion from '@smui-extra/accordion';
import IconButton from '@smui/icon-button';
import { get } from 'svelte/store'
import { state, transport } from '../store'
import Reminder from './Reminder.svelte'
import type { ClientState, CoreResponse, ReminderState } from '@paz/core';
import {v4 as uuidv4} from 'uuid'
import Button, { Label } from '@smui/button';

let s = get(state)

state.subscribe(val => {
    s = val
});

const onAddClick = () => {
    // the add button doesnt really need to be disabled
    // the newReminder can just be appended onto the array of reminders on state object
    // then add delete button
    let newReminder = { id: uuidv4(), name: "New Reminder", wait_sec: 5 * 60, is_active: false}
    s?.reminders.push(newReminder)
    s = s
}

const onCancelClick = async () => {
    let res = (await get(transport).query({key: "ClientGetState"})) as CoreResponse;
    state.set(res.data as ClientState)
}

</script>

<div>
    <div class="reminders-display">
        <Accordion class="reminders-child">
            {#each s?.reminders ?? [] as reminder}
                <Reminder state={reminder} />
            {/each}    
        </Accordion>
        <!-- <IconButton style="margin-top: 5px;" class="material-icons" on:click={onAddClick} ripple={false} size="button">
            add
        </IconButton> -->
        <div style="margin-top: 5px;">
            <Button on:click={onAddClick}>
                <Label>New</Label>
            </Button>
            <Button on:click={() => {}}>
                <Label>Save</Label>
            </Button>
            <Button on:click={onCancelClick}>
                <Label>Cancel</Label>
            </Button>
        </div>
    
    </div>
</div>


<style>

    .reminders-display {
        margin: 30px 30px 30px 30px;
    }

    .reminders-display :global(.reminders-child) {
        /* justify-content: center; */
        display: flex;
        justify-content: center;
        flex-direction: column;
        /* margin: 30px 30px 30px 30px; */
    }
	
</style>
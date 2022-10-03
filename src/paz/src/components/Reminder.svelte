<script lang="ts">
import type { ReminderState } from "@paz/core";
import Switch from '@smui/switch';
import FormField from '@smui/form-field';
import type { MenuComponentDev } from '@smui/menu';
import { Panel, Header, Content } from '@smui-extra/accordion';
import Textfield from '@smui/textfield';
import LayoutGrid, { Cell } from '@smui/layout-grid';
import Icon from '@smui/textfield/icon';
import HelperText from '@smui/textfield/helper-text';
import IconButton from "@smui/icon-button";
import { get } from 'svelte/store'
import { reminderStatusEvent, state as clientState } from '../store'
import { tweened } from 'svelte/motion';



export let state: ReminderState;
export let open= false;

let edit = false;
let checked = false;
let menu: MenuComponentDev;
let timer = null
const hourFrame = 3600
const minuteFrame = 60

const getHour = (x: number) => Math.floor(x / hourFrame)
const getMin = (x: number) => Math.floor((x - (getHour(x) * hourFrame)) / minuteFrame)
const resetTimer = (newSec: number) => {
  timerSec = newSec;
  if(timer) {
      clearInterval(timer);
      timer = null;
  }
  timer = setInterval(() => {
    if(timerSec > 0) timerSec--;
    console.log(timerSec)
  }, 1000);
  console.log("update!")
}

let hourPart = getHour(state.wait_ms);
let minPart = getMin(state.wait_ms);
let timerSec = state.wait_ms / 1000; // https://svelte.dev/repl/86690cfb378e4f3f98cd1a67197a3e42?version=3.25.1

if(state.is_active) {
  resetTimer(timerSec)
}

reminderStatusEvent.subscribe(event => {
  let status = event.ReminderNewStatus;
  if(status.id == state.id) {
    resetTimer(status.next_duration_ms / 1000);
  }
})

$: {
  state.wait_ms = ( hourPart * hourFrame ) + ( minPart * minuteFrame )
}

const onDelete = () => {
  let s = get(clientState);
  s.reminders = s.reminders.filter(r => r.id != state.id);
  clientState.set(s)
}

</script>

<Panel bind:open>
  <Header>
    {state.name}
    <span slot="description">{timerSec}</span>
  </Header>
  <Content>
    <LayoutGrid>
      <Cell>
        <Textfield bind:value={state.name} label="Name"/>
      </Cell>
      <Cell>
        <FormField>
          <Switch bind:checked={state.is_active} />
          <span slot="label">
            {#if state.is_active}
              Active
            {:else}
              Paused
            {/if}
          </span>
        </FormField>
      </Cell>
      <Cell span={8}>
        <Textfield bind:value={hourPart} label="HH" type="number" />
        <Textfield bind:value={minPart} label="mm" type="number" />
      </Cell>
      <Cell span={12}>
        <IconButton style="margin-top: 5px;" class="material-icons" ripple={false} size="button" on:click={onDelete}>
          delete
        </IconButton>
      </Cell>
    </LayoutGrid>

    <!-- <div class="panel-content">
      <Textfield bind:value={state.name} label="Name"/>
      <FormField>
        <Switch bind:checked={state.is_active} />
        <span slot="label">
          {#if state.is_active}
            Active
          {:else}
            Paused
          {/if}
        </span>
      </FormField>
    </div> -->
  </Content>
</Panel>

<style>
    /* .card-container {
        margin: 10px auto;
        min-width: 600px;
    }
    
    .c-tail {
        display: flex;
        justify-content: flex-end;
    } */
	
	
</style>
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
import { state as clientState } from '../store'



  
let edit = false;
let checked = false;
let menu: MenuComponentDev;


export let state: ReminderState;
export let open= false;

let hourSelect = Math.floor(state.wait_sec / 3600)
let minuteValue = Math.floor((state.wait_sec - (hourSelect * 3600)) / 60)

$: {
  state.wait_sec = ( hourSelect * 3600 ) + ( minuteValue * 60 )
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
    <span slot="description">{state.id}</span>
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
        <Textfield bind:value={hourSelect} label="HH" type="number" />
        <Textfield bind:value={minuteValue} label="mm" type="number" />
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
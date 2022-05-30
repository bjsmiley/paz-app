<script lang="ts">
import type { ReminderState } from "@paz/core";
import Card, {
      Content,
      PrimaryAction,
      Actions,
      ActionButtons,
      ActionIcons,
    } from '@smui/card';
import Button, { Label } from '@smui/button';
import IconButton, { Icon } from '@smui/icon-button';
import Switch from '@smui/switch';
import FormField from '@smui/form-field';
import type { MenuComponentDev } from '@smui/menu';
import Menu from '@smui/menu';
import List, { Item, Separator, Text, Graphic } from '@smui/list';

import Navigation from "./Navigation.svelte";
  
let edit = false;
let checked = false;
let menu: MenuComponentDev;



export let state: ReminderState;

</script>
    <div class="card-container">
        <Card>
            <Content>
                <div class="c-content">
                    <p>{state.name}</p>
                </div>
            </Content>

            <div class="c-tail">
                <!-- <FormField align="end"> -->
                    <Switch  bind:checked={checked} />
                    <!-- </FormField> -->
              <IconButton class="material-icons" title="Options" size="mini" on:click={() => menu.setOpen(true)}>more_vert</IconButton>
              <Menu bind:this={menu}>
                <List>
                  <Item on:SMUI:action={() => (edit = !edit)}>
                    <Graphic class="material-icons">edit</Graphic>
                    <Text>Edit</Text>
                  </Item>
                  <!-- <Item on:SMUI:action={() => (clicked = 'Copy')}>
                    <Text>Copy</Text>
                  </Item>
                  <Item on:SMUI:action={() => (clicked = 'Paste')}>
                    <Text>Paste</Text>
                  </Item> -->
                  <!-- <Separator /> -->
                  <Item>
                    <Graphic class="material-icons">clear</Graphic>
                    <Text>Delete</Text>
                  </Item>
                </List>
              </Menu>
            </div>
            <!-- <Content> -->
                
                <!-- <Button variant="raised">
                    <Label>{state.name}</Label>
                </Button> -->
            <!-- </Content> -->

        </Card>
        {#if edit}
          <Card>
            <Content>A card with action buttons.</Content>
            <Actions>
              <Button on:click={() => {edit = !edit}}>
                <Label>Canel</Label>
              </Button>
              <Button on:click={() => {}}>
                <Label>Save</Label>
              </Button>
            </Actions>
          </Card>
        {/if}
    </div>


<!-- 
<div class="card-display">
    <div class="card-container">
      <Card>
        <PrimaryAction on:click={() => clicked++} padded>
          Primary Action, a clickable area of the card.
        </PrimaryAction>
      </Card>
    </div>
  
    <div class="card-container">
      <Card>
        <Content>A card with action buttons.</Content>
        <Actions>
          <Button on:click={() => clicked++}>
            <Label>Action</Label>
          </Button>
          <Button on:click={() => clicked++}>
            <Label>Another</Label>
          </Button>
        </Actions>
      </Card>
    </div>
  
    <div class="card-container">
      <Card>
        <Content>A card with a full-bleed action.</Content>
        <Actions fullBleed>
          <Button on:click={() => clicked++}>
            <Label>Action</Label>
            <i class="material-icons" aria-hidden="true">arrow_forward</i>
          </Button>
        </Actions>
      </Card>
    </div>
  
    <div class="card-container">
      <Card>
        <Content>A card with action icons.</Content>
        <Actions>
          <IconButton
            on:click={() => clicked++}
            toggle
            aria-label="Add to favorites"
            title="Add to favorites"
          >
            <Icon class="material-icons" on>favorite</Icon>
            <Icon class="material-icons">favorite_border</Icon>
          </IconButton>
          <IconButton
            class="material-icons"
            on:click={() => clicked++}
            title="Share">share</IconButton
          >
          <IconButton
            class="material-icons"
            on:click={() => clicked++}
            title="More options">more_vert</IconButton
          >
        </Actions>
      </Card>
    </div>
  
    <div class="card-container">
      <Card>
        <Content>A card with Both.</Content>
        <Actions>
          <ActionButtons>
            <Button on:click={() => clicked++}>
              <Label>Action</Label>
            </Button>
            <Button on:click={() => clicked++}>
              <Label>Another</Label>
            </Button>
          </ActionButtons>
          <ActionIcons>
            <IconButton
              on:click={() => clicked++}
              toggle
              aria-label="Add to favorites"
              title="Add to favorites"
            >
              <Icon class="material-icons" on>favorite</Icon>
              <Icon class="material-icons">favorite_border</Icon>
            </IconButton>
            <IconButton
              class="material-icons"
              on:click={() => clicked++}
              title="Share">share</IconButton
            >
            <IconButton
              class="material-icons"
              on:click={() => clicked++}
              title="More options">more_vert</IconButton
            >
          </ActionIcons>
        </Actions>
      </Card>
    </div>
  </div>
  
  <pre class="status">Clicked: {clicked}</pre>
   -->



<style>
    .card-container {
        margin: 10px auto;
        min-width: 600px;
    }
    
    .c-tail {
        display: flex;
        justify-content: flex-end;
    }
	
	
</style>
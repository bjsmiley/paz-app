<script lang="ts">
import type { View } from "@paz/core";
import Tab, { Label } from '@smui/tab';
import TabBar from '@smui/tab-bar';

export let display: View = "Dashboard"

let choices: View[] = ["Dashboard", "Settings"];

</script>


<div>
  <!--
    Note: tabs must be unique. (They cannot === each other.)
  -->
  <TabBar tabs={choices} let:tab bind:active={display}>
    <!-- Note: the `tab` property is required! -->
    <Tab {tab}>
      <Label>{tab}</Label>
    </Tab>
  </TabBar>


</div>

<style>
</style>
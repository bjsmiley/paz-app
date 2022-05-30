<script lang="ts">
	
import Dashboard from "./components/Dashboard.svelte";
import Settings from "./components/Settings.svelte";
import TopNav from "./components/TopNav.svelte";
import type { View } from "@paz/core";
import { state, transport } from './store'
import { get } from 'svelte/store'
import type { ClientState, CoreResponse } from '@paz/core';

let display: View = "Dashboard"

get(transport)
 .query({key: "ClientGetState"})
 .then(res => {
	state.set((res as CoreResponse).data as ClientState)
})



</script>

<main>
	<TopNav bind:display/>
	{#if display == "Dashboard"}
		<Dashboard/>
	{:else}
		<Settings/>
	{/if}
</main>

<style>
	:global(body) {
		padding: 0;
	}
</style>
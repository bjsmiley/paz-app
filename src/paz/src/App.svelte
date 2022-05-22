<script lang="ts">
	
import Dashboard from "./components/Dashboard.svelte";
import Settings from "./components/Settings.svelte";
import TopNav from "./components/TopNav.svelte";
import type { View } from "@paz/core";
import { state, transport } from './store'
import { get } from 'svelte/store'
import type { ClientState, CoreResponse } from '@paz/core';

let display: View = "Dashboard"

console.log('starting App.svelte!')
console.log('Starting up!')

let t = get(transport);
t.query({key: "ClientGetState"}).then(res => {
	console.log(res)
	state.set((res as CoreResponse).data as ClientState)
})


console.log('I set the state!')

</script>

<main>
	<p>Hi</p>
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
<script lang="ts">
import Dashboard from "./components/Dashboard.svelte";
import Settings from "./components/Settings.svelte";
import TopNav from "./components/TopNav.svelte";
import type { View } from "@paz/core";

let display: View = "Dashboard"



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
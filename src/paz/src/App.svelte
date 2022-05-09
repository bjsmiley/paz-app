<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { ClientCommand, ClientQuery, ClientState, CoreResponse } from '../../core';

	let state: ClientState;
	let num = 0;

	let stateQuery: ClientQuery = { key: "ClientGetState" }
	invoke("client_query", { data: stateQuery })
		.then(st => {
			state = (st as CoreResponse).data as ClientState
		})

	function increment() {
		let cmd: ClientCommand = { key: "AddOne", params: { value: num } }
		invoke("client_command", { data: cmd })
		.then(v => {
			num = (v as CoreResponse).data as number
		})
	}

	function increment2() {
		let cmd: ClientCommand = { key: "Add", params: { x: num, y: 2 } }
		invoke("client_command", { data: cmd })
		.then(v => {
			num = (v as CoreResponse).data as number
		})
	}

</script>

<main>
	<h1>Hello {state?.first_name ?? "???"}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	<button on:click={increment}>
		Clicked {num} time(s)
	</button>
	<button on:click={increment2}>
		Clicked {num} time(s) (+2)
	</button>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
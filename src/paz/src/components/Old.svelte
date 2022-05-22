<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { ClientCommand, ClientQuery, ClientState, CoreResponse } from '@paz/core';

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

<nav class="bg-blue-900 shadow-lg">
    <div class="container mx-auto">
        <div class="sm:flex">
        <a href="#" class="text-white text-3xl font-bold p-3">APP LOGO</a>
        
        <!-- Menus -->
        <div class="ml-55 mt-4">
            <ul class="text-white sm:self-center text-xl">
            <li class="sm:inline-block">
                <a href="#" class="p-3 hover:text-red-900">About</a>
            </li>
            <li class="sm:inline-block">
                <a href="#" class="p-3 hover:text-red-900">Services</a>
            </li>
            <li class="sm:inline-block">
                <a href="#" class="p-3 hover:text-red-900">Blog</a>
            </li>
            <li class="sm:inline-block">
                <a href="#" class="p-3 hover:text-red-900">Contact</a>
            </li>
            </ul>
        </div>
    
        </div>
    </div>
    </nav>
<h1>Hello!</h1>
<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
<button on:click={increment}>
    Clicked {num} time(s)
</button>
<button on:click={increment2}>
    Clicked {num} time(s) (+2)
</button>

<style>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	/* main {
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
	} */
</style>
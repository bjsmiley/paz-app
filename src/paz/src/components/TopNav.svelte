<script lang="ts">
import type { View } from "@paz/core";

export let display: View = "Dashboard"

</script>

<div class="antialiased flex justify-center m-4">
    <div class="flex mb-2 border-t border-r border-l text-sm rounded overflow-hidden select-none">
      <a on:click={() => display = "Dashboard"} href="#" class="px-4 py-2 border-b-2 border-sky-500 bg-sky-100">Dashboard</a>
      <a on:click={() => display = "Settings" } href="#" class="px-4 py-2 border-b-2 hover:border-sky-500 hover:bg-sky-100">Settings</a>
    </div>
</div>



<style>
  
	
	


</style>
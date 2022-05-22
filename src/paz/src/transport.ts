import type { ClientCommand, ClientQuery } from "@paz/core";
import { invoke } from "@tauri-apps/api";

export class Transport {

    constructor(){

    }

    async query(query: ClientQuery): Promise<unknown> {
        return await invoke('client_query', { data: query });
    }
    
    async command(cmd: ClientCommand): Promise<unknown> {
        return await invoke('client_command', { data: cmd });
    }
}